use std::collections::HashMap;
use echo_core::{BackgroundLayer, SceneGraph, SceneObject};
use vello::kurbo::{Affine, Rect, RoundedRect};
use vello::peniko::{Blob, Color, Fill, Gradient, ImageAlphaType, ImageBrush, ImageData, ImageFormat};
use vello::Scene;

use crate::text::{TextAlign, TextRenderer, TextStyleSpec};

#[derive(Default)]
pub struct ImageCache {
    images: HashMap<String, ImageData>,
}

impl ImageCache {
    pub fn get_or_load(&mut self, path: &str) -> Option<ImageData> {
        if let Some(img) = self.images.get(path) {
            return Some(img.clone());
        }
        let decoded = image::open(path).ok()?.into_rgba8();
        let (width, height) = decoded.dimensions();
        let data = ImageData {
            data: Blob::from(decoded.into_raw()),
            format: ImageFormat::Rgba8,
            alpha_type: ImageAlphaType::Alpha,
            width,
            height,
        };
        self.images.insert(path.to_string(), data.clone());
        Some(data)
    }
}

pub fn parse_hex_color(hex: &str) -> Color {
    let s = hex.trim_start_matches('#');
    let parse_byte = |idx: usize| -> u8 {
        s.get(idx..idx + 2)
            .and_then(|h| u8::from_str_radix(h, 16).ok())
            .unwrap_or(255)
    };
    match s.len() {
        6 => Color::from_rgba8(parse_byte(0), parse_byte(2), parse_byte(4), 255),
        8 => Color::from_rgba8(parse_byte(0), parse_byte(2), parse_byte(4), parse_byte(6)),
        _ => Color::from_rgba8(15, 23, 42, 255),
    }
}

