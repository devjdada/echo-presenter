use std::num::NonZeroUsize;
use std::sync::Arc;

use base64::Engine as _;
use echo_core::{
    read_framed, write_framed, MonitorInfo, OutputTargetMode, RendererCommand, RendererEvent,
    SceneGraph, RENDERER_PIPE_NAME,
};
use echo_graphics::vello::peniko::Color;
use echo_graphics::vello::{AaConfig, RenderParams, Renderer as VelloRenderer, RendererOptions};
use echo_graphics::wgpu;
use echo_graphics::{build_vello_scene, ImageCache, TextRenderer};
use tokio::net::windows::named_pipe::ServerOptions;
use tokio::sync::mpsc;
use tracing::{error, info, warn};
use winit::application::ApplicationHandler;
use winit::event::WindowEvent;
use winit::event_loop::{ActiveEventLoop, EventLoop, EventLoopProxy};
use winit::window::{Fullscreen, Window, WindowAttributes, WindowId};

#[derive(Debug)]
enum UserEvent {
    Command(RendererCommand),
}

struct GpuState {
    device: wgpu::Device,
    queue: wgpu::Queue,
    adapter_name: String,
    backend: String,
    vello: VelloRenderer,
    preview_texture: wgpu::Texture,
    preview_view: wgpu::TextureView,
    preview_staging: wgpu::Buffer,
}

struct OutputWindow {
    window: Arc<Window>,
    surface: wgpu::Surface<'static>,
    config: wgpu::SurfaceConfiguration,
}

struct RendererApp {
    proxy: EventLoopProxy<UserEvent>,
    event_tx: mpsc::UnboundedSender<RendererEvent>,
    instance: wgpu::Instance,
    gpu: Option<GpuState>,
    output: Option<OutputWindow>,
    text_renderer: TextRenderer,
    image_cache: ImageCache,
    last_scene: Option<(SceneGraph, String, Option<String>, Option<String>)>,
}

impl RendererApp {
    fn new(
        proxy: EventLoopProxy<UserEvent>,
        event_tx: mpsc::UnboundedSender<RendererEvent>,
    ) -> Self {
        let mut desc = wgpu::InstanceDescriptor::new_without_display_handle();
        desc.backends = wgpu::Backends::DX12 | wgpu::Backends::VULKAN;
        let instance = wgpu::Instance::new(desc);
        Self {
            proxy,
            event_tx,
            instance,
            gpu: None,
            output: None,
            text_renderer: TextRenderer::new(),
            image_cache: ImageCache::default(),
            last_scene: None,
        }
    }

    fn ensure_gpu(&mut self, compatible_surface: Option<&wgpu::Surface<'_>>) {
        if self.gpu.is_some() {
            return;
        }
        let adapter = tokio::runtime::Handle::try_current()
            .ok()
            .and(None::<wgpu::Adapter>)
            .unwrap_or_else(|| {
                futures_lite_block_on(self.instance.request_adapter(&wgpu::RequestAdapterOptions {
                    power_preference: wgpu::PowerPreference::HighPerformance,
                    compatible_surface,
                    force_fallback_adapter: false,
                    apply_limit_buckets: false,
                }))
                .expect("Failed to find HighPerformance wgpu adapter")
            });

        let info = adapter.get_info();
        let adapter_name = info.name.clone();
        let backend = format!("{:?}", info.backend);
        info!(adapter = %adapter_name, backend = %backend, "Selected wgpu GPU adapter");

        let (device, queue) = futures_lite_block_on(adapter.request_device(
            &wgpu::DeviceDescriptor {
                label: Some("echo-renderer-device"),
                required_features: wgpu::Features::empty(),
                required_limits: wgpu::Limits::default(),
                experimental_features: wgpu::ExperimentalFeatures::disabled(),
                memory_hints: wgpu::MemoryHints::Performance,
                trace: wgpu::Trace::Off,
            },
        ))
        .expect("Failed to create wgpu device");

        let vello = VelloRenderer::new(
            &device,
            RendererOptions {
                use_cpu: false,
                antialiasing_support: echo_graphics::vello::AaSupport::area_only(),
                num_init_threads: NonZeroUsize::new(1),
                pipeline_cache: None,
            },
        )
        .expect("Failed to initialize Vello renderer");

        let preview_texture = device.create_texture(&wgpu::TextureDescriptor {
            label: Some("preview-offscreen-640x360"),
            size: wgpu::Extent3d {
                width: 640,
                height: 360,
                depth_or_array_layers: 1,
            },
            mip_level_count: 1,
            sample_count: 1,
            dimension: wgpu::TextureDimension::D2,
            format: wgpu::TextureFormat::Rgba8Unorm,
            usage: wgpu::TextureUsages::STORAGE_BINDING
                | wgpu::TextureUsages::COPY_SRC
                | wgpu::TextureUsages::RENDER_ATTACHMENT,
            view_formats: &[],
        });
        let preview_view = preview_texture.create_view(&wgpu::TextureViewDescriptor::default());

        // 640 * 4 = 2560 bytes per row (already 256-byte aligned!)
        let preview_staging = device.create_buffer(&wgpu::BufferDescriptor {
            label: Some("preview-staging"),
            size: (640 * 360 * 4) as u64,
            usage: wgpu::BufferUsages::COPY_DST | wgpu::BufferUsages::MAP_READ,
            mapped_at_creation: false,
        });

        self.gpu = Some(GpuState {
            device,
            queue,
            adapter_name,
            backend,
            vello,
            preview_texture,
            preview_view,
            preview_staging,
        });
    }

