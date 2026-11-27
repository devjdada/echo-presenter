use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ExtractedQuote {
    pub id: String,
    pub quote_text: String,
    pub cleaned_grammar: String,
    pub category: String,
    pub punchiness_score: f32,
    pub speaker: String,
}

#[derive(Debug, Default)]
pub struct TranscriptBuffer {
    chunks: Vec<(u64, String)>,
    window_secs: u64,
}

impl TranscriptBuffer {
    pub fn new(window_secs: u64) -> Self {
        Self {
            chunks: Vec::new(),
            window_secs,
        }
    }

    pub fn push(&mut self, timestamp_secs: u64, text: String) {
        self.chunks.push((timestamp_secs, text));
        let cutoff = timestamp_secs.saturating_sub(self.window_secs);
        self.chunks.retain(|(ts, _)| *ts >= cutoff);
    }

    pub fn combined_text(&self) -> String {
        self.chunks
            .iter()
            .map(|(_, t)| t.as_str())
            .collect::<Vec<_>>()
            .join(" ")
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_rolling_transcript_buffer() {
        let mut buf = TranscriptBuffer::new(30);
        buf.push(10, "Old sentence.".into());
        buf.push(45, "God doesn't call the qualified.".into());
        assert_eq!(buf.combined_text(), "God doesn't call the qualified.");
    }
}