/// Build a `vello::Scene` from a `SceneGraph` and apply the active Output Look transformation.
pub fn build_vello_scene(
    graph: &SceneGraph,
    look_id: &str,
    next_slide_text: Option<&str>,
    stage_message: Option<&str>,
    text_renderer: &mut TextRenderer,
    image_cache: &mut ImageCache,
) -> Scene {
    let mut scene = Scene::new();
    let w = graph.width as f64;
    let h = graph.height as f64;

    match look_id {
        "ndi-lower-third" => {
            // Transparent RGBA background (alpha = 0.0) + bottom-anchored styled lower-third bar.
            let bar_h = h * 0.20;
            let bar_y = h - bar_h - (h * 0.05);
            let bar_x = w * 0.05;
            let bar_w = w * 0.90;

            let bar_rect = RoundedRect::new(bar_x, bar_y, bar_x + bar_w, bar_y + bar_h, 14.0);
            scene.fill(
                Fill::NonZero,
                Affine::IDENTITY,
                Color::from_rgba8(12, 14, 20, 235),
                None,
                &bar_rect,
            );

            // Accent left stripe
            let accent = Rect::new(bar_x, bar_y + 12.0, bar_x + 8.0, bar_y + bar_h - 12.0);
            scene.fill(
                Fill::NonZero,
                Affine::IDENTITY,
                Color::from_rgba8(245, 158, 11, 255),
                None,
                &accent,
            );

            // Extract primary text from slide layers
            let primary_text = extract_primary_text(graph);
            if !primary_text.is_empty() {
                text_renderer.draw_text(
                    &mut scene,
                    &primary_text,
                    bar_x + 36.0,
                    bar_y + 28.0,
                    &TextStyleSpec {
                        font_family: "Segoe UI".into(),
                        font_size: (h * 0.038) as f32,
                        font_weight: 600,
                        color: Color::WHITE,
                        align: TextAlign::Left,
                        max_width: (bar_w - 72.0) as f32,
                        max_height: (bar_h - 48.0) as f32,
                        shadow_color: Some(Color::from_rgba8(0, 0, 0, 180)),
                        shadow_offset: [2.0, 2.0],
                    },
                );
            }
        }
        "stage-confidence" => {
            // High-contrast black background + large current slide + next slide preview + stage message
            let bg = Rect::new(0.0, 0.0, w, h);
            scene.fill(Fill::NonZero, Affine::IDENTITY, Color::BLACK, None, &bg);

            let primary_text = extract_primary_text(graph);
            if !primary_text.is_empty() {
                text_renderer.draw_text(
                    &mut scene,
                    &primary_text,
                    w * 0.06,
                    h * 0.14,
                    &TextStyleSpec {
                        font_family: "Segoe UI".into(),
                        font_size: (h * 0.062) as f32,
                        font_weight: 700,
                        color: Color::WHITE,
                        align: TextAlign::Left,
                        max_width: (w * 0.88) as f32,
                        max_height: (h * 0.45) as f32,
                        shadow_color: None,
                        shadow_offset: [0.0, 0.0],
                    },
                );
            }

            // Divider line
            let divider = Rect::new(w * 0.06, h * 0.64, w * 0.94, h * 0.643);
            scene.fill(
                Fill::NonZero,
                Affine::IDENTITY,
                Color::from_rgba8(63, 63, 70, 255),
                None,
                &divider,
            );

            // Next slide preview
            let next_label = format!("NEXT: {}", next_slide_text.unwrap_or("End of Item"));
            text_renderer.draw_text(
                &mut scene,
                &next_label,
                w * 0.06,
                h * 0.68,
                &TextStyleSpec {
                    font_family: "Segoe UI".into(),
                    font_size: (h * 0.036) as f32,
                    font_weight: 500,
                    color: Color::from_rgba8(161, 161, 170, 255),
                    align: TextAlign::Left,
                    max_width: (w * 0.88) as f32,
                    max_height: (h * 0.18) as f32,
                    shadow_color: None,
                    shadow_offset: [0.0, 0.0],
                },
            );

            if let Some(msg) = stage_message.filter(|m| !m.trim().is_empty()) {
                text_renderer.draw_text(
                    &mut scene,
                    msg,
                    w * 0.06,
                    h * 0.88,
                    &TextStyleSpec {
                        font_family: "Segoe UI".into(),
                        font_size: (h * 0.032) as f32,
                        font_weight: 700,
                        color: Color::from_rgba8(251, 191, 36, 255),
                        align: TextAlign::Left,
                        max_width: (w * 0.88) as f32,
                        max_height: (h * 0.10) as f32,
                        shadow_color: None,
                        shadow_offset: [0.0, 0.0],
                    },
                );
            }
        }
        _ => {
            // "main-fullscreen" (default): Full background + centered scaled content
            let bg_rect = Rect::new(0.0, 0.0, w, h);
            match &graph.background {
                BackgroundLayer::Solid { color } => {
                    scene.fill(
                        Fill::NonZero,
                        Affine::IDENTITY,
                        parse_hex_color(color),
                        None,
                        &bg_rect,
                    );
                }
                BackgroundLayer::Gradient { from, to, .. } => {
                    let grad = Gradient::new_linear((0.0, 0.0), (w, h))
                        .with_stops([parse_hex_color(from), parse_hex_color(to)]);
                    scene.fill(Fill::NonZero, Affine::IDENTITY, &grad, None, &bg_rect);
                }
                BackgroundLayer::Image { source, opacity } => {
                    if let Some(img) = image_cache.get_or_load(source) {
                        let sx = w / img.width.max(1) as f64;
                        let sy = h / img.height.max(1) as f64;
                        let brush = ImageBrush::new(img).with_alpha(*opacity);
                        scene.draw_image(&brush, Affine::scale_non_uniform(sx, sy));
                    }
                }
                BackgroundLayer::Video { .. } => {
                    scene.fill(
                        Fill::NonZero,
                        Affine::IDENTITY,
                        Color::from_rgba8(15, 23, 42, 255),
                        None,
                        &bg_rect,
                    );
                }
                BackgroundLayer::Transparent => {}
            }

            for layer in &graph.layers {
                if !layer.visible {
                    continue;
                }
                for obj in &layer.objects {
                    match obj {
                        SceneObject::Text {
                            content,
                            font_family,
                            font_size,
                            font_weight,
                            color,
                            position,
                            size,
                            shadow,
                            ..
                        } => {
                            text_renderer.draw_text(
                                &mut scene,
                                content,
                                position.x as f64,
                                position.y as f64,
                                &TextStyleSpec {
                                    font_family: font_family.clone(),
                                    font_size: *font_size,
                                    font_weight: *font_weight,
                                    color: parse_hex_color(color),
                                    align: TextAlign::Center,
                                    max_width: size.w,
                                    max_height: size.h,
                                    shadow_color: shadow
                                        .as_ref()
                                        .map(|s| parse_hex_color(&s.color)),
                                    shadow_offset: shadow
                                        .as_ref()
                                        .map(|s| s.offset)
                                        .unwrap_or([3.0, 3.0]),
                                },
                            );
                        }
                        SceneObject::Image {
                            source,
                            position,
                            size,
                            opacity,
                        } => {
                            if let Some(img) = image_cache.get_or_load(source) {
                                let sx = size.w as f64 / img.width.max(1) as f64;
                                let sy = size.h as f64 / img.height.max(1) as f64;
                                let xform = Affine::translate((position.x as f64, position.y as f64))
                                    * Affine::scale_non_uniform(sx, sy);
                                let brush = ImageBrush::new(img).with_alpha(*opacity);
                                scene.draw_image(&brush, xform);
                            }
                        }
                        SceneObject::Shape {
                            position,
                            size,
                            color,
                            corner_radius,
                            ..
                        } => {
                            let rect = RoundedRect::new(
                                position.x as f64,
                                position.y as f64,
                                (position.x + size.w) as f64,
                                (position.y + size.h) as f64,
                                *corner_radius as f64,
                            );
                            scene.fill(
                                Fill::NonZero,
                                Affine::IDENTITY,
                                parse_hex_color(color),
                                None,
                                &rect,
                            );
                        }
                    }
                }
            }
        }
    }

    scene
}

