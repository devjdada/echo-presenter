pub mod scene_builder;
pub mod text;

pub use scene_builder::{build_vello_scene, parse_hex_color, ImageCache};
pub use text::{TextAlign, TextRenderer, TextStyleSpec};
pub use vello;
pub use vello::kurbo;
pub use vello::peniko;
pub use vello::wgpu;

use echo_core::{EchoError, SceneGraph};

/// Graphics rendering trait implemented by `echo-renderer`
/// using `vello::wgpu` + `vello` + `cosmic-text`.
pub trait GraphicsBackend: Send + Sync {
    fn adapter_name(&self) -> &str;
    fn render_scene(&mut self, output_id: &str, scene: &SceneGraph) -> Result<(), EchoError>;
}
