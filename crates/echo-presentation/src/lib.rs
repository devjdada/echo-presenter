pub mod db;
pub use db::*;

use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "snake_case")]
pub enum SlideGroupKind {
    Verse,
    Chorus,
    Bridge,
    Scripture,
    Quote,
    Announcement,
    Custom,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Slide {
    pub id: String,
    pub label: String,
    pub group: SlideGroupKind,
    pub text: String,
    pub reference: Option<String>,
    pub background_color: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Presentation {
    pub id: Uuid,
    pub title: String,
    pub slides: Vec<Slide>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PlaylistItem {
    pub id: String,
    pub title: String,
    pub kind: String,
    pub presentation_id: Option<String>,
}
