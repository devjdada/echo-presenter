use std::io::Read;
use std::path::Path;
use quick_xml::events::Event;
use quick_xml::Reader;
use zip::ZipArchive;

/// Extract plain text from a `.docx` file (`word/document.xml`).
pub fn extract_docx_text(path: &Path) -> Result<String, String> {
    let file = std::fs::File::open(path).map_err(|e| e.to_string())?;
    let mut archive = ZipArchive::new(file).map_err(|e| e.to_string())?;
    let mut entry = archive.by_name("word/document.xml").map_err(|e| e.to_string())?;
    let mut xml = String::new();
    entry.read_to_string(&mut xml).map_err(|e| e.to_string())?;
    extract_docx_text_from_xml(&xml)
}

pub fn extract_docx_text_from_xml(xml: &str) -> Result<String, String> {
    let mut reader = Reader::from_str(xml);
    let mut buf = Vec::new();
    let mut in_text = false;
    let mut paragraphs = Vec::new();
    let mut current = String::new();

    loop {
        match reader.read_event_into(&mut buf) {
            Ok(Event::Start(e)) if e.name().as_ref().ends_with(b":t") || e.name().as_ref() == b"t" => {
                in_text = true;
            }
            Ok(Event::End(e)) if e.name().as_ref().ends_with(b":t") || e.name().as_ref() == b"t" => {
                in_text = false;
            }
            Ok(Event::End(e)) if e.name().as_ref().ends_with(b":p") || e.name().as_ref() == b"p" => {
                if !current.trim().is_empty() {
                    paragraphs.push(current.trim().to_string());
                    current.clear();
                }
            }
            Ok(Event::Text(e)) if in_text => {
                if let Ok(t) = e.unescape() {
                    current.push_str(&t);
                }
            }
            Ok(Event::Eof) => break,
            Err(e) => return Err(e.to_string()),
            _ => {}
        }
        buf.clear();
    }
    if !current.trim().is_empty() {
        paragraphs.push(current.trim().to_string());
    }
    Ok(paragraphs.join("\n\n"))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_extract_docx_xml() {
        let xml = r#"<w:document xmlns:w="http://schemas.openxmlformats.org/wordprocessingml/2006/main">
            <w:body><w:p><w:r><w:t>Sermon on Faith</w:t></w:r></w:p>
            <w:p><w:r><w:t>Hebrews 11:1</w:t></w:r></w:p></w:body></w:document>"#;
        let out = extract_docx_text_from_xml(xml).unwrap();
        assert_eq!(out, "Sermon on Faith\n\nHebrews 11:1");
    }
}
