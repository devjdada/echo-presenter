use std::io::Read;
use std::path::Path;
use echo_presentation::{Slide, SlideGroupKind};
use quick_xml::events::Event;
use quick_xml::Reader;
use thiserror::Error;
use zip::ZipArchive;

#[derive(Debug, Error)]
pub enum PptxError {
    #[error("IO error: {0}")]
    Io(#[from] std::io::Error),
    #[error("ZIP error: {0}")]
    Zip(#[from] zip::result::ZipError),
    #[error("XML error: {0}")]
    Xml(#[from] quick_xml::Error),
}

/// Extract plain text paragraphs from a single PowerPoint slide XML (`ppt/slides/slideN.xml`).
pub fn extract_slide_text_from_xml(xml: &str) -> Result<String, PptxError> {
    let mut reader = Reader::from_str(xml);
    let mut buf = Vec::new();
    let mut in_text = false;
    let mut lines: Vec<String> = Vec::new();
    let mut current_line = String::new();

    loop {
        match reader.read_event_into(&mut buf) {
            Ok(Event::Start(e)) => {
                let name = e.name();
                if name.as_ref().ends_with(b":t") || name.as_ref() == b"t" {
                    in_text = true;
                } else if (name.as_ref().ends_with(b":p") || name.as_ref() == b"p")
                    && !current_line.trim().is_empty()
                {
                    lines.push(current_line.trim().to_string());
                    current_line.clear();
                }
            }
            Ok(Event::End(e)) => {
                let name = e.name();
                if name.as_ref().ends_with(b":t") || name.as_ref() == b"t" {
                    in_text = false;
                } else if (name.as_ref().ends_with(b":p") || name.as_ref() == b"p")
                    && !current_line.trim().is_empty()
                {
                    lines.push(current_line.trim().to_string());
                    current_line.clear();
                }
            }
            Ok(Event::Text(e)) if in_text => {
                if let Ok(t) = e.unescape() {
                    current_line.push_str(&t);
                }
            }
            Ok(Event::Eof) => break,
            Err(e) => return Err(PptxError::Xml(e)),
            _ => {}
        }
        buf.clear();
    }
    if !current_line.trim().is_empty() {
        lines.push(current_line.trim().to_string());
    }
    Ok(lines.join("\n"))
}

/// Import a `.pptx` file into EchoPresenter `Slide` items.
pub fn import_pptx_file(path: &Path) -> Result<Vec<Slide>, PptxError> {
    let file = std::fs::File::open(path)?;
    let mut archive = ZipArchive::new(file)?;
    let mut slide_names: Vec<String> = (0..archive.len())
        .filter_map(|i| {
            let name = archive.by_index(i).ok()?.name().to_string();
            if name.starts_with("ppt/slides/slide") && name.ends_with(".xml") {
                Some(name)
            } else {
                None
            }
        })
        .collect();

    slide_names.sort_by_key(|name| {
        name.trim_start_matches("ppt/slides/slide")
            .trim_end_matches(".xml")
            .parse::<usize>()
            .unwrap_or(0)
    });

    let mut slides = Vec::new();
    for (idx, name) in slide_names.iter().enumerate() {
        let mut entry = archive.by_name(name)?;
        let mut xml = String::new();
        entry.read_to_string(&mut xml)?;
        let text = extract_slide_text_from_xml(&xml)?;
        slides.push(Slide {
            id: format!("pptx-{}", idx + 1),
            label: format!("Slide {}", idx + 1),
            group: SlideGroupKind::Custom,
            text: if text.is_empty() {
                format!("Slide {}", idx + 1)
            } else {
                text
            },
            reference: None,
            background_color: "#0F172A".into(),
        });
    }
    Ok(slides)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_extract_pptx_slide_xml() {
        let xml = r#"<p:sld xmlns:a="http://schemas.openxmlformats.org/drawingml/2006/main" xmlns:p="http://schemas.openxmlformats.org/presentationml/2006/main">
            <p:cSld><p:spTree><p:sp><p:txBody>
                <a:p><a:r><a:t>Welcome to Church</a:t></a:r></a:p>
                <a:p><a:r><a:t>Sunday Service</a:t></a:r></a:p>
            </p:txBody></p:sp></p:spTree></p:cSld></p:sld>"#;
        let text = extract_slide_text_from_xml(xml).unwrap();
        assert_eq!(text, "Welcome to Church\nSunday Service");
    }
}
