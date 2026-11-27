import { useEffect, useState } from "react";
import { invoke } from "@tauri-apps/api/core";
import { BookOpen, Search, Radio, Bookmark, History } from "lucide-react";
import { useShowStore } from "../stores/useShowStore";

interface BibleTranslation {
  id: string;
  name: string;
  abbreviation: string;
  language: string;
  license_type: string;
}

interface BibleVerseResult {
  translation_id: string;
  book_name: string;
  book_number: number;
  chapter: number;
  verse: number;
  text: string;
}

const isTauri = () => typeof window !== "undefined" && "__TAURI_INTERNALS__" in window;

export function BibleStudio() {
  const [translations, setTranslations] = useState<BibleTranslation[]>([]);
  const [selectedTid, setSelectedTid] = useState("KJV");
  const [query, setQuery] = useState("John 3:16");
  const [results, setResults] = useState<BibleVerseResult[]>([]);
  const [compareIds, setCompareIds] = useState<string[]>(["KJV", "WEB"]);
  const [compareResults, setCompareResults] = useState<BibleVerseResult[][]>([]);
  const [history, setHistory] = useState<string[]>(["John 3:16", "Romans 8:28", "Psalms 23:1-3"]);
  const { triggerSlideLive, setOutputLook, addGeneratedPlaylist } = useShowStore();

  useEffect(() => {
    if (!isTauri()) {
      setTranslations([
        { id: "KJV", name: "King James Version", abbreviation: "KJV", language: "en", license_type: "public_domain" },
        { id: "WEB", name: "World English Bible", abbreviation: "WEB", language: "en", license_type: "public_domain" },
        { id: "ASV", name: "American Standard Version", abbreviation: "ASV", language: "en", license_type: "public_domain" },
      ]);
      return;
    }
    invoke<BibleTranslation[]>("bible_list_translations")
      .then((list) => {
        setTranslations(list);
        const tid = list.length > 0 && !list.some((t) => t.id === selectedTid) ? list[0].id : selectedTid;
        if (tid !== selectedTid) setSelectedTid(tid);
        return invoke<BibleVerseResult[]>("bible_search_reference", {
          reference: "John 3:16",
          translationId: tid,
        });
      })
      .then((initial) => {
        if (initial) setResults(initial);
      })
      .catch(console.error);
  }, []);

  const handleSearch = async (e?: React.FormEvent) => {
    e?.preventDefault();
    const q = query.trim();
    if (!q) return;
    setHistory((prev) => [q, ...prev.filter((x) => x !== q)].slice(0, 12));

    if (!isTauri()) {
      setResults([
        {
          translation_id: selectedTid,
          book_name: "John",
          book_number: 43,
          chapter: 3,
          verse: 16,
          text: "For God so loved the world, that he gave his only begotten Son, that whosoever believeth in him should not perish, but have everlasting life.",
        },
      ]);
      return;
    }

    const isRef = /\d/.test(q);
    if (isRef) {
      const res = await invoke<BibleVerseResult[]>("bible_search_reference", {
        reference: q,
        translationId: selectedTid,
      });
      setResults(res);
      const cmp = await invoke<BibleVerseResult[][]>("bible_compare_translations", {
        reference: q,
        translationIds: compareIds,
      });
      setCompareResults(cmp);
    } else {
      const res = await invoke<BibleVerseResult[]>("bible_search_keyword", {
        query: q,
        translationId: selectedTid,
      });
      setResults(res);
      setCompareResults([]);
    }
  };

  const goLiveVerse = async (v: BibleVerseResult, lowerThird = false) => {
    if (lowerThird) {
      await setOutputLook("ndi-lower-third");
    } else {
      await setOutputLook("main-fullscreen");
    }
    await invoke("trigger_slide", {
      slideId: `bible-${v.translation_id}-${v.book_number}-${v.chapter}-${v.verse}`,
      text: v.text,
      reference: `${v.book_name} ${v.chapter}:${v.verse} (${v.translation_id})`,
      nextSlideText: null,
      stageMessage: null,
    }).catch(() => triggerSlideLive("s-202"));
  };

  return (
    <div className="flex-1 min-h-0 flex bg-zinc-950 text-zinc-100">
      {/* Left: Translations & History */}
      <aside className="w-64 bg-zinc-900/90 border-r border-zinc-800 p-3 flex flex-col gap-4">
        <div>
          <div className="text-[11px] font-semibold uppercase tracking-wider text-zinc-400 mb-2 flex items-center gap-1.5">
            <BookOpen className="w-3.5 h-3.5 text-amber-400" />
            Translations ({translations.length})
          </div>
          <div className="space-y-1">
            {translations.map((t) => (
              <button
                key={t.id}
                onClick={() => setSelectedTid(t.id)}
                className={`w-full flex items-center justify-between px-2.5 py-1.5 rounded text-xs ${
                  selectedTid === t.id
                    ? "bg-amber-600/20 border border-amber-500/40 text-amber-200 font-medium"
                    : "text-zinc-400 hover:bg-zinc-800/60"
                }`}
              >
                <span>{t.abbreviation}</span>
                <span className="text-[10px] text-zinc-500">{t.license_type}</span>
              </button>
            ))}
          </div>
        </div>

        <div className="flex-1 min-h-0 flex flex-col">
          <div className="text-[11px] font-semibold uppercase tracking-wider text-zinc-400 mb-2 flex items-center gap-1.5">
            <History className="w-3.5 h-3.5 text-zinc-400" />
            Recent History
          </div>
          <div className="space-y-1 overflow-y-auto">
            {history.map((h) => (
              <button
                key={h}
                onClick={() => {
                  setQuery(h);
                }}
                className="w-full text-left px-2 py-1 rounded text-xs text-zinc-400 hover:bg-zinc-800 hover:text-zinc-200 truncate"
              >
                {h}
              </button>
            ))}
          </div>
        </div>
      </aside>

      {/* Center: Search & Results */}
      <main className="flex-1 min-w-0 flex flex-col p-4 gap-4 overflow-y-auto">
        <div className="flex items-center gap-2 flex-wrap">
          <span className="text-[11px] text-zinc-400">Compare:</span>
          {translations.map((t) => {
            const active = compareIds.includes(t.id);
            return (
              <button
                key={t.id}
                type="button"
                onClick={() =>
                  setCompareIds((prev) =>
                    active ? prev.filter((x) => x !== t.id) : [...prev, t.id].slice(0, 4)
                  )
                }
                className={`px-2 py-0.5 rounded text-[11px] border ${
                  active
                    ? "bg-indigo-600/20 border-indigo-500 text-indigo-200"
                    : "bg-zinc-900 border-zinc-800 text-zinc-400"
                }`}
              >
                {t.abbreviation}
              </button>
            );
          })}
        </div>
        <form onSubmit={handleSearch} className="flex gap-2">
          <div className="relative flex-1">
            <Search className="w-4 h-4 text-zinc-500 absolute left-3 top-2.5" />
            <input
              type="text"
              value={query}
              onChange={(e) => setQuery(e.target.value)}
              placeholder="Search reference (John 3:16, 1 Cor 13:4-7) or keyword (everlasting life)..."
              className="w-full bg-zinc-900 border border-zinc-800 rounded-md pl-9 pr-3 py-2 text-sm text-zinc-100 placeholder:text-zinc-500 focus:outline-none focus:border-amber-500"
            />
          </div>
          <button
            type="submit"
            className="px-4 py-2 rounded-md bg-amber-600 hover:bg-amber-500 text-xs font-semibold text-white"
          >
            Search
          </button>
          <button
            type="button"
            onClick={async () => {
              if (!isTauri()) return;
              const slides = await invoke<{ id: string; label: string; text: string; reference?: string }[]>(
                "bible_generate_slides",
                { reference: query, translationId: selectedTid }
              );
              if (slides.length > 0) {
                await addGeneratedPlaylist(
                  `Scripture: ${query}`,
                  slides.map((s) => ({
                    id: s.id,
                    label: s.label,
                    group: "scripture",
                    text: s.text,
                    reference: s.reference,
                    bgGradient: "from-amber-950 via-zinc-900 to-zinc-950",
                  }))
                );
              }
            }}
            className="px-3 py-2 rounded-md bg-zinc-800 hover:bg-zinc-700 text-xs font-medium text-zinc-200"
          >
            + Add to Playlist
          </button>
          <button
            type="button"
            onClick={async () => {
              if (!isTauri()) return;
              const path = window.prompt("Enter Bible JSON path:");
              const tid = window.prompt("Translation ID (e.g. NIV, ESV):", "NIV");
              if (path && tid) {
                await invoke("bible_import_user_json", { filePath: path, translationId: tid });
              }
            }}
            className="px-3 py-2 rounded-md bg-zinc-800 hover:bg-zinc-700 text-xs font-medium text-zinc-200"
          >
            Import JSON
          </button>
        </form>

        {/* Results */}
        <div className="space-y-2.5">
          {results.map((v) => (
            <div
              key={`${v.translation_id}-${v.book_number}-${v.chapter}-${v.verse}`}
              className="p-3 rounded-md bg-zinc-900 border border-zinc-800 flex items-start justify-between gap-4"
            >
              <div>
                <div className="text-xs font-semibold text-amber-400 mb-1">
                  {v.book_name} {v.chapter}:{v.verse} ({v.translation_id})
                </div>
                <p className="text-sm text-zinc-100 leading-relaxed">{v.text}</p>
              </div>
              <div className="flex flex-col gap-1.5 shrink-0">
                <button
                  onClick={() => goLiveVerse(v, false)}
                  className="flex items-center gap-1 px-2.5 py-1 rounded bg-red-600 hover:bg-red-500 text-[11px] font-semibold text-white"
                >
                  <Radio className="w-3 h-3" /> Go Live
                </button>
                <button
                  onClick={() => goLiveVerse(v, true)}
                  className="flex items-center gap-1 px-2.5 py-1 rounded bg-zinc-800 hover:bg-zinc-700 text-[11px] font-medium text-zinc-200"
                >
                  Lower-3rd
                </button>
              </div>
            </div>
          ))}
        </div>

        {/* Multi-Translation Side-by-Side */}
        {compareResults.length > 0 && (
          <div className="pt-2 border-t border-zinc-800">
            <div className="text-xs font-semibold text-zinc-400 mb-2 flex items-center gap-2">
              <Bookmark className="w-3.5 h-3.5 text-indigo-400" />
              Side-by-Side Comparison ({compareIds.join(" vs ")})
            </div>
            <div className="grid grid-cols-2 gap-3">
              {compareResults.map((col, idx) => (
                <div key={idx} className="p-3 rounded bg-zinc-900/60 border border-zinc-800 space-y-1.5">
                  <div className="text-[11px] font-bold text-indigo-300">{compareIds[idx]}</div>
                  {col.map((v) => (
                    <p key={v.verse} className="text-xs text-zinc-300">
                      <span className="text-zinc-500 font-mono mr-1">{v.verse}</span>
                      {v.text}
                    </p>
                  ))}
                </div>
              ))}
            </div>
          </div>
        )}
      </main>
    </div>
  );
}