    fn open_or_update_window(&mut self, event_loop: &ActiveEventLoop, target: OutputTargetMode) {
        let mut attrs = WindowAttributes::default()
            .with_title("EchoPresenter — Audience Output")
            .with_transparent(true);

        match &target {
            OutputTargetMode::WindowedTest { width, height } => {
                attrs = attrs
                    .with_inner_size(winit::dpi::LogicalSize::new(*width, *height))
                    .with_decorations(true)
                    .with_resizable(true);
            }
            OutputTargetMode::BorderlessFullscreen { monitor_id } => {
                let monitor = event_loop
                    .available_monitors()
                    .enumerate()
                    .find(|(idx, _)| format!("mon-{idx}") == *monitor_id)
                    .map(|(_, m)| m)
                    .or_else(|| event_loop.primary_monitor());
                attrs = attrs
                    .with_decorations(false)
                    .with_fullscreen(Some(Fullscreen::Borderless(monitor)));
            }
        }

        let window = Arc::new(
            event_loop
                .create_window(attrs)
                .expect("Failed to create output window"),
        );
        let surface = self
            .instance
            .create_surface(window.clone())
            .expect("Failed to create wgpu surface");

        self.ensure_gpu(Some(&surface));
        let gpu = self.gpu.as_ref().unwrap();
        let size = window.inner_size();
        let config = wgpu::SurfaceConfiguration {
            usage: wgpu::TextureUsages::RENDER_ATTACHMENT | wgpu::TextureUsages::STORAGE_BINDING,
            format: wgpu::TextureFormat::Rgba8Unorm,
            width: size.width.max(1),
            height: size.height.max(1),
            present_mode: wgpu::PresentMode::Fifo,
            alpha_mode: wgpu::CompositeAlphaMode::Auto,
            view_formats: vec![],
            desired_maximum_frame_latency: 2,
            color_space: Default::default(),
        };
        surface.configure(&gpu.device, &config);

        self.output = Some(OutputWindow {
            window,
            surface,
            config,
        });

        if let Some((scene, look_id, next_text, stage_msg)) = self.last_scene.clone() {
            self.render_and_emit_preview(&scene, &look_id, next_text.as_deref(), stage_msg.as_deref());
        }
    }

