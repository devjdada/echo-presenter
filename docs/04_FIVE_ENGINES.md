# 04 — The Five Engines

---

## Engine 1: Presentation Studio (ProPresenter-inspired)

The core slide and service management engine.

| Feature | Description |
|---|---|
| **Slide Editor** | Text, images, shapes, videos — multiple independent objects per slide |
| **Slide Groups** | Lyrics, scripture, sermon notes, announcements, custom |
| **Themes** | Reusable design templates with fonts, colors, backgrounds |
| **Animations** | Entrance, exit, text reveal, object motion (spring physics) |
| **Transitions** | Cut, fade, wipe, dissolve, push, custom shader transitions |
| **Playlists** | Complete service programmes with ordering and cue points |
| **Live Preview** | WYSIWYG preview before sending to audience |
| **Layers** | Independent: background, lyrics, scripture, announcements, media, logo |
| **Stage Display** | Current/next slides, clocks, countdown timers, messages to stage |
| **Live Editing** | Edit slides without interrupting the current output |
| **PowerPoint Import** | Parse .pptx and retain text, images, layouts |
| **Document Ingest** | Import PDF, DOCX, TXT, MD → semantic slide templates |
| **Timeline** | Synchronize media, slides, and production actions to a timeline |
| **Output Looks** | Single click routes different compositions per output simultaneously |

> [!NOTE]
> A slide is NOT a single text box. It is a **composition** — a canvas containing multiple independently controllable objects (text, images, video, shapes) with their own animations, positions, and blend modes. The composition is defined by the shared Scene Graph JSON schema (see [03_SYSTEM_ARCHITECTURE.md](03_SYSTEM_ARCHITECTURE.md)).

---

## Engine 2: Visual Studio (Resolume-inspired)

Live visual performance and effects engine.

| Feature | Description |
|---|---|
| **Clip Decks** | Banks of video clips, images, and generative visuals |
| **Multiple Layers** | Stack and blend visual sources |
| **Live Effects** | Blur, glow, color shift, distortion, particles |
| **Effects Rack** | Chain effects per-clip, per-layer, or per-composition |
| **Blend Modes** | Add, multiply, screen, overlay, etc. |
| **Opacity / Transform** | Per-layer position, scale, rotation, opacity |
| **Audio-Reactive** | Visual effects driven by audio amplitude/frequency |
| **Camera Inputs** | Live video capture via GStreamer |
| **Projection Mapping** | Multi-surface mapping with warping |
| **LED Wall Mapping** | Slice, position, and crop for LED panel arrays |
| **NDI Input/Output** | Network video I/O (NDI 6, premultiplied RGBA alpha) |
| **Spout** | Windows local GPU texture sharing |
| **MIDI/OSC/DMX** | Hardware control of visual parameters |

> [!TIP]
> The Visual Studio is hidden by default in **Volunteer Mode**. Toggle to **Pro Visual Mode** to unlock the full Resolume-grade effects rack and layer controls.

---

## Engine 3: Bible Studio

Dedicated scripture workspace with deep search and presentation.

| Feature | Description |
|---|---|
| **Bible Library** | Manage installed translations |
| **Translation Manager** | Download, import, enable/disable translations |
| **Search by Reference** | `John 3:16`, `Rom 8:28`, `1 Cor 13` |
| **Search by Keyword** | `everlasting life`, `forgiveness` |
| **Multi-Translation View** | Side-by-side comparison (up to 4 simultaneous) |
| **Verse Splitting** | Auto-split passages into presentation slides |
| **Scripture Collections** | Bookmarks, favorites, sermon lists |
| **Scripture History** | Track all displayed verses |
| **Offline Mode** | All Bibles stored locally in SQLite |
| **Display Themes** | Customizable verse card templates |
| **Scripture Intelligence** | AI-powered contextual scripture suggestions |

---

## Engine 4: Audio Intelligence Studio

Real-time sermon listening, analysis, and quotable-quotes extraction.

| Feature | Description |
|---|---|
| **Audio Input Selection** | Microphone, mixer, system audio, NDI audio, WASAPI/ASIO channel select |
| **Live Waveform** | Real-time audio level monitoring |
| **Speech Transcription** | Offline via Whisper (GPU-accelerated), optional cloud STT |
| **Scripture Detection** | Aho-Corasick for direct references, HNSW for semantic matching |
| **Quotable Quotes** | Real-time extraction of powerful sermon moments into graphics |
| **Confidence Scores** | Each detection rated: High / Medium / Low |
| **Scripture Queue** | Detected verses queue for operator review |
| **Auto/Manual Mode** | Auto-present high-confidence, queue lower for approval |
| **Speaker ID** | Optional speaker diarization |
| **Sermon Recording** | Capture and store sermon audio with timestamps |

---

## Engine 5: Echo AI

The central intelligence layer connecting to all engines.

### Operating Modes

1. **AI Chat** — Natural language commands (`"Create a 12-slide presentation on faith"`)
2. **Voice Control** — Spoken commands (`"Echo, next slide"`, `"Display John 3:16"`)
3. **AI Automation** — Trigger workflows (`When worship starts → load playlist, activate background, enable lyrics`)
4. **External AI Agents** — MCP/API access for ChatGPT, Gemini, Claude, Antigravity, Codex
5. **Document/Story-to-Slides** — Import PDF, DOCX, sermon notes → semantic slide generation