fn extract_primary_text(graph: &SceneGraph) -> String {
    for layer in &graph.layers {
        if !layer.visible {
            continue;
        }
        for obj in &layer.objects {
            if let SceneObject::Text { content, .. } = obj {
                return content.clone();
            }
        }
    }
    String::new()
}

#[cfg(test)]
mod tests {
    use super::*;
    use echo_core::{SceneLayer, Size2D, Vec2};
    use uuid::Uuid;

    fn sample_scene() -> SceneGraph {
        SceneGraph {
            scene_id: Uuid::new_v4(),
            width: 1920,
            height: 1080,
            background: BackgroundLayer::Gradient {
                from: "#0F172A".into(),
                to: "#1E1B4B".into(),
                angle_deg: 135.0,
            },
            layers: vec![SceneLayer {
                id: "slide".into(),
                name: "Slide".into(),
                visible: true,
                opacity: 1.0,
                objects: vec![SceneObject::Text {
                    content: "Amazing grace, how sweet the sound".into(),
                    font_family: "Segoe UI".into(),
                    font_size: 64.0,
                    font_weight: 700,
                    color: "#FFFFFF".into(),
                    position: Vec2 { x: 160.0, y: 400.0 },
                    size: Size2D { w: 1600.0, h: 300.0 },
                    outline: None,
                    shadow: None,
                }],
            }],
        }
    }

    #[test]
    fn test_output_looks_build_scenes() {
        let graph = sample_scene();
        let mut tr = TextRenderer::new();
        let mut ic = ImageCache::default();

        for look in ["main-fullscreen", "ndi-lower-third", "stage-confidence"] {
            let scene = build_vello_scene(
                &graph,
                look,
                Some("Next verse"),
                Some("2 mins left"),
                &mut tr,
                &mut ic,
            );
            assert!(!scene.encoding().is_empty(), "Look {look} produced empty scene");
        }
    }
}