    fn render_and_emit_preview(
        &mut self,
        graph: &SceneGraph,
        look_id: &str,
        next_slide_text: Option<&str>,
        stage_message: Option<&str>,
    ) {
        self.ensure_gpu(None);
        let vello_scene = build_vello_scene(
            graph,
            look_id,
            next_slide_text,
            stage_message,
            &mut self.text_renderer,
            &mut self.image_cache,
        );

        let gpu = self.gpu.as_mut().unwrap();
        let base_color = if look_id == "ndi-lower-third" {
            Color::TRANSPARENT
        } else {
            Color::BLACK
        };

        // 1. Render to live audience window if open
        if let Some(out) = &self.output
            && let wgpu::CurrentSurfaceTexture::Success(frame) = out.surface.get_current_texture()
        {
            let view = frame
                .texture
                .create_view(&wgpu::TextureViewDescriptor::default());
            let _ = gpu.vello.render_to_texture(
                &gpu.device,
                &gpu.queue,
                &vello_scene,
                &view,
                &RenderParams {
                    base_color,
                    width: out.config.width,
                    height: out.config.height,
                    antialiasing_method: AaConfig::Area,
                },
            );
            gpu.queue.present(frame);
        }

        // 2. Render downscaled 640x360 offscreen preview and send PNG over IPC
        let preview_params = RenderParams {
            base_color,
            width: 640,
            height: 360,
            antialiasing_method: AaConfig::Area,
        };
        let mut preview_scene = echo_graphics::vello::Scene::new();
        preview_scene.append(
            &vello_scene,
            Some(echo_graphics::kurbo::Affine::scale_non_uniform(
                640.0 / graph.width.max(1) as f64,
                360.0 / graph.height.max(1) as f64,
            )),
        );

        if gpu
            .vello
            .render_to_texture(
                &gpu.device,
                &gpu.queue,
                &preview_scene,
                &gpu.preview_view,
                &preview_params,
            )
            .is_ok()
        {
            let mut encoder = gpu
                .device
                .create_command_encoder(&wgpu::CommandEncoderDescriptor {
                    label: Some("preview-copy"),
                });
            encoder.copy_texture_to_buffer(
                wgpu::TexelCopyTextureInfo {
                    texture: &gpu.preview_texture,
                    mip_level: 0,
                    origin: wgpu::Origin3d::ZERO,
                    aspect: wgpu::TextureAspect::All,
                },
                wgpu::TexelCopyBufferInfo {
                    buffer: &gpu.preview_staging,
                    layout: wgpu::TexelCopyBufferLayout {
                        offset: 0,
                        bytes_per_row: Some(640 * 4),
                        rows_per_image: Some(360),
                    },
                },
                wgpu::Extent3d {
                    width: 640,
                    height: 360,
                    depth_or_array_layers: 1,
                },
            );
            gpu.queue.submit(Some(encoder.finish()));

            let slice = gpu.preview_staging.slice(..);
            let (tx, rx) = std::sync::mpsc::channel();
            slice.map_async(wgpu::MapMode::Read, move |res| {
                let _ = tx.send(res);
            });
            let _ = gpu.device.poll(wgpu::PollType::wait_indefinitely());
            if let Ok(Ok(())) = rx.recv() {
                if let Ok(data) = slice.get_mapped_range() {
                    if let Some(img) = image::RgbaImage::from_raw(640, 360, data.to_vec()) {
                        let mut png_bytes = Vec::new();
                        if img
                            .write_to(&mut std::io::Cursor::new(&mut png_bytes), image::ImageFormat::Png)
                            .is_ok()
                        {
                            let b64 = base64::engine::general_purpose::STANDARD.encode(&png_bytes);
                            let _ = self.event_tx.send(RendererEvent::PreviewFrame {
                                output_id: "output-1".into(),
                                width: 640,
                                height: 360,
                                png_base64: b64,
                            });
                        }
                    }
                    drop(data);
                }
                gpu.preview_staging.unmap();
            }
        }
    }
}

impl ApplicationHandler<UserEvent> for RendererApp {
    fn resumed(&mut self, _event_loop: &ActiveEventLoop) {
        self.ensure_gpu(None);
    }

