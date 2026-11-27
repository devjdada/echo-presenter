# 12 — Build & Packaging

---

## Your Development Environment ✅

| Component | Status | Details |
|---|---|---|
| **OS** | ✅ | Windows 11 Pro (22000) |
| **Rust** | ✅ | 1.99.1 |
| **Node.js** | ✅ | 24.12.0 |
| **Primary GPU** | ✅ | NVIDIA Quadro RTX 3000 (wgpu rendering + CUDA Whisper) |
| **Secondary GPU** | ✅ | Intel UHD 630 (can drive additional output displays) |

---

## Required Setup

```powershell
# 1. Install Tauri CLI
cargo install tauri-cli

# 2. Install pnpm (faster than npm)
npm install -g pnpm

# 3. Install GStreamer (Windows runtime + development)
# Download from https://gstreamer.freedesktop.org/download/
# Install BOTH runtime and development packages
# Set GST_PLUGIN_PATH and PATH accordingly

# 4. Visual Studio Build Tools
# Tauri needs MSVC C++ build tools + Windows 10/11 SDK

# 5. LLVM / Clang (required by whisper-rs, some crates with C bindings)
# Download from https://github.com/llvm/llvm-project/releases
# Set LIBCLANG_PATH=C:\Program Files\LLVM\bin
choco install llvm
$env:LIBCLANG_PATH = "C:\Program Files\LLVM\bin"

# 6. CMake (required by whisper-rs build)
choco install cmake --installargs 'ADD_CMAKE_TO_PATH=System'

# 7. CUDA Toolkit (for GPU-accelerated Whisper)
# Download from https://developer.nvidia.com/cuda-toolkit
# Ensure nvcc is on PATH
# Required for whisper-rs "cuda" feature flag

# 8. NDI 6 SDK (for grafton-ndi)
# Download from https://ndi.video/for-developers/ndi-sdk/
# Set NDI_SDK_DIR=C:\Program Files\NDI\NDI 6 SDK
# Copy Processing.NDI.Lib.x64.dll to target directory or PATH

# 9. ASIO SDK (optional, for cpal ASIO support)
# Download from https://www.steinberg.net/developers/
# Set CPAL_ASIO_DIR=path\to\asio-sdk
```

---

## Dual-GPU `wgpu` Adapter Pinning

The `echo-renderer` companion process must pin specific GPU adapters to specific outputs:

| Output | Recommended GPU | Reason |
|---|---|---|
| Main LED / projector | NVIDIA Quadro RTX 3000 | Full shader support, effects, compositing |
| Stage confidence monitor | Intel UHD 630 | Drive 3rd display without loading main GPU |
| NDI RGBA output | NVIDIA Quadro RTX 3000 | GPU encode + NDI send |

```rust
// In echo-renderer: enumerate and pin adapters
let instance = wgpu::Instance::new(&wgpu::InstanceDescriptor::default());
let adapters = instance.enumerate_adapters(wgpu::Backends::all());
// Select by name or vendor ID:
// - "NVIDIA Quadro RTX 3000" for primary rendering
// - "Intel(R) UHD Graphics 630" for secondary outputs
```

> [!WARNING]
> If the NVIDIA GPU is not available (e.g., laptop with no discrete GPU), `echo-renderer` falls back to the default adapter. Test this fallback path in CI.

---

## Packaging Targets

| Platform | Format | Method |
|---|---|---|
| **Windows** | `.msi` / `.exe` (NSIS) | ✅ Direct from Windows (`cargo tauri build`) |
| **Linux** | `.AppImage` / `.deb` | Via WSL2 or GitHub Actions CI |
| **macOS** | `.dmg` / `.app` | Via macOS runner in GitHub Actions CI |

> [!WARNING]
> **Cross-compilation limitation:** You cannot build Linux packages directly from Windows. Use WSL2 for local Linux builds, or set up GitHub Actions for automated cross-platform CI/CD. macOS builds require a macOS runner.

---

## Installer Must Include

| Asset | Path |
|---|---|
| `echo-renderer` binary | `resources/echo-renderer.exe` |
| `bundled_public_domain.sqlite` | `resources/bible-modules/` |
| Whisper `base.en` model | `resources/ai-models/ggml-base.en.bin` |
| Sample themes + templates | `resources/themes/`, `resources/templates/` |
| NDI 6 runtime DLL | Bundled or checked at first launch |

---

## Recommended CI/CD

```yaml
# .github/workflows/build.yml
name: Build EchoPresenter
on: [push, pull_request]
jobs:
  build-windows:
    runs-on: windows-latest
    steps:
      - checkout
      - setup-rust
      - setup-node + pnpm
      - install LLVM, CMake, GStreamer, NDI SDK
      - cargo tauri build
      - upload .msi artifact

  build-linux:
    runs-on: ubuntu-latest
    steps:
      - checkout
      - setup-rust
      - setup-node + pnpm
      - install linux deps (libgtk-3, libwebkit2gtk, gstreamer)
      - cargo tauri build
      - upload .AppImage artifact

  build-macos:
    runs-on: macos-latest
    steps:
      - checkout
      - setup-rust
      - setup-node + pnpm
      - cargo tauri build
      - upload .dmg artifact
```
