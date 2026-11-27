use echo_core::EchoError;

/// Media playback trait. GStreamer implementation is gated behind `gstreamer-backend`.
pub trait MediaPipeline: Send + Sync {
    fn load(&mut self, uri: &str) -> Result<(), EchoError>;
    fn play(&mut self) -> Result<(), EchoError>;
    fn pause(&mut self) -> Result<(), EchoError>;
    fn stop(&mut self) -> Result<(), EchoError>;
}

pub struct StubMediaPipeline;

impl MediaPipeline for StubMediaPipeline {
    fn load(&mut self, _uri: &str) -> Result<(), EchoError> {
        Ok(())
    }
    fn play(&mut self) -> Result<(), EchoError> {
        Ok(())
    }
    fn pause(&mut self) -> Result<(), EchoError> {
        Ok(())
    }
    fn stop(&mut self) -> Result<(), EchoError> {
        Ok(())
    }
}
