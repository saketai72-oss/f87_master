# AULA F87 RGB Controller (`f87_master`)

[🇻🇳 Tiếng Việt](README_VI.md) | [🇬🇧 English](README.md)

Open-source, high-performance per-key RGB lighting software and animation suite for the **AULA F87 / F87 Pro** mechanical keyboard (Wired USB `258a:010c` and 2.4GHz Wireless Dongle `3554:fa09`), built natively in Rust.

---

## ⚡ Highlights & Features

- **Full ANSI 87-Key Tenkeyless (TKL) Control**: Pixel-accurate per-key RGB streaming without requiring bloated vendor software or cloud accounts.
- **Ultra-Smooth Dual Connection (USB-C & 2.4GHz Wireless at 21-25 FPS)**: Automatic detection and seamless switching. The 2.4GHz Dongle mode is specifically optimized with Dynamic Adaptive Color Quantization and tuned radio pacing (82 chunks/s), achieving **21–25 FPS fluid streaming** and completely unlocking keyframe animations and audio visualizers over wireless.
- **Single-Instance Protection**: Enforces a system-wide named mutex (`Local\AULA_F87_RGB_CONTROLLER_MUTEX`) to ensure clean hardware access.
- **Music Equalizer (Nháy theo nhạc)**: Dynamic 16-band audio graphic equalizer with realistic bass bounce, mid-frequency harmonics, peak hold meters, and beat pulse shockwaves.
- **Rich Effect Library**:
  - `Music Equalizer`: Audio-reactive frequency visualizer.
  - `Cyber Pulse`: Synthwave dual-color cyan/magenta waves.
  - `Matrix Rain`: Cascading digital code stream.
  - `Fire`: Rising ember particles with organic flicker.
  - `Aurora`: Borealis ribbons with shimmering emerald and violet gradients.
  - `Wave`, `Breathe`, `Ripple`, `Spiral`, `Bloom`, `Sweep`, `Solid`, `Text`.
- **Per-Setting "Set Default" Controls**: Dedicated reset buttons for each slider, color picker, and tempo parameter.
- **Background Tray Persistence**: Closing the window (`[X]`) automatically minimizes the app to the system tray so lighting never stops.
- **Hardware Memory Retention**: When quitting the app, the active frame is automatically written to the keyboard's static onboard memory (`kb.set_static`), keeping keys illuminated instead of turning dark.
- **Streamlined UI**: Clean, responsive layout without unnecessary sidebars; live keyboard visualizer; top quick-device switcher; tray minimization.
- **Keyframe & Composition Editor**: Timeline keyframe painter, layered masks, and GIF/image import.
- **Stand-alone Installer**: Native Windows installer with desktop and Start Menu shortcuts, folder selection, and clean uninstaller.

---

## 📦 Download & Installation

Pre-built binaries are ready in [`release_package/`](release_package/):

| File | Description |
| --- | --- |
| **[`AULA-F87-Setup-x64.exe`](release_package/AULA-F87-Setup-x64.exe)** | Full Windows Installer (Start Menu, Desktop Shortcut, Uninstaller) |
| **[`f87.exe`](release_package/f87.exe)** | Portable executable (runs standalone without installation) |

---

## 💡 AULA F87 Hardware Notes

On AULA F87 hardware, the physical **side LED strips** and **middle battery lightbar** are managed directly by an independent onboard MCU sub-controller. They are controlled using onboard hardware hotkeys:

- **`Fn + Right Shift`**: Cycle side light modes (Wave, Breathing, Static, Neon, Off).
- **`Fn + Right Ctrl`**: Change side light color.
- **`Fn + Right Alt`**: Adjust side light brightness.
- **`Fn + >`**: Adjust side light animation speed.
- **`Fn + B`**: Display battery percentage on the middle lightbar.

The software per-key streaming stream addresses the 87 mechanical key switches with full fidelity and zero radio overhead.

---

## 🛠️ Building from Source

### Prerequisites
- **Rust Toolchain**: 1.75+ (`stable-x86_64-pc-windows-msvc` or `gnu`)
- **Git**

```powershell
# Clone the repository
git clone https://github.com/saketai72-oss/f87_master.git
cd f87_master

# Run all automated tests
cargo test --workspace

# Build the release binary
cargo build --release -p aula-app --bin f87

# Build standalone installer
cargo build --release --manifest-path packaging/windows/installer/Cargo.toml
```

Binary outputs:
- App binary: `target/release/f87.exe`
- Installer: `target/release/AULA-F87-Setup-x64.exe`

---

## 🏗️ Architecture

```
f87_master/
├── crates/
│   ├── aula-protocol/   # SinoWealth & Compx HID protocols, packet framing, and radio pacing
│   ├── aula-effects/    # Rhai scripting host, keyframe timeline engine, and composition layers
│   └── aula-app/        # Egui/Eframe desktop interface, render thread, and settings
├── effects/             # Hot-reloaded .rhai effect scripts
├── packaging/windows/   # Native Windows installer builder
└── release_package/     # Packaged release binaries and installer
```

---

## 📜 License

Licensed under either of [MIT](LICENSE-MIT) or [Apache-2.0](LICENSE-APACHE) at your option.
