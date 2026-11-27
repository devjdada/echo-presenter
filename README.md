# EchoPresenter

> **AI-Powered Intelligent Live Presentation & Production Platform**  
> *One application. Five engines. One AI brain.*

Licensed under the **Apache License 2.0** ([LICENSE](LICENSE)).

---

## Features

- **Presentation Studio:** 4-zone ProPresenter-inspired dark operator UI (`Show`, `Edit`, `Bible`, `Reflow`), color-coded slide groups, WYSIWYG Scene Graph JSON, `.pptx` import, `.echopres` portable bundles.
- **Isolated GPU Renderer (`echo-renderer`):** Standalone `winit` + `wgpu 30` + `vello` + `cosmic-text` process communicating over Windows Named Pipes (`rmp-serde` MessagePack) so live audience outputs survive UI reloads.
- **Output Looks:** Single-click routing to `Main Full-Screen`, `NDI Lower-Third Alpha` (premultiplied RGBA), and `Stage Confidence`.
- **Bible Studio:** Bundled public-domain SQLite + FTS5 (`KJV`, `ASV`, `WBT`, `WEB`, `YLT` — 155,504 verses), reference/keyword search, 4-way translation comparison, and instant slide generation.
- **Echo AI & Document Ingest:** OpenRouter + Ollama integration, AI Safety Gate, and `.docx` / `.pptx` / `.md` / `.txt` beat-chunked slide generation.
- **Audio Intelligence & Quotable Quotes:** `cpal` live audio capture, Aho-Corasick scripture detection, and rolling transcript quote extraction.
- **Broadcast Pro:** Axum REST API (`/api/v1/*`), MCP server (`/mcp`), Browser Remote (`/remote`), OSC (`rosc`), and Art-Net DMX512.

---

## Development Setup

```powershell
# 1. Install frontend dependencies
pnpm --dir apps/desktop install

# 2. Run all Rust workspace tests
cargo test --workspace

# 3. Build frontend
pnpm --dir apps/desktop build

# 4. Run GPU renderer smoke test
cargo run -p echo-renderer -- --smoke-test

# 5. Package Windows installer (.msi / .exe)
pnpm --dir apps/desktop tauri build
```
