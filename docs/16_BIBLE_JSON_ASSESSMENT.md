# 16 — Bible JSON Assessment

Assessment of the existing wchapel Bible JSON data for reuse in EchoPresenter.

---

## Source Data

| Item | Details |
|---|---|
| **Main file** | `C:\Users\USER\Work\bible-versions\bible_data.json` |
| **Size** | ~177 MB (169 MB raw) |
| **Mapping file** | `book_name_mapping.json` (~106 KB) — translation-specific book name lookups for 50+ languages |
| **Origin** | Scraped Bible data used by the wchapel backend |

---

## JSON Schema

```json
{
  "NEW INTERNATIONAL VERSION": {
    "Genesis": {
      "1": {
        "1": "In the beginning God created the heavens and the earth.",
        "2": "Now the earth was formless and empty…"
      }
    }
  },
  "KJV": { ... }
}
```

- **Top-level keys** = translation identifiers (string names)
- Each translation → **book name** (English) → **chapter number** (string) → **verse number** (string) → **verse text**
- No explicit metadata (license, language code, version id) beyond the key name

---

## How wchapel-app Uses Bible Data

The `wchapel-app` project (React Native / Expo) consumes Bible data via a **backend API**, not from local JSON:

- [`api/bible.service.ts`](file:///c:/Users/USER/Work/wchapel-app/api/bible.service.ts) defines TypeScript interfaces: `BibleBook`, `BibleChapter`, `BibleVerse`, `BibleVersion`
- API calls: `GET /bible/books`, `GET /bible/books/{id}/chapters/{num}/verses?version=KJV`
- The backend (Laravel) imports `bible_data.json` into a relational database and serves it via REST

---

## Fit for EchoPresenter

| Requirement | Current JSON | Verdict |
|---|---|---|
| Hierarchical book–chapter–verse structure | ✅ Direct mapping | ✅ Good |
| Multiple translations | ✅ Separate top-level objects | ✅ Good |
| License / provenance metadata | ❌ Absent | ⚠️ Needs addition |
| Language / ISO code | ❌ Not present | ⚠️ Needs addition |
| Multilingual book names | ✅ `book_name_mapping.json` has 50+ languages | ✅ Excellent |
| Size handling | 177 MB (large but manageable) | ⚠️ Chunked import into SQLite |

---

## Split Database Migration Strategy

> [!IMPORTANT]
> EchoPresenter uses a **split database** strategy to enforce the licensing boundary:

### `bundled_public_domain.sqlite` — Shipped in Installer

| Translation | Abbreviation | License | Included |
|---|---|---|---|
| King James Version | KJV | Public domain | ✅ |
| American Standard Version | ASV | Public domain | ✅ |
| World English Bible | WEB | Public domain | ✅ |
| Young's Literal Translation | YLT | Public domain | ✅ |
| Darby | DBY | Public domain | ✅ |

- **Built during CI/CD** from verified public-domain JSON extracts
- Committed to the repository under `resources/bible-modules/`
- Distributed with every installer

### `user_imported.sqlite` — Runtime Only

| Translation | Abbreviation | License | Included |
|---|---|---|---|
| NIV | NIV | Biblica commercial | ❌ User imports |
| ESV | ESV | Crossway application license | ❌ User imports |
| NASB | NASB | Lockman Foundation | ❌ User imports |
| NLT | NLT | Tyndale commercial | ❌ User imports |
| NKJV | NKJV | Thomas Nelson commercial | ❌ User imports |
| *(any other)* | — | User-provided | ❌ User imports |

- Created at runtime in the user's app data directory
- **Strictly `.gitignore`d** — never committed to repo
- **Excluded from distributable builds** — never packaged in installer
- User imports via Bible Module Manager in Bible Studio

---

## Migration Script Requirements

Write a one-off build-time script that:

1. Streams `bible_data.json` using an incremental parser
2. **Filters** to only the 5 public-domain translations (KJV, ASV, WEB, YLT, DBY)
3. Inserts rows into `bundled_public_domain.sqlite` following the EchoPresenter schema
4. Creates FTS5 virtual table for full-text search
5. Creates index on `(translation_id, book_number, chapter, verse)`
6. Feeds `book_name_mapping.json` into the Aho-Corasick pattern file for multilingual scripture detection

A separate runtime import flow handles `user_imported.sqlite`:
1. User selects a JSON, SWORD module, or OSIS XML file
2. EchoPresenter parses and inserts into `user_imported.sqlite`
3. FTS5 index rebuilt for the new translation

---

## Conclusion

> **The existing JSON is good enough** for the core hierarchical data required by EchoPresenter. With the split-database strategy (`bundled_public_domain.sqlite` + `user_imported.sqlite`), licensing boundaries are enforced at the architecture level. The `book_name_mapping.json` is a valuable asset for multilingual Aho-Corasick scripture detection.
