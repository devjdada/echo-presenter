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

#[cfg(feature = "local-whisper")]
pub struct LocalWhisperProvider {
    ctx: whisper_rs::WhisperContext,
}

#[cfg(feature = "local-whisper")]
impl LocalWhisperProvider {
    pub fn new(model_path: &str) -> Result<Self, String> {
        let mut params = whisper_rs::WhisperContextParameters::default();
        params.use_gpu(true);
        let ctx = whisper_rs::WhisperContext::new_with_params(model_path, params)
            .map_err(|e| format!("Failed to load Whisper model: {e}"))?;
        Ok(Self { ctx })
    }
}

#[cfg(feature = "local-whisper")]
impl SttProvider for LocalWhisperProvider {
    fn provider_name(&self) -> &str {
        "local-whisper-cuda"
    }

    fn transcribe_chunk(&self, pcm_16khz: &[f32]) -> Option<TranscriptSegment> {
        let mut state = self.ctx.create_state().ok()?;
        let mut params = whisper_rs::FullParams::new(whisper_rs::SamplingStrategy::Greedy { best_of: 1 });
        params.set_print_special(false);
        params.set_print_progress(false);
        params.set_print_realtime(false);
        params.set_print_timestamps(false);
        state.full(params, pcm_16khz).ok()?;
        let num_segments = state.full_n_segments();
        let mut out = String::new();
        for i in 0..num_segments {
            if let Some(seg) = state.get_segment(i) {
                if let Ok(text) = seg.to_str_lossy() {
                    out.push_str(&text);
                }
            }
        }
        let trimmed = out.trim().to_string();
        if trimmed.is_empty() {
            None
        } else {
            Some(TranscriptSegment {
                text: trimmed,
                is_final: true,
                timestamp_secs: 0,
            })
        }
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
