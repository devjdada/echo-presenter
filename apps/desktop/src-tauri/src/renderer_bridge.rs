use std::path::PathBuf;
use std::process::{Child, Command};
use std::sync::Mutex;

use echo_core::{read_framed, write_framed, RendererCommand, RendererEvent, RENDERER_PIPE_NAME};
use tauri::{AppHandle, Emitter};
use tokio::net::windows::named_pipe::ClientOptions;
use tokio::sync::mpsc;
use tracing::{error, info, warn};

pub struct RendererBridge {
    cmd_tx: mpsc::UnboundedSender<RendererCommand>,
    child: Mutex<Option<Child>>,
}

impl RendererBridge {
    pub fn start(app: AppHandle) -> Self {
        let (cmd_tx, mut cmd_rx) = mpsc::unbounded_channel::<RendererCommand>();
        let child = spawn_renderer_process();
        let bridge_child = Mutex::new(child);

        tauri::async_runtime::spawn(async move {
            loop {
                let client = loop {
                    match ClientOptions::new().open(RENDERER_PIPE_NAME) {
                        Ok(c) => break c,
                        Err(_) => {
                            tokio::time::sleep(std::time::Duration::from_millis(250)).await;
                        }
                    }
                };

                info!("Connected to echo-renderer Named Pipe");
                let (mut reader, mut writer) = tokio::io::split(client);

                // Initial Ping
                let _ = write_framed(&mut writer, &RendererCommand::Ping).await;

                let app_clone = app.clone();
                let read_task = tokio::spawn(async move {
                    while let Ok(ev) = read_framed::<_, RendererEvent>(&mut reader).await {
                        match &ev {
                            RendererEvent::Pong {
                                adapter_name,
                                backend,
                            } => {
                                info!(adapter = %adapter_name, backend = %backend, "echo-renderer Pong");
                                let _ = app_clone.emit("renderer://pong", &ev);
                            }
                            RendererEvent::MonitorsList { .. } => {
                                let _ = app_clone.emit("renderer://monitors", &ev);
                            }
                            RendererEvent::PreviewFrame { .. } => {
                                let _ = app_clone.emit("renderer://preview-frame", &ev);
                            }
                            RendererEvent::Error { message } => {
                                error!("echo-renderer error: {message}");
                            }
                            RendererEvent::FrameReady { .. } => {}
                        }
                    }
                });

                loop {
                    tokio::select! {
                        Some(cmd) = cmd_rx.recv() => {
                            if write_framed(&mut writer, &cmd).await.is_err() {
                                warn!("Failed writing to echo-renderer pipe, reconnecting...");
                                break;
                            }
                        }
                        _ = tokio::time::sleep(std::time::Duration::from_secs(5)) => {
                            if read_task.is_finished() {
                                warn!("echo-renderer read task ended, reconnecting...");
                                break;
                            }
                            let _ = write_framed(&mut writer, &RendererCommand::Ping).await;
                        }
                    }
                }
            }
        });

        Self {
            cmd_tx,
            child: bridge_child,
        }
    }

    pub fn send(&self, cmd: RendererCommand) {
        let _ = self.cmd_tx.send(cmd);
    }

    pub fn shutdown(&self) {
        let _ = self.cmd_tx.send(RendererCommand::Shutdown);
        if let Ok(mut guard) = self.child.lock()
            && let Some(mut child) = guard.take()
        {
            let _ = child.kill();
        }
    }
}

fn find_renderer_binary() -> Option<PathBuf> {
    if let Ok(exe) = std::env::current_exe()
        && let Some(dir) = exe.parent()
    {
        let candidate = dir.join("echo-renderer.exe");
        if candidate.exists() {
            return Some(candidate);
        }
    }
    let dev_candidate = PathBuf::from(env!("CARGO_MANIFEST_DIR"))
        .join("../../../target/debug/echo-renderer.exe");
    if dev_candidate.exists() {
        return Some(dev_candidate);
    }
    None
}

fn spawn_renderer_process() -> Option<Child> {
    let bin = match find_renderer_binary() {
        Some(b) => b,
        None => {
            warn!("echo-renderer.exe not found; build it with `cargo build -p echo-renderer`");
            return None;
        }
    };
    info!("Spawning echo-renderer from {:?}", bin);
    match Command::new(bin).spawn() {
        Ok(child) => Some(child),
        Err(e) => {
            error!("Failed to spawn echo-renderer: {e}");
            None
        }
    }
}
