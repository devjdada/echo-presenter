use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TranscriptSegment {
    pub text: String,
    pub is_final: bool,
    pub timestamp_secs: u64,
}

pub trait SttProvider: Send + Sync {
    fn provider_name(&self) -> &str;
    fn transcribe_chunk(&self, pcm_16khz: &[f32]) -> Option<TranscriptSegment>;
}

pub struct SimulatedSttProvider;

impl SttProvider for SimulatedSttProvider {
    fn provider_name(&self) -> &str {
        "simulated-stt"
    }

    fn transcribe_chunk(&self, _pcm_16khz: &[f32]) -> Option<TranscriptSegment> {
        Some(TranscriptSegment {
            text: "Let's read John 3:16 together — For God so loved the world.".into(),
            is_final: true,
            timestamp_secs: 1,
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_simulated_stt() {
        let stt = SimulatedSttProvider;
        let seg = stt.transcribe_chunk(&[0.0; 160]).unwrap();
        assert!(seg.is_final);
        assert!(seg.text.contains("John 3:16"));
    }
}
