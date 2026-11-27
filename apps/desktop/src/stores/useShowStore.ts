import { create } from "zustand";
import { invoke } from "@tauri-apps/api/core";
import { listen } from "@tauri-apps/api/event";

export type WorkspaceTab = "Show" | "Edit" | "Bible" | "Reflow";
export type DockTab = "media" | "audio_intel" | "visual_fx" | "echo_ai";
export type SlideGroupKind = "verse" | "chorus" | "bridge" | "scripture" | "quote" | "announcement";
export type PreviewMode = "audience" | "stage";

export interface SlideItem {
  id: string;
  label: string;
  group: SlideGroupKind;
  text: string;
  reference?: string;
  bgGradient: string;
}

export interface PlaylistItem {
  id: string;
  title: string;
  kind: "song" | "sermon" | "scripture" | "announcement";
  slides: SlideItem[];
}

export interface LibraryFolder {
  id: string;
  name: string;
  items: string[];
}

interface LiveStateResponse {
  active_slide_id: string | null;
  active_look_id: string;
  audience_live: boolean;
  cleared_layers: string[];
}

interface ShowState {
  workspaceTab: WorkspaceTab;
  proVisualMode: boolean;
  dockOpen: boolean;
  dockTab: DockTab;
  previewMode: PreviewMode;
  activeLookId: string;
  audienceLive: boolean;
  activePlaylistId: string;
  liveSlideId: string | null;
  previewSlideId: string | null;
  slideScale: number;
  stageMessage: string;
  countdownSeconds: number;
  clearedLayers: string[];
  libraries: LibraryFolder[];
  playlist: PlaylistItem[];
  backendVersion: string;
  previewPngBase64: string | null;
  gpuAdapterName: string | null;

  setWorkspaceTab: (tab: WorkspaceTab) => void;
  toggleProVisualMode: () => void;
  setDockOpen: (open: boolean) => void;
  setDockTab: (tab: DockTab) => void;
  setPreviewMode: (mode: PreviewMode) => void;
  setSlideScale: (scale: number) => void;
  setStageMessage: (msg: string) => void;
  selectPlaylistItem: (id: string) => void;
  selectPreviewSlide: (id: string) => void;
  initApp: () => Promise<void>;
  triggerSlideLive: (slideId: string) => Promise<void>;
  clearLayer: (layerId: string) => Promise<void>;
  setOutputLook: (lookId: string) => Promise<void>;
  openOutputWindow: (mode: "windowed" | "fullscreen", monitorId?: string) => Promise<void>;
  addGeneratedPlaylist: (title: string, slides: SlideItem[]) => Promise<void>;
}

const SAMPLE_PLAYLIST: PlaylistItem[] = [
  {
    id: "pl-1",
    title: "Amazing Grace (My Chains Are Gone)",
    kind: "song",
    slides: [
      {
        id: "s-101",
        label: "Verse 1",
        group: "verse",
        text: "Amazing grace, how sweet the sound\nThat saved a wretch like me",
        bgGradient: "from-slate-900 via-blue-950 to-slate-950",
      },
      {
        id: "s-102",
        label: "Verse 1b",
        group: "verse",
        text: "I once was lost, but now am found\nWas blind, but now I see",
        bgGradient: "from-slate-900 via-blue-950 to-slate-950",
      },
      {
        id: "s-103",
        label: "Chorus",
        group: "chorus",
        text: "My chains are gone, I've been set free\nMy God, my Savior has ransomed me",
        bgGradient: "from-teal-950 via-emerald-950 to-zinc-950",
      },
      {
        id: "s-104",
        label: "Bridge",
        group: "bridge",
        text: "The earth shall soon dissolve like snow\nThe sun forbear to shine",
        bgGradient: "from-purple-950 via-indigo-950 to-zinc-950",
      },
    ],
  },
  {
    id: "pl-2",
    title: "Sunday Sermon: Walking in the Light",
    kind: "sermon",
    slides: [
      {
        id: "s-201",
        label: "Title",
        group: "announcement",
        text: "WALKING IN THE LIGHT\nPastor James — Sunday Morning",
        bgGradient: "from-amber-950 via-stone-900 to-zinc-950",
      },
      {
        id: "s-202",
        label: "John 8:12",
        group: "scripture",
        text: "I am the light of the world. Whoever follows me will never walk in darkness, but will have the light of life.",
        reference: "John 8:12 (WEB)",
        bgGradient: "from-amber-950 via-zinc-900 to-zinc-950",
      },
      {
        id: "s-203",
        label: "Key Quote",
        group: "quote",
        text: "God doesn't call the qualified — He qualifies the called.",
        reference: "Pastor James",
        bgGradient: "from-emerald-950 via-zinc-900 to-zinc-950",
      },
    ],
  },
  {
    id: "pl-3",
    title: "Welcome & Announcements",
    kind: "announcement",
    slides: [
      {
        id: "s-301",
        label: "Welcome",
        group: "announcement",
        text: "Welcome to Sunday Worship\nWe're glad you're here!",
        bgGradient: "from-rose-950 via-zinc-900 to-zinc-950",
      },
    ],
  },
];

const isTauri = () => typeof window !== "undefined" && "__TAURI_INTERNALS__" in window;