    fn user_event(&mut self, event_loop: &ActiveEventLoop, event: UserEvent) {
        let UserEvent::Command(cmd) = event;
        match cmd {
            RendererCommand::Ping => {
                self.ensure_gpu(None);
                let gpu = self.gpu.as_ref().unwrap();
                let _ = self.event_tx.send(RendererEvent::Pong {
                    adapter_name: gpu.adapter_name.clone(),
                    backend: gpu.backend.clone(),
                });
            }
            RendererCommand::ListMonitors => {
                let primary = event_loop.primary_monitor();
                let monitors = event_loop
                    .available_monitors()
                    .enumerate()
                    .map(|(idx, m)| {
                        let size = m.size();
                        let is_primary = primary
                            .as_ref()
                            .map(|p| p.name() == m.name())
                            .unwrap_or(idx == 0);
                        MonitorInfo {
                            id: format!("mon-{idx}"),
                            name: m.name().unwrap_or_else(|| format!("Display {}", idx + 1)),
                            width: size.width,
                            height: size.height,
                            is_primary,
                        }
                    })
                    .collect();
                let _ = self.event_tx.send(RendererEvent::MonitorsList { monitors });
            }
            RendererCommand::SetOutputTarget { target, .. } => {
                self.open_or_update_window(event_loop, target);
            }
            RendererCommand::RenderScene {
                scene,
                look_id,
                next_slide_text,
                stage_message,
                ..
            } => {
                self.last_scene = Some((
                    scene.clone(),
                    look_id.clone(),
                    next_slide_text.clone(),
                    stage_message.clone(),
                ));
                self.render_and_emit_preview(
                    &scene,
                    &look_id,
                    next_slide_text.as_deref(),
                    stage_message.as_deref(),
                );
            }
            RendererCommand::ClearLayer { .. } => {
                let clear_scene = SceneGraph {
                    scene_id: uuid::Uuid::new_v4(),
                    width: 1920,
                    height: 1080,
                    background: echo_core::BackgroundLayer::Transparent,
                    layers: vec![],
                };
                self.last_scene = None;
                self.render_and_emit_preview(&clear_scene, "main-fullscreen", None, None);
            }
            RendererCommand::SetOutputLook { look_id } => {
                if let Some((scene, _, next_text, stage_msg)) = self.last_scene.clone() {
                    self.last_scene =
                        Some((scene.clone(), look_id.clone(), next_text.clone(), stage_msg.clone()));
                    self.render_and_emit_preview(
                        &scene,
                        &look_id,
                        next_text.as_deref(),
                        stage_msg.as_deref(),
                    );
                }
            }
            RendererCommand::SetTransition { kind, duration_ms } => {
                info!(kind = %kind, duration_ms, "Updated transition");
            }
            RendererCommand::UpdateVisualLayer {
                opacity,
                blend_mode,
                audio_reactive,
            } => {
                info!(opacity, blend_mode = %blend_mode, audio_reactive, "Updated visual layer");
            }
            RendererCommand::Shutdown => {
                info!("echo-renderer received Shutdown");
                event_loop.exit();
            }
        }
    }

    fn window_event(
        &mut self,
        _event_loop: &ActiveEventLoop,
        _window_id: WindowId,
        event: WindowEvent,
    ) {
        match event {
            WindowEvent::CloseRequested => {
                self.output = None;
            }
            WindowEvent::Resized(new_size) => {
                if let (Some(out), Some(gpu)) = (&mut self.output, &self.gpu)
                    && new_size.width > 0
                    && new_size.height > 0
                {
                    out.config.width = new_size.width;
                    out.config.height = new_size.height;
                    out.surface.configure(&gpu.device, &out.config);
                }
                if let Some((scene, look_id, next_text, stage_msg)) = self.last_scene.clone() {
                    self.render_and_emit_preview(
                        &scene,
                        &look_id,
                        next_text.as_deref(),
                        stage_msg.as_deref(),
                    );
                }
            }
            _ => {}
        }
    }
}

fn futures_lite_block_on<F: std::future::Future>(fut: F) -> F::Output {
    let rt = tokio::runtime::Builder::new_current_thread()
        .enable_all()
        .build()
        .unwrap();
    rt.block_on(fut)
}

