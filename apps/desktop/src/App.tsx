import { useEffect, useState } from "react";
import { TopBar } from "./components/TopBar";
import { LeftSidebar } from "./components/LeftSidebar";
import { SlideGrid } from "./components/SlideGrid";
import { RightProductionPanel } from "./components/RightProductionPanel";
import { BottomDock } from "./components/BottomDock";
import { QuickSearchModal } from "./components/QuickSearchModal";
import { BibleStudio } from "./pages/BibleStudio";
import { SlideEditor } from "./pages/SlideEditor";
import { ReflowEditor } from "./pages/ReflowEditor";
import { useShowStore } from "./stores/useShowStore";

export default function App() {
  const { initApp, workspaceTab, playlist, activePlaylistId, liveSlideId, triggerSlideLive, clearLayer } =
    useShowStore();
  const [quickSearchOpen, setQuickSearchOpen] = useState(false);

  useEffect(() => {
    initApp();
  }, [initApp]);

  useEffect(() => {
    const onKeyDown = (e: KeyboardEvent) => {
      if ((e.ctrlKey || e.metaKey) && e.key.toLowerCase() === "k") {
        e.preventDefault();
        setQuickSearchOpen((v) => !v);
        return;
      }
      if (e.target instanceof HTMLInputElement || e.target instanceof HTMLTextAreaElement) {
        return;
      }
      if (e.key === "Escape") {
        clearLayer("all");
      } else if (e.key === "ArrowRight" || e.key === " ") {
        e.preventDefault();
        const item = playlist.find((p) => p.id === activePlaylistId) ?? playlist[0];
        if (!item || item.slides.length === 0) return;
        const idx = item.slides.findIndex((s) => s.id === liveSlideId);
        const next = idx < 0 ? item.slides[0] : item.slides[Math.min(idx + 1, item.slides.length - 1)];
        triggerSlideLive(next.id);
      } else if (e.key === "ArrowLeft") {
        e.preventDefault();
        const item = playlist.find((p) => p.id === activePlaylistId) ?? playlist[0];
        if (!item || item.slides.length === 0) return;
        const idx = item.slides.findIndex((s) => s.id === liveSlideId);
        const prev = idx <= 0 ? item.slides[0] : item.slides[idx - 1];
        triggerSlideLive(prev.id);
      }
    };
    window.addEventListener("keydown", onKeyDown);
    return () => window.removeEventListener("keydown", onKeyDown);
  }, [playlist, activePlaylistId, liveSlideId, triggerSlideLive, clearLayer]);

  return (
    <div className="h-screen w-screen flex flex-col bg-zinc-950 text-zinc-100 overflow-hidden">
      <TopBar />
      <main className="flex-1 min-h-0 flex overflow-hidden">
        {workspaceTab === "Bible" ? (
          <BibleStudio />
        ) : workspaceTab === "Edit" ? (
          <SlideEditor />
        ) : workspaceTab === "Reflow" ? (
          <ReflowEditor />
        ) : (
          <>
            <LeftSidebar />
            <SlideGrid />
            <RightProductionPanel />
          </>
        )}
      </main>
      <BottomDock />
      <QuickSearchModal open={quickSearchOpen} onClose={() => setQuickSearchOpen(false)} />
    </div>
  );
}
