use aho_corasick::AhoCorasick;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DetectedReference {
    pub matched_text: String,
    pub book: String,
    pub confidence: f32,
}

pub struct ScriptureDetector {
    automaton: AhoCorasick,
    books: Vec<String>,
}

impl ScriptureDetector {
    pub fn new(book_names: Vec<String>) -> Self {
        let patterns: Vec<String> = book_names.iter().map(|b| b.to_lowercase()).collect();
        let automaton = AhoCorasick::new(&patterns).expect("valid patterns");
        Self {
            automaton,
            books: book_names,
        }
    }

    pub fn detect(&self, transcript: &str) -> Vec<DetectedReference> {
        let lower = transcript.to_lowercase();
        let mut hits = Vec::new();
        for mat in self.automaton.find_iter(&lower) {
            let book = self.books[mat.pattern().as_usize()].clone();
            hits.push(DetectedReference {
                matched_text: transcript[mat.start()..mat.end()].to_string(),
                book,
                confidence: 0.92,
            });
        }
        hits
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_aho_corasick_detection() {
        let detector = ScriptureDetector::new(vec!["John".into(), "Romans".into()]);
        let hits = detector.detect("Let's turn to Romans chapter 8 today.");
        assert_eq!(hits.len(), 1);
        assert_eq!(hits[0].book, "Romans");
    }
}
