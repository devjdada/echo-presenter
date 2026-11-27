# 15 — Design Decisions

---

## Approved Decisions

### Decision 1: Visual Studio Integration — ✅ Approved: **A. Fully Integrated**

| Option | Description | Status |
|---|---|---|
| **A. Fully Integrated** | Build a native Resolume-inspired engine inside EchoPresenter | ✅ **Approved** |
| B. Companion | EchoPresenter controls an external Resolume installation | — |
| C. Both | Native engine + optional external Resolume control | Long-term goal |

---

### Decision 2: AI Provider Strategy — ✅ Approved: **A. Hybrid**

| Option | Description | Status |
|---|---|---|
| **A. Hybrid** | Cloud AI (OpenRouter) + Local AI (Whisper, Ollama), offline capable | ✅ **Approved** |
| B. Cloud-first | OpenRouter/OpenAI/Anthropic/Gemini only | — |
| C. Local-first | Local models primary, cloud optional | — |

---

### Decision 3: Initial Product Scope — ✅ Approved: **C. Modular Release**

| Option | Description | Status |
|---|---|---|
| A. Full platform | All 5 engines from the start | — |
| B. Church-first | Perfect presentation + Bible, then expand | — |
| **C. Modular release** | Core foundation → add engines progressively | ✅ **Approved** |

---

### Decision 4: Package Manager — ✅ Approved: **A. pnpm**

| Option | Description | Status |
|---|---|---|
| **A. pnpm** | Fast, disk-efficient, strict | ✅ **Approved** |
| B. npm | Standard, universal | — |
| C. bun | Very fast, newer | — |

---

### Decision 5: State Management (Frontend) — ✅ Approved: **A. Zustand**

| Option | Description | Status |
|---|---|---|
| **A. Zustand** | Lightweight, simple, React-first | ✅ **Approved** |
| B. Jotai | Atomic, granular reactivity | — |
| C. Redux Toolkit | Full-featured, more boilerplate | — |

---

### Decision 6: Product Name — ✅ Approved: **EchoPresenter**

| Item | Value |
|---|---|
| Product name | **EchoPresenter** |
| Tauri bundle ID | `com.echopresenter.app` |
| Custom URI scheme | `echopresenter://` |
| Project bundle extension | `.echopres` |
| Rust crate prefix | `echo-*` (unchanged) |

---

## Approved Decisions (7–9)

### Decision 7: 2D / Text Renderer — ✅ Approved: **A. Vello + cosmic-text**

| Option | Description | Status |
|---|---|---|
| **A. Vello + cosmic-text** | GPU-native Rust 2D renderer + Unicode text engine. Pure Rust, no C++ FFI, smaller binary. | ✅ **Approved** |
| B. skia-safe | Google Skia via Rust FFI. Battle-tested text rendering, C++ FFI, ~20 MB binary addition. | — |

---

### Decision 8: Render Engine Isolation — ✅ Approved: **Companion `echo-renderer` process (Named Pipes + MessagePack)**

| Option | Description | Status |
|---|---|---|
| **Companion `echo-renderer` process** | Separate native executable (`winit` + `wgpu` + `vello`) communicating via Windows Named Pipes + `rmp-serde` MessagePack. Live output survives Tauri restarts. | ✅ **Approved** |
| In-process Tauri native window | wgpu surfaces created in the Tauri process. Simpler, but output dies on UI reload. | — |

---

### Decision 9: Cloud AI Bridge — ✅ Approved: **B. Local `rmcp` + opt-in Remote Relay + `.echopres` deep-link**

| Option | Description | Status |
|---|---|---|
| A. Local `rmcp` only | MCP server on localhost. External AI must run on the same machine or LAN. | — |
| **B. Local `rmcp` + opt-in Remote Relay + `.echopres` deep-link** | Add authenticated Cloudflare Tunnel for cloud AI + `.echopres` bundle import via `echopresenter://` URI. | ✅ **Approved** |

---

> [!IMPORTANT]
> **All 9 Design Decisions are Approved.** Proceeding with Phase 1 milestones.