fn spawn_pipe_server(
    proxy: EventLoopProxy<UserEvent>,
    mut event_rx: mpsc::UnboundedReceiver<RendererEvent>,
) {
    std::thread::spawn(move || {
        let rt = tokio::runtime::Builder::new_multi_thread()
            .enable_all()
            .build()
            .expect("Failed to build tokio runtime");

        rt.block_on(async move {
            loop {
                let server = match ServerOptions::new()
                    .first_pipe_instance(true)
                    .create(RENDERER_PIPE_NAME)
                {
                    Ok(s) => s,
                    Err(e) => {
                        error!("Failed to create Named Pipe {RENDERER_PIPE_NAME}: {e}");
                        tokio::time::sleep(std::time::Duration::from_millis(500)).await;
                        continue;
                    }
                };

                info!("Listening on Named Pipe {}", RENDERER_PIPE_NAME);
                if let Err(e) = server.connect().await {
                    warn!("Pipe connect error: {e}");
                    continue;
                }
                info!("Client connected to echo-renderer Named Pipe");

                let (mut reader, mut writer) = tokio::io::split(server);
                let proxy_clone = proxy.clone();

                let read_task = tokio::spawn(async move {
                    while let Ok(cmd) = read_framed::<_, RendererCommand>(&mut reader).await {
                        let is_shutdown = matches!(cmd, RendererCommand::Shutdown);
                        let _ = proxy_clone.send_event(UserEvent::Command(cmd));
                        if is_shutdown {
                            break;
                        }
                    }
                });

                loop {
                    tokio::select! {
                        Some(ev) = event_rx.recv() => {
                            if write_framed(&mut writer, &ev).await.is_err() {
                                break;
                            }
                        }
                        _ = tokio::time::sleep(std::time::Duration::from_millis(50)) => {
                            if read_task.is_finished() {
                                break;
                            }
                        }
                    }
                }
            }
        });
    });
}

