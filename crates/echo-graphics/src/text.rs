use std::collections::HashMap;
use cosmic_text::{
    Attrs, Buffer, Family, FontSystem, Metrics, Shaping, Weight, Wrap,
};
use vello::kurbo::Affine;
use vello::peniko::{Blob, Color, Fill, FontData};
use vello::{Glyph, Scene};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum TextAlign {
    Left,
    Center,
    Right,
}

#[derive(Debug, Clone)]
pub struct TextStyleSpec {
    pub font_family: String,
    pub font_size: f32,
    pub font_weight: u16,
    pub color: Color,
    pub align: TextAlign,
    pub max_width: f32,
    pub max_height: f32,
    pub shadow_color: Option<Color>,
    pub shadow_offset: [f32; 2],
}

pub struct TextRenderer {
    font_system: FontSystem,
    font_cache: HashMap<cosmic_text::fontdb::ID, FontData>,
}

impl Default for TextRenderer {
    fn default() -> Self {
        Self::new()
    }
}

impl TextRenderer {
    pub fn new() -> Self {
        Self {
            font_system: FontSystem::new(),
            font_cache: HashMap::new(),
        }
    }

    fn get_or_cache_font(&mut self, id: cosmic_text::fontdb::ID) -> Option<FontData> {
        if let Some(cached) = self.font_cache.get(&id) {
            return Some(cached.clone());
        }
        let face = self.font_system.db().face(id)?;
        let data: Vec<u8> = match &face.source {
            cosmic_text::fontdb::Source::Binary(b) => b.as_ref().as_ref().to_vec(),
            cosmic_text::fontdb::Source::File(path) => std::fs::read(path).ok()?,
            cosmic_text::fontdb::Source::SharedFile(path, _) => std::fs::read(path).ok()?,
        };
        let font_data = FontData::new(Blob::from(data), face.index);
        self.font_cache.insert(id, font_data.clone());
        Some(font_data)
    }

    /// Layout `text` with `cosmic-text` and emit `vello::Scene::draw_glyphs` calls.
    pub fn draw_text(
        &mut self,
        scene: &mut Scene,
        text: &str,
        origin_x: f64,
        origin_y: f64,
        style: &TextStyleSpec,
    ) {
        let line_height = style.font_size * 1.28;
        let metrics = Metrics::new(style.font_size, line_height);
        let mut buffer = Buffer::new(&mut self.font_system, metrics);

        buffer.set_size(
            Some(style.max_width.max(10.0)),
            Some(style.max_height.max(10.0)),
        );
        buffer.set_wrap(Wrap::Word);

        let attrs = Attrs::new()
            .family(Family::Name(&style.font_family))
            .weight(Weight(style.font_weight));

        buffer.set_text(text, &attrs, Shaping::Advanced, None);
        buffer.shape_until_scroll(&mut self.font_system, false);

        // Collect shaped glyphs grouped by font ID so we can draw shadow + fill cleanly.
        let mut runs: Vec<(cosmic_text::fontdb::ID, Vec<Glyph>)> = Vec::new();

        for run in buffer.layout_runs() {
            let line_width = run.line_w;
            let align_offset_x = match style.align {
                TextAlign::Left => 0.0,
                TextAlign::Center => ((style.max_width - line_width) * 0.5).max(0.0),
                TextAlign::Right => (style.max_width - line_width).max(0.0),
            };

            let mut current_font: Option<cosmic_text::fontdb::ID> = None;
            let mut current_glyphs: Vec<Glyph> = Vec::new();

            for glyph in run.glyphs.iter() {
                let phys = glyph.physical((0., 0.), 1.0);
                if current_font != Some(phys.cache_key.font_id) {
                    if let Some(fid) = current_font.take()
                        && !current_glyphs.is_empty()
                    {
                        runs.push((fid, std::mem::take(&mut current_glyphs)));
                    }
                    current_font = Some(phys.cache_key.font_id);
                }
                current_glyphs.push(Glyph {
                    id: phys.cache_key.glyph_id as u32,
                    x: phys.x as f32 + align_offset_x,
                    y: run.line_y + phys.y as f32,
                });
            }

            if let Some(fid) = current_font
                && !current_glyphs.is_empty()
            {
                runs.push((fid, current_glyphs));
            }
        }

        // Draw optional drop shadow first
        if let Some(shadow_color) = style.shadow_color {
            let shadow_transform = Affine::translate((
                origin_x + style.shadow_offset[0] as f64,
                origin_y + style.shadow_offset[1] as f64,
            ));
            for (fid, glyphs) in &runs {
                if let Some(font) = self.get_or_cache_font(*fid) {
                    scene
                        .draw_glyphs(&font)
                        .font_size(style.font_size)
                        .transform(shadow_transform)
                        .brush(shadow_color)
                        .draw(Fill::NonZero, glyphs.iter().copied());
                }
            }
        }

        // Draw main text fill
        let transform = Affine::translate((origin_x, origin_y));
        for (fid, glyphs) in &runs {
            if let Some(font) = self.get_or_cache_font(*fid) {
                scene
                    .draw_glyphs(&font)
                    .font_size(style.font_size)
                    .transform(transform)
                    .brush(style.color)
                    .draw(Fill::NonZero, glyphs.iter().copied());
            }
        }
    }
}
