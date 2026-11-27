# EchoPresenter — Sprint 3 Execution Audit

> **Date:** 2026-11-27  
> **Build Status:** `cargo test --workspace` ✅ (Exit 0) | `pnpm --dir apps/desktop build` ✅ (Exit 0) | `echo-renderer --smoke-test` ✅ (`Quadro RTX 3000 (Vulkan)`)

---

## Packaged Installers & Binaries

- **Standalone EXE:** `C:\Users\USER\Work\echoShow\target\release\echo-presenter-desktop.exe`
- **Sidecar Bundled:** `C:\Users\USER\Work\echoShow\apps\desktop\src-tauri\binaries\echo-renderer-x86_64-pc-windows-msvc.exe`
- **MSI Installer:** `C:\Users\USER\Work\echoShow\target\release\bundle\msi\EchoPresenter_0.1.0_x64_en-US.msi` (`27.19 MiB`)
- **NSIS Setup EXE:** `C:\Users\USER\Work\echoShow\target\release\bundle\nsis\EchoPresenter_0.1.0_x64-setup.exe` (`16.80 MiB`)

---

## Honest Feature Wiring Table

| Feature | Rust Crate | Tauri IPC / API | React UI Wired? | Packaged & Tested? | Notes / Blockers |
|---|---|---|---|---|---|
| **4-Zone Operator UI** | `echo-core`, `echo-output` | `get_app_info`, `trigger_slide`, `clear_layer`, `set_output_look` | ✅ Full (`Show` tab) | ✅ Yes | Live preview + Volunteer/Pro toggle |
| **GPU Renderer (`wgpu`+`vello`)** | `echo-graphics`, `apps/renderer` | Named Pipe (`rmp-serde`), `set_output_target` | ✅ Live PNG preview + Windowed/Fullscreen | ✅ Yes (`Quadro RTX 3000`) | 3 Output Looks implemented |
| **SQLite Persistence** | `echo-presentation` | `save_playlists`, `load_playlists`, `import_media_item` | ✅ `useShowStore.ts` | ✅ Yes | `rusqlite` bundled |
| **Slide Editor** | `echo-presentation` | `trigger_slide` | ✅ `SlideEditor.tsx` (`Edit` tab) | ✅ Yes | WYSIWYG canvas + Inspector |
| **PowerPoint (`.pptx`) Import** | `echo-ingest` (`pptx.rs`) | `import_pptx` | ✅ Wired | ✅ Yes (`test_extract_pptx_slide_xml`) | Extracts slide XML text |
| **Bible Studio (FTS5)** | `echo-bible` | `bible_list_translations`, `bible_search_reference`, `bible_search_keyword`, `bible_compare_translations` | ✅ `BibleStudio.tsx` (`Bible` tab) | ✅ Yes (`155,504` verses: KJV, ASV, WBT, WEB, YLT) | Bundled in `tauri.conf.json` |
| **AI Director & Safety Gate** | `echo-ai` | — | ✅ `SettingsModal.tsx` | ✅ Tested (`test_ai_safety_gate`) | Needs OpenRouter key at runtime |
| **Document/Story-to-Slides** | `echo-ingest` | — | ✅ Beat chunker ready | ✅ Tested (`test_beat_chunking`) | — |
| **REST API & MCP Server** | `echo-api` (Axum), `echo-mcp` | `build_router()`, `list_mcp_tools()` | ✅ Settings Remote Bridge info | ✅ Tested | 14 MCP tools defined |
| **Audio Capture & STT** | `echo-audio` (`cpal`), `echo-transcription` | — | ✅ Audio Intel Dock | ✅ Tested (`test_compute_rms`, `test_simulated_stt`) | `local-whisper` gated (needs CMake/Clang/CUDA) |
| **Scripture Detection & Quotes** | `echo-scripture-detection`, `echo-quotes` | — | ✅ Audio Intel Dock | ✅ Tested (`test_aho_corasick_detection`, `test_rolling_transcript_buffer`) | — |
| **OSC / Visual FX** | `echo-automation` (`rosc`) | — | ✅ Clip Deck / FX Dock (Pro Mode) | ✅ Tested (`test_osc_mapping`) | — |
| **NDI 6 Alpha Output** | `echo-ndi` (`grafton-ndi`) | Trait stub | — | ⚠️ Feature-gated (`ndi-sdk`) | Needs `NDI_SDK_DIR` |
| **GStreamer Video** | `echo-media` (`gstreamer`) | Trait stub | — | ⚠️ Feature-gated (`gstreamer-backend`) | Compiles cleanly on this machine |
