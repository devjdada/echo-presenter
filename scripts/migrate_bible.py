import json
import os
import sqlite3

src = r"C:\Users\USER\Work\bible-versions\bible_data.json"
dst = r"c:\Users\USER\Work\echoShow\resources\bible-modules\bundled_public_domain.sqlite"
os.makedirs(os.path.dirname(dst), exist_ok=True)
if os.path.exists(dst):
    os.remove(dst)

PUBLIC_DOMAIN_MAP = {
    "KING JAMES BIBLE": ("KJV", "King James Version"),
    "KING JAMES VERSION": ("KJV", "King James Version"),
    "KJV": ("KJV", "King James Version"),
    "AMERICAN STANDARD VERSION": ("ASV", "American Standard Version"),
    "ASV": ("ASV", "American Standard Version"),
    "WORLD ENGLISH BIBLE": ("WEB", "World English Bible"),
    "WEB": ("WEB", "World English Bible"),
    "YOUNG'S LITERAL TRANSLATION": ("YLT", "Young's Literal Translation"),
    "YLT": ("YLT", "Young's Literal Translation"),
    "DARBY BIBLE": ("DBY", "Darby Bible"),
    "DBY": ("DBY", "Darby Bible"),
    "WEBSTER'S BIBLE TRANSLATION": ("WBT", "Webster's Bible"),
    "WBT": ("WBT", "Webster's Bible"),
}

with open(src, "r", encoding="utf-8") as f:
    data = json.load(f)

conn = sqlite3.connect(dst)
cur = conn.cursor()
cur.executescript(
    """
CREATE TABLE translations (id TEXT PRIMARY KEY, name TEXT NOT NULL, abbreviation TEXT NOT NULL, language TEXT NOT NULL, license_type TEXT NOT NULL);
CREATE TABLE books (id INTEGER PRIMARY KEY AUTOINCREMENT, translation_id TEXT NOT NULL, book_number INTEGER NOT NULL, name TEXT NOT NULL, abbreviation TEXT NOT NULL, testament TEXT NOT NULL, chapter_count INTEGER NOT NULL);
CREATE TABLE verses (id INTEGER PRIMARY KEY AUTOINCREMENT, translation_id TEXT NOT NULL, book_number INTEGER NOT NULL, book_name TEXT NOT NULL, chapter INTEGER NOT NULL, verse INTEGER NOT NULL, text TEXT NOT NULL);
CREATE INDEX idx_verses_lookup ON verses(translation_id, book_name, chapter, verse);
CREATE VIRTUAL TABLE verses_fts USING fts5(text, content='verses', content_rowid='id', tokenize='porter unicode61');
"""
)

total_verses = 0
imported_tids = []
for key, books in data.items():
    norm = key.strip().upper()
    if norm not in PUBLIC_DOMAIN_MAP:
        continue
    tid, tname = PUBLIC_DOMAIN_MAP[norm]
    if tid in imported_tids:
        continue
    imported_tids.append(tid)
    cur.execute(
        "INSERT INTO translations VALUES (?, ?, ?, ?, ?)",
        (tid, tname, tid, "en", "public_domain"),
    )
    batch = []
    for b_idx, (b_name, chapters) in enumerate(books.items(), start=1):
        testament = "OT" if b_idx <= 39 else "NT"
        cur.execute(
            "INSERT INTO books (translation_id, book_number, name, abbreviation, testament, chapter_count) VALUES (?, ?, ?, ?, ?, ?)",
            (tid, b_idx, b_name, b_name[:3], testament, len(chapters)),
        )
        for ch_str, verses in chapters.items():
            ch_num = int(ch_str)
            for v_str, v_text in verses.items():
                v_num = int(v_str)
                batch.append((tid, b_idx, b_name, ch_num, v_num, str(v_text)))
                if len(batch) >= 10000:
                    cur.executemany(
                        "INSERT INTO verses (translation_id, book_number, book_name, chapter, verse, text) VALUES (?, ?, ?, ?, ?, ?)",
                        batch,
                    )
                    total_verses += len(batch)
                    batch.clear()
    if batch:
        cur.executemany(
            "INSERT INTO verses (translation_id, book_number, book_name, chapter, verse, text) VALUES (?, ?, ?, ?, ?, ?)",
            batch,
        )
        total_verses += len(batch)

cur.execute("INSERT INTO verses_fts(verses_fts) VALUES('rebuild')")
conn.commit()
conn.close()
print(f"BIBLE_MIGRATION_OK translations={imported_tids} total_verses={total_verses}")
