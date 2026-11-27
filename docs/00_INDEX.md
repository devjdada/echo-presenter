# EchoPresenter — Documentation Index

> **AI-Powered Intelligent Live Presentation & Production Platform**
>
> *One application. Five engines. One AI brain.*

---

## Identifiers

| Item | Value |
|---|---|
| **Product name** | EchoPresenter |
| **Tauri bundle ID** | `com.echopresenter.app` |
| **Custom URI scheme** | `echopresenter://` |
| **Project bundle extension** | `.echopres` |
| **Rust crate prefix** | `echo-*` |

---

## Plan Documents

| # | Document | Description |
|---|----------|-------------|
| 01 | [Product Vision](01_PRODUCT_VISION.md) | Core differentiator, target markets, what makes EchoPresenter unique |
| 02 | [Competitive Analysis](02_COMPETITIVE_ANALYSIS.md) | Direct competitors, AI competitors (incl. RhemaPresenter), key insights |
| 03 | [System Architecture](03_SYSTEM_ARCHITECTURE.md) | Hybrid rendering, companion renderer process, scene graph, multi-output |
| 04 | [The Five Engines](04_FIVE_ENGINES.md) | Presentation, Visual, Bible, Audio Intelligence, Echo AI |
| 05 | [AI Architecture](05_AI_ARCHITECTURE.md) | Provider abstraction, OpenRouter, local models, safety model |
| 06 | [Bible Engine Deep Dive](06_BIBLE_ENGINE.md) | SQLite schema, import formats, public domain translations, licensing |
| 07 | [Audio Intelligence Pipeline](07_AUDIO_INTELLIGENCE.md) | GPU Whisper tiers, cloud STT fallback, scripture detection, WASAPI routing |
| 08 | [API & MCP Specification](08_API_AND_MCP.md) | REST API, MCP tools, Remote MCP Bridge, `.echopres` deep-link |
| 09 | [Technology Stack & Crates](09_TECHNOLOGY_STACK.md) | Rust crates (2026 releases), frontend deps, full tech stack |
| 10 | [Project Structure](10_PROJECT_STRUCTURE.md) | Directory layout, crate organization, resource locations |
| 11 | [Development Phases](11_DEVELOPMENT_PHASES.md) | 7-phase roadmap (re-sequenced) with deliverables and acceptance criteria |
| 12 | [Build & Packaging](12_BUILD_AND_PACKAGING.md) | Dev environment, LLVM/CUDA/NDI setup, dual-GPU notes, CI/CD |
| 13 | [Architectural Principles](13_ARCHITECTURAL_PRINCIPLES.md) | Offline-first, provider independence, modularity, reliability |
| 14 | [Risk Register](14_RISK_REGISTER.md) | Risks, impact, likelihood, mitigations (incl. dual-GPU, renderer isolation) |
| 15 | [Design Decisions](15_DESIGN_DECISIONS.md) | Approved decisions (1–6) and pending decisions (7–9) |
| 16 | [Bible JSON Assessment](16_BIBLE_JSON_ASSESSMENT.md) | wchapel Bible JSON assessment, split-DB migration strategy |
| 17 | [UI/UX & Output Looks Spec](17_UI_UX_AND_LOOKS_SPEC.md) | 4-zone operator layout, Volunteer/Pro modes, Output Looks matrix |
| 18 | [Quote & Content Pipeline](18_QUOTE_AND_CONTENT_PIPELINE.md) | Live Quotable Quotes engine, Document/Story-to-Slides pipeline |