fn run_smoke_test() {
    let mut desc = wgpu::InstanceDescriptor::new_without_display_handle();
    desc.backends = wgpu::Backends::DX12 | wgpu::Backends::VULKAN;
    let instance = wgpu::Instance::new(desc);

    let adapter = futures_lite_block_on(instance.request_adapter(&wgpu::RequestAdapterOptions {
        power_preference: wgpu::PowerPreference::HighPerformance,
        compatible_surface: None,
        force_fallback_adapter: false,
        apply_limit_buckets: false,
    }))
    .expect("Failed to find HighPerformance wgpu adapter");

    let info = adapter.get_info();
    println!("SMOKE_TEST_GPU_ADAPTER: {} ({:?})", info.name, info.backend);
    info!(adapter = %info.name, backend = ?info.backend, "Smoke test selected wgpu adapter");

    let (device, queue) = futures_lite_block_on(adapter.request_device(&wgpu::DeviceDescriptor {
        label: Some("smoke-test-device"),
        required_features: wgpu::Features::empty(),
        required_limits: wgpu::Limits::default(),
        experimental_features: wgpu::ExperimentalFeatures::disabled(),
        memory_hints: wgpu::MemoryHints::Performance,
        trace: wgpu::Trace::Off,
    }))
    .expect("Failed to create wgpu device");

    let mut vello = VelloRenderer::new(
        &device,
        RendererOptions {
            use_cpu: false,
            antialiasing_support: echo_graphics::vello::AaSupport::area_only(),
            num_init_threads: NonZeroUsize::new(1),
            pipeline_cache: None,
        },
    )
    .expect("Failed to initialize Vello renderer");

    let tex = device.create_texture(&wgpu::TextureDescriptor {
        label: Some("smoke-offscreen-1280x720"),
        size: wgpu::Extent3d {
            width: 1280,
            height: 720,
            depth_or_array_layers: 1,
        },
        mip_level_count: 1,
        sample_count: 1,
        dimension: wgpu::TextureDimension::D2,
        format: wgpu::TextureFormat::Rgba8Unorm,
        usage: wgpu::TextureUsages::STORAGE_BINDING
            | wgpu::TextureUsages::COPY_SRC
            | wgpu::TextureUsages::RENDER_ATTACHMENT,
        view_formats: &[],
    });
    let view = tex.create_view(&wgpu::TextureViewDescriptor::default());
    let staging = device.create_buffer(&wgpu::BufferDescriptor {
        label: Some("smoke-staging"),
        size: (1280 * 720 * 4) as u64,
        usage: wgpu::BufferUsages::COPY_DST | wgpu::BufferUsages::MAP_READ,
        mapped_at_creation: false,
    });

    let mut text_renderer = TextRenderer::new();
    let mut image_cache = ImageCache::default();
    let graph = SceneGraph {
        scene_id: uuid::Uuid::new_v4(),
        width: 1280,
        height: 720,
        background: echo_core::BackgroundLayer::Gradient {
            from: "#0F172A".into(),
            to: "#1E1B4B".into(),
            angle_deg: 135.0,
        },
        layers: vec![echo_core::SceneLayer {
            id: "slide".into(),
            name: "Slide".into(),
            visible: true,
            opacity: 1.0,
            objects: vec![echo_core::SceneObject::Text {
                content: "EchoPresenter GPU Smoke Test — John 3:16".into(),
                font_family: "Segoe UI".into(),
                font_size: 48.0,
                font_weight: 700,
                color: "#FFFFFF".into(),
                position: echo_core::Vec2 { x: 80.0, y: 260.0 },
                size: echo_core::Size2D { w: 1120.0, h: 200.0 },
                outline: None,
                shadow: None,
            }],
        }],
    };

    for look in ["main-fullscreen", "ndi-lower-third"] {
        let scene = build_vello_scene(
            &graph,
            look,
            None,
            None,
            &mut text_renderer,
            &mut image_cache,
        );
        let base_color = if look == "ndi-lower-third" {
            Color::TRANSPARENT
        } else {
            Color::BLACK
        };
        vello
            .render_to_texture(
                &device,
                &queue,
                &scene,
                &view,
                &RenderParams {
                    base_color,
                    width: 1280,
                    height: 720,
                    antialiasing_method: AaConfig::Area,
                },
            )
            .expect("Vello render_to_texture failed");
    }

    let mut encoder = device.create_command_encoder(&wgpu::CommandEncoderDescriptor {
        label: Some("smoke-copy"),
    });
    encoder.copy_texture_to_buffer(
        wgpu::TexelCopyTextureInfo {
            texture: &tex,
            mip_level: 0,
            origin: wgpu::Origin3d::ZERO,
            aspect: wgpu::TextureAspect::All,
        },
        wgpu::TexelCopyBufferInfo {
            buffer: &staging,
            layout: wgpu::TexelCopyBufferLayout {
                offset: 0,
                bytes_per_row: Some(1280 * 4),
                rows_per_image: Some(720),
            },
        },
        wgpu::Extent3d {
            width: 1280,
            height: 720,
            depth_or_array_layers: 1,
        },
    );
    queue.submit(Some(encoder.finish()));

    let slice = staging.slice(..);
    let (tx, rx) = std::sync::mpsc::channel();
    slice.map_async(wgpu::MapMode::Read, move |res| {
        let _ = tx.send(res);
    });
    let _ = device.poll(wgpu::PollType::wait_indefinitely());
    rx.recv().unwrap().unwrap();
    let data = slice.get_mapped_range().unwrap();
    let img = image::RgbaImage::from_raw(1280, 720, data.to_vec()).unwrap();
    let _ = std::fs::create_dir_all("target");
    img.save("target/smoke_preview.png")
        .expect("Failed to save target/smoke_preview.png");
    println!("SMOKE_TEST_SAVED: target/smoke_preview.png");
}

fn main() {
    tracing_subscriber::fmt::init();
    if std::env::args().any(|a| a == "--smoke-test") {
        run_smoke_test();
        return;
    }
    info!("Starting echo-renderer companion process...");

    let event_loop = EventLoop::<UserEvent>::with_user_event()
        .build()
        .expect("Failed to create winit EventLoop");
    let proxy = event_loop.create_proxy();
    let (event_tx, event_rx) = mpsc::unbounded_channel();

    spawn_pipe_server(proxy.clone(), event_rx);

    let mut app = RendererApp::new(proxy, event_tx);
    event_loop.run_app(&mut app).expect("EventLoop error");
}
