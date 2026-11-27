# 14 — Risk Register

---

| Risk | Impact | Likelihood | Mitigation |
|---|---|---|---|
| **GStreamer complexity on Windows** | High | Medium | Start with simple playback, use pre-built binaries, test early |
| **wgpu 30 multi-output reliability** | High | Low | Test on multiple GPUs, start with 2-output, scale up. Dual-GPU adapter pinning (see below) |
| **Companion renderer IPC stability** | High | Medium | Use MessagePack over named pipes; implement heartbeat + auto-restart watchdog. Test crash recovery in Phase 1 |
| **Dual-GPU adapter pinning** | Medium | Medium | Intel UHD 630 + NVIDIA Quadro RTX 3000 require explicit `wgpu` adapter enumeration and selection. Test fallback when discrete GPU is absent. Add CI test on single-GPU machine |
| **Whisper.cpp real-time performance** | Medium | Low | Start with `small.en` on GPU, `base.en` CPU fallback. Cloud STT toggle for difficult audio |
| **NDI 6 SDK licensing** | Medium | Medium | Review `grafton-ndi` terms before commercial release. Prepare Spout fallback. NDI DLL must be bundled or detected at first launch |
| **Bible translation licensing** | Medium | Low | Ship public domain only in `bundled_public_domain.sqlite`. User-imported in `user_imported.sqlite`, strictly `.gitignore`d |
| **Tauri 2 multi-monitor edge cases** | Medium | Medium | Test early with Tao window positioning. Renderer companion owns output windows independently of Tauri |
| **LLVM / CMake / CUDA build chain** | Medium | Medium | Document exact setup in [12_BUILD_AND_PACKAGING.md](12_BUILD_AND_PACKAGING.md). Pre-build steps in CI. Provide `scripts/setup-dev.ps1` |
| **Vello + cosmic-text maturity** | Medium | Medium | These are newer than Skia. Evaluate text rendering quality early in Phase 1. Skia fallback documented in Decision 7 |
| **Scene Graph JSON parity drift** | Medium | Low | Single JSON schema source of truth. Property-test: serialize in React, render in `echo-renderer`, screenshot-diff |
| **Scope creep across 5 engines** | High | High | Strict phase gates: Output Looks + NDI in Phase 1, Visuals deferred to Phase 6. Don't start Phase N+1 until Phase N is accepted |
| **AI provider API changes** | Low | Medium | Abstraction layer. OpenRouter shields from individual provider changes |
| **Remote MCP Bridge security** | High | Low | Opt-in only. Cloudflare Tunnel with API key auth + rate limiting. Never expose without explicit user consent |