export const useShowStore = create<ShowState>((set) => ({
  workspaceTab: "Show",
  proVisualMode: false,
  dockOpen: true,
  dockTab: "audio_intel",
  previewMode: "audience",
  activeLookId: "main-fullscreen",
  audienceLive: false,
  activePlaylistId: "pl-1",
  liveSlideId: null,
  previewSlideId: "s-101",
  slideScale: 100,
  stageMessage: "",
  countdownSeconds: 300,
  clearedLayers: [],
  backendVersion: "0.1.0",
  previewPngBase64: null,
  gpuAdapterName: null,
  libraries: [
    { id: "lib-songs", name: "Worship Songs", items: ["Amazing Grace", "10,000 Reasons", "Way Maker"] },
    { id: "lib-sermons", name: "Sermon Series", items: ["Walking in the Light", "Faith & Hope"] },
    { id: "lib-ann", name: "Announcements", items: ["Sunday Welcome", "Youth Night"] },
  ],
  playlist: SAMPLE_PLAYLIST,

  setWorkspaceTab: (workspaceTab) => set({ workspaceTab }),
  toggleProVisualMode: () =>
    set((s) => ({
      proVisualMode: !s.proVisualMode,
      dockTab: !s.proVisualMode ? "visual_fx" : s.dockTab === "visual_fx" ? "media" : s.dockTab,
    })),
  setDockOpen: (dockOpen) => set({ dockOpen }),
  setDockTab: (dockTab) => set({ dockTab, dockOpen: true }),
  setPreviewMode: (previewMode) => set({ previewMode }),
  setSlideScale: (slideScale) => set({ slideScale }),
  setStageMessage: (stageMessage) => set({ stageMessage }),
  selectPlaylistItem: (activePlaylistId) => set({ activePlaylistId }),
  selectPreviewSlide: (previewSlideId) => set({ previewSlideId }),

  initApp: async () => {
    if (!isTauri()) return;
    try {
      await listen<{ png_base64: string }>("renderer://preview-frame", (ev) => {
        set({ previewPngBase64: ev.payload.png_base64 });
      });
      await listen<{ adapter_name: string }>("renderer://pong", (ev) => {
        set({ gpuAdapterName: ev.payload.adapter_name });
      });
      const info = await invoke<{ version: string; active_look_id: string; audience_live: boolean }>("get_app_info");
      const savedPlaylists = await invoke<string | null>("load_playlists");
      if (savedPlaylists) {
        try {
          const parsed = JSON.parse(savedPlaylists) as PlaylistItem[];
          if (Array.isArray(parsed) && parsed.length > 0) {
            set({ playlist: parsed, activePlaylistId: parsed[0].id });
          }
        } catch {
          // ignore malformed saved json
        }
      }
      set({
        backendVersion: info.version,
        activeLookId: info.active_look_id,
        audienceLive: info.audience_live,
      });
    } catch (e) {
      console.error("get_app_info failed:", e);
    }
  },

  triggerSlideLive: async (slideId) => {
    const state = useShowStore.getState();
    const allSlides = state.playlist.flatMap((p) => p.slides);
    const idx = allSlides.findIndex((s) => s.id === slideId);
    const slide = idx >= 0 ? allSlides[idx] : undefined;
    const nextSlide = idx >= 0 && idx + 1 < allSlides.length ? allSlides[idx + 1] : undefined;

    set({ liveSlideId: slideId, previewSlideId: slideId, audienceLive: true, clearedLayers: [] });
    if (!isTauri()) return;
    try {
      const res = await invoke<LiveStateResponse>("trigger_slide", {
        slideId,
        text: slide?.text ?? null,
        reference: slide?.reference ?? null,
        nextSlideText: nextSlide?.text ?? null,
        stageMessage: state.stageMessage || null,
      });
      set({
        liveSlideId: res.active_slide_id,
        activeLookId: res.active_look_id,
        audienceLive: res.audience_live,
        clearedLayers: res.cleared_layers,
      });
    } catch (e) {
      console.error("trigger_slide failed:", e);
    }
  },

  openOutputWindow: async (mode, monitorId) => {
    if (!isTauri()) return;
    try {
      await invoke("set_output_target", { mode, monitorId: monitorId ?? null });
    } catch (e) {
      console.error("set_output_target failed:", e);
    }
  },

  addGeneratedPlaylist: async (title, slides) => {
    const newItem: PlaylistItem = {
      id: `pl-${Date.now()}`,
      title,
      kind: "sermon",
      slides,
    };
    const nextPlaylist = [...useShowStore.getState().playlist, newItem];
    set({ playlist: nextPlaylist, activePlaylistId: newItem.id });
    if (isTauri()) {
      await invoke("save_playlists", { playlistsJson: JSON.stringify(nextPlaylist) }).catch(
        console.error
      );
    }
  },

  clearLayer: async (layerId) => {
    if (layerId === "all") {
      set({ liveSlideId: null, audienceLive: false, clearedLayers: [] });
    } else if (layerId === "slide") {
      set((s) => ({ liveSlideId: null, clearedLayers: [...s.clearedLayers, "slide"] }));
    } else {
      set((s) => ({ clearedLayers: [...s.clearedLayers, layerId] }));
    }
    if (!isTauri()) return;
    try {
      const res = await invoke<LiveStateResponse>("clear_layer", { layerId });
      set({
        liveSlideId: res.active_slide_id,
        activeLookId: res.active_look_id,
        audienceLive: res.audience_live,
        clearedLayers: res.cleared_layers,
      });
    } catch (e) {
      console.error("clear_layer failed:", e);
    }
  },

  setOutputLook: async (lookId) => {
    set({ activeLookId: lookId });
    if (!isTauri()) return;
    try {
      const res = await invoke<LiveStateResponse>("set_output_look", { lookId });
      set({ activeLookId: res.active_look_id });
    } catch (e) {
      console.error("set_output_look failed:", e);
    }
  },
}));
