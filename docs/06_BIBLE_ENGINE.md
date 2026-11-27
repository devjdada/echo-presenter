# 06 — Bible Engine Deep Dive

---

## Bible Module Format

EchoPresenter uses a custom SQLite-based Bible storage format, with importers for common formats.

### Schema

```sql
-- Bible translation metadata
CREATE TABLE translations (
    id TEXT PRIMARY KEY,           -- e.g., 'KJV', 'ESV', 'WEB'
    name TEXT NOT NULL,            -- 'King James Version'
    abbreviation TEXT NOT NULL,
    language TEXT NOT NULL,        -- 'en', 'es', 'fr'
    license_type TEXT NOT NULL,    -- 'public_domain', 'licensed', 'user_imported'
    copyright_text TEXT,
    version INTEGER DEFAULT 1
);

-- Books (66 canonical + deuterocanonical support)
CREATE TABLE books (
    id INTEGER PRIMARY KEY,
    translation_id TEXT NOT NULL,
    book_number INTEGER NOT NULL,  -- 1-66 (Protestant), extended for Catholic
    name TEXT NOT NULL,            -- 'Genesis', 'Revelation'
    abbreviation TEXT NOT NULL,    -- 'Gen', 'Rev'
    testament TEXT NOT NULL,       -- 'OT', 'NT'
    chapter_count INTEGER NOT NULL,
    FOREIGN KEY (translation_id) REFERENCES translations(id)
);

-- Verses (the actual text)
CREATE TABLE verses (
    id INTEGER PRIMARY KEY,
    translation_id TEXT NOT NULL,
    book_number INTEGER NOT NULL,
    chapter INTEGER NOT NULL,
    verse INTEGER NOT NULL,
    text TEXT NOT NULL,
    FOREIGN KEY (translation_id) REFERENCES translations(id)
);

-- Full-text search index
CREATE VIRTUAL TABLE verses_fts USING fts5(
    text,
    content='verses',
    content_rowid='id',
    tokenize='porter unicode61'
);

-- Verse embeddings for semantic search
CREATE TABLE verse_embeddings (
    verse_id INTEGER PRIMARY KEY,
    translation_id TEXT NOT NULL,
    embedding BLOB NOT NULL,       -- float32 vector (384 dims for MiniLM)
    FOREIGN KEY (verse_id) REFERENCES verses(id)
);
```

---

## Split Database Strategy

| Database | Contents | Distribution |
|---|---|---|
| `bundled_public_domain.sqlite` | KJV, ASV, WEB, YLT, DBY | ✅ Shipped in installer, committed to repo |
| `user_imported.sqlite` | All user-imported or licensed translations | ❌ Created at runtime, `.gitignore`d, excluded from builds |

> [!IMPORTANT]
> The split ensures we never accidentally redistribute copyrighted Bible text. `bundled_public_domain.sqlite` is built from verified public-domain sources during the build process. `user_imported.sqlite` lives in the user's app data directory only.

---

## Supported Import Formats

| Format | Source | Priority |
|---|---|---|
| **OSIS XML** | Standard open format | Phase 3 |
| **USFM/USX** | United Bible Societies | Phase 3 |
| **SWORD modules** | CrossWire Bible Society | Phase 3 (most translations available here) |
| **Zefania XML** | Open format | Phase 4 |
| **Custom JSON/CSV** | User import | Phase 3 |

---

## Public Domain Translations (Ship with EchoPresenter)

| Translation | Abbreviation | Language | Status |
|---|---|---|---|
| King James Version | KJV | English | ✅ Public domain |
| American Standard Version | ASV | English | ✅ Public domain |
| World English Bible | WEB | English | ✅ Public domain |
| Young's Literal Translation | YLT | English | ✅ Public domain |
| Darby | DBY | English | ✅ Public domain |

---

## Licensed Translations (User must provide/accept license)

| Translation | Licensing |
|---|---|
| NIV | Biblica — requires commercial license |
| ESV | Crossway — free for apps under 500 verses, then paid |
| NASB | Lockman Foundation — application license required |
| NLT | Tyndale — commercial license required |
| NKJV | Thomas Nelson — commercial license required |

> [!IMPORTANT]
> EchoPresenter ships with public domain translations only in `bundled_public_domain.sqlite`. Licensed translations are imported by the user into `user_imported.sqlite` at runtime. We do NOT redistribute copyrighted Bible text.
