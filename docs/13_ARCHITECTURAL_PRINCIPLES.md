# 13 — Architectural Principles

---

## 1. Offline-First Operation

Every core feature works without internet. Bible libraries, presentations, themes, media, and local AI models are stored locally. Cloud AI is optional and gracefully degrades.

---

## 2. AI Provider Independence

Never hardwire to one AI provider. The `AiProvider` trait abstracts chat, STT, embeddings, and image generation. OpenRouter serves as the unified cloud gateway; Ollama and Whisper.cpp provide offline capabilities.

---

## 3. AI Has Application-Wide Access (Gated)

AI can interact with ALL engines through documented tools — presentations, Bible, media, effects, playlists, and production controls. But live operations respect configurable permissions.

---

## 4. Production Reliability First

The `echo-renderer` companion process continues displaying content even if the main Tauri process reloads, AI disconnects, internet drops, or transcription fails. Presentation state is independent of AI state. GPU output surfaces survive UI restarts.

---

## 5. Modularity From Day One

Every engine is an independent, testable Rust crate. No circular dependencies. The event bus decouples engines. Individual crates can be tested without the full application.

---

## 6. Brand-Agnostic Internals

Internal identifiers use `echo-presenter` or `echo_*` prefixes. Database schemas, API paths, and configuration keys avoid brand names. Rebranding is a UI-only change.

---

## 7. WYSIWYG Parity via Scene Graph

A single Scene Graph JSON schema defines slide composition. Both the React slide editor and the `echo-renderer` GPU output consume the same schema, guaranteeing what the operator sees in preview is exactly what the audience sees on screen.

---

## 8. Renderer Isolation

The GPU rendering engine runs in a separate process (`echo-renderer`) communicating via IPC. This isolation ensures live audience output is never interrupted by frontend development hot-reloads, WebView crashes, or Tauri restarts.
