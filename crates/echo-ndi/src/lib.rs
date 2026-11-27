use echo_core::EchoError;

/// NDI 6 output trait for premultiplied RGBA alpha streams.
/// Implementation gated behind `ndi-sdk` feature (`grafton-ndi`).
pub trait NdiSender: Send + Sync {
    fn stream_name(&self) -> &str;
    fn send_rgba_frame(&mut self, width: u32, height: u32, data: &[u8]) -> Result<(), EchoError>;
}

pub struct StubNdiSender {
    name: String,
}

impl StubNdiSender {
    pub fn new(name: impl Into<String>) -> Self {
        Self { name: name.into() }
    }
}

impl NdiSender for StubNdiSender {
    fn stream_name(&self) -> &str {
        &self.name
    }
    fn send_rgba_frame(&mut self, _width: u32, _height: u32, _data: &[u8]) -> Result<(), EchoError> {
        Ok(())
    }
}
