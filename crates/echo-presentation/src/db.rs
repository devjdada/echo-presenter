use std::path::Path;
use rusqlite::{params, Connection};
use serde::{Deserialize, Serialize};
use thiserror::Error;

#[derive(Debug, Error)]
pub enum StorageError {
    #[error("SQLite error: {0}")]
    Sqlite(#[from] rusqlite::Error),
    #[error("IO error: {0}")]
    Io(#[from] std::io::Error),
    #[error("JSON error: {0}")]
    Json(#[from] serde_json::Error),
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MediaItemRecord {
    pub id: String,
    pub name: String,
    pub file_path: String,
    pub kind: String,
    pub thumbnail_path: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ProjectBundle {
    pub version: u32,
    pub title: String,
    pub active_look_id: String,
    pub playlists_json: String,
    pub media: Vec<MediaItemRecord>,
}

pub struct ProjectDb {
    conn: Connection,
}

impl ProjectDb {
    pub fn open(path: &Path) -> Result<Self, StorageError> {
        if let Some(parent) = path.parent() {
            std::fs::create_dir_all(parent)?;
        }
        let conn = Connection::open(path)?;
        let db = Self { conn };
        db.init_schema()?;
        Ok(db)
    }

    pub fn open_in_memory() -> Result<Self, StorageError> {
        let conn = Connection::open_in_memory()?;
        let db = Self { conn };
        db.init_schema()?;
        Ok(db)
    }

    fn init_schema(&self) -> Result<(), StorageError> {
        self.conn.execute_batch(
            "
            CREATE TABLE IF NOT EXISTS projects (
                id TEXT PRIMARY KEY,
                title TEXT NOT NULL,
                active_look_id TEXT NOT NULL,
                playlists_json TEXT NOT NULL,
                updated_at TEXT DEFAULT CURRENT_TIMESTAMP
            );
            CREATE TABLE IF NOT EXISTS media_items (
                id TEXT PRIMARY KEY,
                name TEXT NOT NULL,
                file_path TEXT NOT NULL,
                kind TEXT NOT NULL,
                thumbnail_path TEXT,
                created_at TEXT DEFAULT CURRENT_TIMESTAMP
            );
            CREATE TABLE IF NOT EXISTS themes (
                id TEXT PRIMARY KEY,
                name TEXT NOT NULL,
                scene_template_json TEXT NOT NULL
            );
            CREATE TABLE IF NOT EXISTS app_settings (
                id TEXT PRIMARY KEY,
                settings_json TEXT NOT NULL
            );
            ",
        )?;
        Ok(())
    }

    pub fn upsert_media(&self, item: &MediaItemRecord) -> Result<(), StorageError> {
        self.conn.execute(
            "INSERT INTO media_items (id, name, file_path, kind, thumbnail_path)
             VALUES (?1, ?2, ?3, ?4, ?5)
             ON CONFLICT(id) DO UPDATE SET name=excluded.name, file_path=excluded.file_path",
            params![item.id, item.name, item.file_path, item.kind, item.thumbnail_path],
        )?;
        Ok(())
    }

    pub fn list_media(&self) -> Result<Vec<MediaItemRecord>, StorageError> {
        let mut stmt = self
            .conn
            .prepare("SELECT id, name, file_path, kind, thumbnail_path FROM media_items ORDER BY created_at DESC")?;
        let rows = stmt.query_map([], |row| {
            Ok(MediaItemRecord {
                id: row.get(0)?,
                name: row.get(1)?,
                file_path: row.get(2)?,
                kind: row.get(3)?,
                thumbnail_path: row.get(4)?,
            })
        })?;
        let mut out = Vec::new();
        for r in rows {
            out.push(r?);
        }
        Ok(out)
    }

    pub fn save_project_bundle(&self, path: &Path, bundle: &ProjectBundle) -> Result<(), StorageError> {
        let json = serde_json::to_string_pretty(bundle)?;
        std::fs::write(path, json)?;
        Ok(())
    }

    pub fn load_project_bundle(&self, path: &Path) -> Result<ProjectBundle, StorageError> {
        let raw = std::fs::read_to_string(path)?;
        let bundle: ProjectBundle = serde_json::from_str(&raw)?;
        Ok(bundle)
    }

    pub fn save_playlists_json(&self, json: &str) -> Result<(), StorageError> {
        self.conn.execute(
            "INSERT INTO projects (id, title, active_look_id, playlists_json)
             VALUES ('default', 'Default Service', 'main-fullscreen', ?1)
             ON CONFLICT(id) DO UPDATE SET playlists_json=excluded.playlists_json, updated_at=CURRENT_TIMESTAMP",
            params![json],
        )?;
        Ok(())
    }

    pub fn load_playlists_json(&self) -> Result<Option<String>, StorageError> {
        let mut stmt = self
            .conn
            .prepare("SELECT playlists_json FROM projects WHERE id = 'default'")?;
        let mut rows = stmt.query([])?;
        if let Some(row) = rows.next()? {
            Ok(Some(row.get(0)?))
        } else {
            Ok(None)
        }
    }

    pub fn save_settings_json(&self, json: &str) -> Result<(), StorageError> {
        self.conn.execute(
            "INSERT INTO app_settings (id, settings_json) VALUES ('default', ?1)
             ON CONFLICT(id) DO UPDATE SET settings_json=excluded.settings_json",
            params![json],
        )?;
        Ok(())
    }

    pub fn load_settings_json(&self) -> Result<Option<String>, StorageError> {
        let mut stmt = self
            .conn
            .prepare("SELECT settings_json FROM app_settings WHERE id = 'default'")?;
        let mut rows = stmt.query([])?;
        if let Some(row) = rows.next()? {
            Ok(Some(row.get(0)?))
        } else {
            Ok(None)
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_sqlite_media_and_project_roundtrip() {
        let db = ProjectDb::open_in_memory().unwrap();
        db.upsert_media(&MediaItemRecord {
            id: "m-1".into(),
            name: "Background.png".into(),
            file_path: "C:/media/bg.png".into(),
            kind: "image".into(),
            thumbnail_path: None,
        })
        .unwrap();

        let items = db.list_media().unwrap();
        assert_eq!(items.len(), 1);
        assert_eq!(items[0].name, "Background.png");

        db.save_settings_json(r#"{"osc_port":7000}"#).unwrap();
        let loaded = db.load_settings_json().unwrap().unwrap();
        assert!(loaded.contains("7000"));
    }
}
