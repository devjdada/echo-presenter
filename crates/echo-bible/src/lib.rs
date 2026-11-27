use std::path::Path;
use rusqlite::{params, Connection};
use serde::{Deserialize, Serialize};
use thiserror::Error;

#[derive(Debug, Error)]
pub enum BibleError {
    #[error("SQLite error: {0}")]
    Sqlite(#[from] rusqlite::Error),
    #[error("Invalid reference: {0}")]
    InvalidReference(String),
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BibleTranslation {
    pub id: String,
    pub name: String,
    pub abbreviation: String,
    pub language: String,
    pub license_type: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BibleVerseResult {
    pub translation_id: String,
    pub book_name: String,
    pub book_number: i64,
    pub chapter: i64,
    pub verse: i64,
    pub text: String,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ParsedReference {
    pub book: String,
    pub chapter: i64,
    pub verse_start: Option<i64>,
    pub verse_end: Option<i64>,
}

/// Parse references like `"John 3:16"`, `"1 Cor 13:4-7"`, `"Psalms 23"`.
pub fn parse_reference(input: &str) -> Result<ParsedReference, BibleError> {
    let trimmed = input.trim();
    if trimmed.is_empty() {
        return Err(BibleError::InvalidReference("Empty reference".into()));
    }
    let parts: Vec<&str> = trimmed.rsplitn(2, ' ').collect();
    if parts.len() != 2 {
        return Err(BibleError::InvalidReference(trimmed.into()));
    }
    let chap_verse = parts[0];
    let book = parts[1].trim().to_string();

    if let Some((chap_str, verse_part)) = chap_verse.split_once(':') {
        let chapter: i64 = chap_str
            .parse()
            .map_err(|_| BibleError::InvalidReference(trimmed.into()))?;
        if let Some((v1, v2)) = verse_part.split_once('-') {
            let verse_start: i64 = v1
                .parse()
                .map_err(|_| BibleError::InvalidReference(trimmed.into()))?;
            let verse_end: i64 = v2
                .parse()
                .map_err(|_| BibleError::InvalidReference(trimmed.into()))?;
            Ok(ParsedReference {
                book,
                chapter,
                verse_start: Some(verse_start),
                verse_end: Some(verse_end),
            })
        } else {
            let verse: i64 = verse_part
                .parse()
                .map_err(|_| BibleError::InvalidReference(trimmed.into()))?;
            Ok(ParsedReference {
                book,
                chapter,
                verse_start: Some(verse),
                verse_end: Some(verse),
            })
        }
    } else {
        let chapter: i64 = chap_verse
            .parse()
            .map_err(|_| BibleError::InvalidReference(trimmed.into()))?;
        Ok(ParsedReference {
            book,
            chapter,
            verse_start: None,
            verse_end: None,
        })
    }
}

pub struct BibleEngine {
    conn: Connection,
}

impl BibleEngine {
    pub fn open(path: &Path) -> Result<Self, BibleError> {
        let conn = Connection::open(path)?;
        Ok(Self { conn })
    }

    pub fn open_in_memory() -> Result<Self, BibleError> {
        let conn = Connection::open_in_memory()?;
        let engine = Self { conn };
        engine.init_schema()?;
        Ok(engine)
    }

    pub fn init_schema(&self) -> Result<(), BibleError> {
        self.conn.execute_batch(
            "
            CREATE TABLE IF NOT EXISTS translations (
                id TEXT PRIMARY KEY,
                name TEXT NOT NULL,
                abbreviation TEXT NOT NULL,
                language TEXT NOT NULL,
                license_type TEXT NOT NULL
            );
            CREATE TABLE IF NOT EXISTS books (
                id INTEGER PRIMARY KEY AUTOINCREMENT,
                translation_id TEXT NOT NULL,
                book_number INTEGER NOT NULL,
                name TEXT NOT NULL,
                abbreviation TEXT NOT NULL,
                testament TEXT NOT NULL,
                chapter_count INTEGER NOT NULL
            );
            CREATE TABLE IF NOT EXISTS verses (
                id INTEGER PRIMARY KEY AUTOINCREMENT,
                translation_id TEXT NOT NULL,
                book_number INTEGER NOT NULL,
                book_name TEXT NOT NULL,
                chapter INTEGER NOT NULL,
                verse INTEGER NOT NULL,
                text TEXT NOT NULL
            );
            CREATE INDEX IF NOT EXISTS idx_verses_lookup
                ON verses(translation_id, book_name, chapter, verse);
            CREATE VIRTUAL TABLE IF NOT EXISTS verses_fts USING fts5(
                text,
                content='verses',
                content_rowid='id',
                tokenize='porter unicode61'
            );
            ",
        )?;
        Ok(())
    }

    pub fn list_translations(&self) -> Result<Vec<BibleTranslation>, BibleError> {
        let mut stmt = self
            .conn
            .prepare("SELECT id, name, abbreviation, language, license_type FROM translations ORDER BY id")?;
        let rows = stmt.query_map([], |r| {
            Ok(BibleTranslation {
                id: r.get(0)?,
                name: r.get(1)?,
                abbreviation: r.get(2)?,
                language: r.get(3)?,
                license_type: r.get(4)?,
            })
        })?;
        let mut out = Vec::new();
        for r in rows {
            out.push(r?);
        }
        Ok(out)
    }

    pub fn search_fts(&self, query: &str, translation_id: &str, limit: usize) -> Result<Vec<BibleVerseResult>, BibleError> {
        let mut stmt = self.conn.prepare(
            "SELECT v.translation_id, v.book_name, v.book_number, v.chapter, v.verse, v.text
             FROM verses_fts f
             JOIN verses v ON v.id = f.rowid
             WHERE verses_fts MATCH ?1 AND v.translation_id = ?2
             LIMIT ?3",
        )?;
        let rows = stmt.query_map(params![query, translation_id, limit as i64], |r| {
            Ok(BibleVerseResult {
                translation_id: r.get(0)?,
                book_name: r.get(1)?,
                book_number: r.get(2)?,
                chapter: r.get(3)?,
                verse: r.get(4)?,
                text: r.get(5)?,
            })
        })?;
        let mut out = Vec::new();
        for r in rows {
            out.push(r?);
        }
        Ok(out)
    }

    pub fn lookup_reference(&self, reference: &str, translation_id: &str) -> Result<Vec<BibleVerseResult>, BibleError> {
        let parsed = parse_reference(reference)?;
        let normalized_book = normalize_book_abbrev(&parsed.book);
        let like_book = format!("{normalized_book}%");
        let (v1, v2) = match (parsed.verse_start, parsed.verse_end) {
            (Some(a), Some(b)) => (a, b),
            _ => (1, 999),
        };
        let mut stmt = self.conn.prepare(
            "SELECT translation_id, book_name, book_number, chapter, verse, text
             FROM verses
             WHERE translation_id = ?1 AND book_name LIKE ?2 AND chapter = ?3 AND verse BETWEEN ?4 AND ?5
             ORDER BY verse ASC",
        )?;
        let rows = stmt.query_map(params![translation_id, like_book, parsed.chapter, v1, v2], |r| {
            Ok(BibleVerseResult {
                translation_id: r.get(0)?,
                book_name: r.get(1)?,
                book_number: r.get(2)?,
                chapter: r.get(3)?,
                verse: r.get(4)?,
                text: r.get(5)?,
            })
        })?;
        let mut out = Vec::new();
        for r in rows {
            out.push(r?);
        }
        Ok(out)
    }

    pub fn compare_translations(
        &self,
        reference: &str,
        translation_ids: &[String],
    ) -> Result<Vec<Vec<BibleVerseResult>>, BibleError> {
        let mut results = Vec::new();
        for tid in translation_ids.iter().take(4) {
            results.push(self.lookup_reference(reference, tid)?);
        }
        Ok(results)
    }
}

pub fn normalize_book_abbrev(book: &str) -> &str {
    match book.trim().to_lowercase().as_str() {
        "gen" => "Genesis",
        "ex" | "exod" => "Exodus",
        "ps" | "psa" | "psalm" => "Psalms",
        "prov" | "pr" => "Proverbs",
        "isa" => "Isaiah",
        "matt" | "mt" => "Matthew",
        "mk" => "Mark",
        "lk" => "Luke",
        "jn" | "joh" => "John",
        "rom" => "Romans",
        "1 cor" | "1cor" => "1 Corinthians",
        "2 cor" | "2cor" => "2 Corinthians",
        "gal" => "Galatians",
        "eph" => "Ephesians",
        "phil" | "php" => "Philippians",
        "col" => "Colossians",
        "heb" => "Hebrews",
        "rev" => "Revelation",
        _ => book.trim(),
    }
}

/// Auto-split a list of verses into slide-sized chunks (`max_chars` per slide).
pub fn split_verses_into_slides(verses: &[BibleVerseResult], max_chars: usize) -> Vec<(String, String)> {
    let mut slides = Vec::new();
    let mut current_text = String::new();
    let mut start_v: Option<i64> = None;
    let mut end_v: Option<i64> = None;
    let mut book = String::new();
    let mut chapter = 0;
    let mut tid = String::new();

    for v in verses {
        let piece = format!("{} {}", v.verse, v.text);
        if !current_text.is_empty() && current_text.len() + piece.len() + 1 > max_chars {
            let ref_str = match (start_v, end_v) {
                (Some(a), Some(b)) if a == b => format!("{book} {chapter}:{a} ({tid})"),
                (Some(a), Some(b)) => format!("{book} {chapter}:{a}-{b} ({tid})"),
                _ => format!("{book} {chapter} ({tid})"),
            };
            slides.push((current_text.trim().to_string(), ref_str));
            current_text.clear();
            start_v = None;
        }
        if start_v.is_none() {
            start_v = Some(v.verse);
            book = v.book_name.clone();
            chapter = v.chapter;
            tid = v.translation_id.clone();
        }
        end_v = Some(v.verse);
        if !current_text.is_empty() {
            current_text.push('\n');
        }
        current_text.push_str(&piece);
    }

    if !current_text.is_empty() {
        let ref_str = match (start_v, end_v) {
            (Some(a), Some(b)) if a == b => format!("{book} {chapter}:{a} ({tid})"),
            (Some(a), Some(b)) => format!("{book} {chapter}:{a}-{b} ({tid})"),
            _ => format!("{book} {chapter} ({tid})"),
        };
        slides.push((current_text.trim().to_string(), ref_str));
    }
    slides
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_reference() {
        let r = parse_reference("1 Cor 13:4-7").unwrap();
        assert_eq!(r.book, "1 Cor");
        assert_eq!(r.chapter, 13);
        assert_eq!(r.verse_start, Some(4));
        assert_eq!(r.verse_end, Some(7));
    }

    #[test]
    fn test_bible_fts_and_lookup() {
        let engine = BibleEngine::open_in_memory().unwrap();
        engine.conn.execute(
            "INSERT INTO translations VALUES ('KJV', 'King James Version', 'KJV', 'en', 'public_domain')",
            [],
        ).unwrap();
        engine.conn.execute(
            "INSERT INTO verses (translation_id, book_number, book_name, chapter, verse, text)
             VALUES ('KJV', 43, 'John', 3, 16, 'For God so loved the world, that he gave his only begotten Son')",
            [],
        ).unwrap();
        engine.conn.execute("INSERT INTO verses_fts(verses_fts) VALUES('rebuild')", []).unwrap();

        let ref_res = engine.lookup_reference("John 3:16", "KJV").unwrap();
        assert_eq!(ref_res.len(), 1);
        let fts_res = engine.search_fts("begotten", "KJV", 10).unwrap();
        assert_eq!(fts_res.len(), 1);
    }

    #[test]
    fn test_abbrev_and_split_slides() {
        assert_eq!(normalize_book_abbrev("Rom"), "Romans");
        assert_eq!(normalize_book_abbrev("1 Cor"), "1 Corinthians");
        assert_eq!(normalize_book_abbrev("Ps"), "Psalms");

        let verses = vec![
            BibleVerseResult {
                translation_id: "KJV".into(),
                book_name: "Psalms".into(),
                book_number: 19,
                chapter: 23,
                verse: 1,
                text: "The LORD is my shepherd; I shall not want.".into(),
            },
            BibleVerseResult {
                translation_id: "KJV".into(),
                book_name: "Psalms".into(),
                book_number: 19,
                chapter: 23,
                verse: 2,
                text: "He maketh me to lie down in green pastures: he leadeth me beside the still waters.".into(),
            },
        ];
        let slides = split_verses_into_slides(&verses, 70);
        assert_eq!(slides.len(), 2);
        assert_eq!(slides[0].1, "Psalms 23:1 (KJV)");
    }

    #[test]
    fn test_bundled_public_domain_sqlite_queries() {
        let db_path = std::path::Path::new(env!("CARGO_MANIFEST_DIR"))
            .join("../../resources/bible-modules/bundled_public_domain.sqlite");
        if !db_path.exists() {
            return;
        }
        let engine = BibleEngine::open(&db_path).unwrap();
        let jn = engine.lookup_reference("John 3:16", "KJV").unwrap();
        assert_eq!(jn.len(), 1);
        assert!(jn[0].text.contains("loved the world"));

        let rom = engine.lookup_reference("Romans 8:28", "KJV").unwrap();
        assert_eq!(rom.len(), 1);

        let ps = engine.lookup_reference("Psalms 23:1-3", "KJV").unwrap();
        assert_eq!(ps.len(), 3);
    }
}
