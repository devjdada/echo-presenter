use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SceneGraph {
    pub scene_id: Uuid,
    pub width: u32,
    pub height: u32,
    pub background: BackgroundLayer,
    pub layers: Vec<SceneLayer>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(tag = "type", rename_all = "snake_case")]
pub enum BackgroundLayer {
    Solid { color: String },
    Gradient { from: String, to: String, angle_deg: f32 },
    Image { source: String, opacity: f32 },
    Video { source: String, opacity: f32 },
    Transparent,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SceneLayer {
    pub id: String,
    pub name: String,
    pub visible: bool,
    pub opacity: f32,
    pub objects: Vec<SceneObject>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(tag = "type", rename_all = "snake_case")]
pub enum SceneObject {
    Text {
        content: String,
        font_family: String,
        font_size: f32,
        font_weight: u16,
        color: String,
        position: Vec2,
        size: Size2D,
        outline: Option<TextOutline>,
        shadow: Option<TextShadow>,
    },
    Image {
        source: String,
        position: Vec2,
        size: Size2D,
        opacity: f32,
    },
    Shape {
        kind: ShapeKind,
        position: Vec2,
        size: Size2D,
        color: String,
        corner_radius: f32,
    },
}

#[derive(Debug, Clone, Copy, Serialize, Deserialize)]
pub struct Vec2 {
    pub x: f32,
    pub y: f32,
}

#[derive(Debug, Clone, Copy, Serialize, Deserialize)]
pub struct Size2D {
    pub w: f32,
    pub h: f32,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TextOutline {
    pub color: String,
    pub width: f32,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TextShadow {
    pub color: String,
    pub offset: [f32; 2],
    pub blur: f32,
}

#[derive(Debug, Clone, Copy, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum ShapeKind {
    Rect,
    Ellipse,
}
