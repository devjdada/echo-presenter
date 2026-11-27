pub mod docx;
pub mod pptx;
pub use docx::*;
pub use pptx::*;

use echo_presentation::{Slide, SlideGroupKind};
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum SemanticTemplate {
    TitleHero,
    ScriptureSplit,
    QuoteCard,
    StoryVisual,
    TwoColumnPoint,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ContentBeat {
    pub template: SemanticTemplate,
    pub title: String,
    pub body: String,
}

/// Chunk raw sermon notes, markdown, or story text into semantic slide beats.
pub fn chunk_text_to_beats(input: &str) -> Vec<ContentBeat> {
    let mut beats = Vec::new();
    for para in input.split("\n\n").map(str::trim).filter(|p| !p.is_empty()) {
        if para.starts_with("# ") {
            beats.push(ContentBeat {
                template: SemanticTemplate::TitleHero,
                title: para.trim_start_matches("# ").to_string(),
                body: String::new(),
            });
        } else if para.starts_with('>') {
            beats.push(ContentBeat {
                template: SemanticTemplate::QuoteCard,
                title: "Quote".into(),
                body: para.trim_start_matches('>').trim().to_string(),
            });
        } else if para.contains(':') && para.len() < 80 {
            beats.push(ContentBeat {
                template: SemanticTemplate::ScriptureSplit,
                title: "Scripture".into(),
                body: para.to_string(),
            });
        } else {
            beats.push(ContentBeat {
                template: SemanticTemplate::StoryVisual,
                title: "Point".into(),
                body: para.to_string(),
            });
        }
    }
    beats
}

pub fn beats_to_slides(beats: &[ContentBeat]) -> Vec<Slide> {
    beats
        .iter()
        .enumerate()
        .map(|(i, b)| Slide {
            id: format!("ingest-{}", i + 1),
            label: b.title.clone(),
            group: match b.template {
                SemanticTemplate::ScriptureSplit => SlideGroupKind::Scripture,
                SemanticTemplate::QuoteCard => SlideGroupKind::Quote,
                SemanticTemplate::TitleHero => SlideGroupKind::Announcement,
                _ => SlideGroupKind::Custom,
            },
            text: if b.body.is_empty() {
                b.title.clone()
            } else {
                b.body.clone()
            },
            reference: None,
            background_color: "#0F172A".into(),
        })
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_beat_chunking() {
        let md = "# Walking in the Light\n\n> God is light and in Him is no darkness.\n\nA story of hope.";
        let beats = chunk_text_to_beats(md);
        assert_eq!(beats.len(), 3);
        assert_eq!(beats[0].template, SemanticTemplate::TitleHero);
        assert_eq!(beats[1].template, SemanticTemplate::QuoteCard);
        assert_eq!(beats[2].template, SemanticTemplate::StoryVisual);
        let slides = beats_to_slides(&beats);
        assert_eq!(slides.len(), 3);
    }
}
