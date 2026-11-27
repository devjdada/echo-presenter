import { useShowStore, SlideGroupKind } from "../stores/useShowStore";
import logoSymbol from "../assets/logo-symbol.svg";

const GROUP_BADGE: Record<SlideGroupKind, { bg: string; label: string }> = {
  verse: { bg: "bg-blue-600", label: "Verse" },
  chorus: { bg: "bg-teal-600", label: "Chorus" },
  bridge: { bg: "bg-purple-600", label: "Bridge" },
  scripture: { bg: "bg-amber-600", label: "Scripture" },
  quote: { bg: "bg-emerald-600", label: "Quote" },
  announcement: { bg: "bg-rose-600", label: "Announcement" },
};

export function SlideGrid() {
  const {
    playlist,
    activePlaylistId,
    liveSlideId,
    previewSlideId,
    slideScale,
    setSlideScale,
    triggerSlideLive,
  } = useShowStore();

  const currentItem = playlist.find((p) => p.id === activePlaylistId) ?? playlist[0];
  const cardWidth = Math.round(200 * (slideScale / 100));

  return (
    <section className="flex-1 min-w-0 bg-zinc-950 flex flex-col">
      {/* Slide Grid Header */}
      <div className="h-9 px-4 border-b border-zinc-800 bg-zinc-900/50 flex items-center justify-between shrink-0">
        <div className="flex items-center gap-2">
          <span className="text-xs font-semibold text-zinc-200">{currentItem?.title}</span>
          <span className="text-[11px] text-zinc-500">
            ({currentItem?.slides.length ?? 0} slides)
          </span>
        </div>

        <div className="flex items-center gap-2">
          <span className="text-[11px] text-zinc-500">Zoom</span>
          <input
            type="range"
            min={75}
            max={150}
            value={slideScale}
            onChange={(e) => setSlideScale(Number(e.target.value))}
            className="w-24 accent-indigo-500 h-1 bg-zinc-800 rounded-lg cursor-pointer"
          />
        </div>
      </div>

      {/* Color-Coded Slide Grid */}
      <div className="flex-1 overflow-y-auto p-4">
        {(!currentItem || currentItem.slides.length === 0) ? (
          <div className="h-full flex flex-col items-center justify-center text-center gap-3 opacity-60">
            <img src={logoSymbol} className="w-16 h-16 select-none" alt="EchoPresenter" />
            <p className="text-xs text-zinc-400">
              Select a presentation or type in Echo AI to begin
            </p>
          </div>
        ) : (
        <div
          className="grid gap-3"
          style={{
            gridTemplateColumns: `repeat(auto-fill, minmax(${cardWidth}px, 1fr))`,
          }}
        >
          {currentItem?.slides.map((slide, idx) => {
            const isLive = slide.id === liveSlideId;
            const isPreview = slide.id === previewSlideId && !isLive;
            const badge = GROUP_BADGE[slide.group];

            return (
              <button
                key={slide.id}
                onClick={() => triggerSlideLive(slide.id)}
                className={`group text-left rounded-md overflow-hidden bg-zinc-900 transition-all border-2 ${
                  isLive
                    ? "border-red-500 shadow-[0_0_15px_rgba(239,68,68,0.35)]"
                    : isPreview
                    ? "border-emerald-500/80"
                    : "border-zinc-800 hover:border-zinc-600"
                }`}
              >
                {/* 16:9 Slide Thumbnail */}
                <div
                  className={`aspect-video w-full bg-gradient-to-br ${slide.bgGradient} p-3 flex flex-col items-center justify-center text-center relative`}
                >
                  <p className="text-[11px] leading-snug font-medium text-white/95 whitespace-pre-line line-clamp-4">
                    {slide.text}
                  </p>
                  {slide.reference && (
                    <span className="mt-1.5 text-[9px] font-semibold uppercase tracking-wider text-amber-300/90">
                      {slide.reference}
                    </span>
                  )}
                  {isLive && (
                    <span className="absolute top-1.5 right-1.5 px-1.5 py-0.5 rounded bg-red-600 text-[9px] font-bold uppercase tracking-wider text-white">
                      LIVE
                    </span>
                  )}
                </div>

                {/* Color-Coded Group Footer */}
                <div className="px-2 py-1 bg-zinc-900 flex items-center justify-between border-t border-zinc-800/80">
                  <div className="flex items-center gap-1.5">
                    <span className={`w-2 h-2 rounded-full ${badge.bg}`} />
                    <span className="text-[11px] font-medium text-zinc-200">{slide.label}</span>
                  </div>
                  <span className="text-[10px] font-mono text-zinc-500">{idx + 1}</span>
                </div>
              </button>
            );
          })}
        </div>
        )}
      </div>
    </section>
  );
}
