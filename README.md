<div align="center">

  <img src="resources/icons/awayra.png" alt="Awayra Logo" width="128" height="128" />

  # Awayra — Calm Break Reminder for Linux

  **A modern, high-performance, native Linux application designed to protect your eyes, prevent RSI, and cultivate healthy computer habits.**

  [![License: MIT](https://img.shields.io/badge/License-MIT-blue.svg)](LICENSE)
  [![Rust](https://img.shields.io/badge/Language-Rust-orange.svg)](https://www.rust-lang.org/)
  [![GTK4](https://img.shields.io/badge/GUI-GTK4-green.svg)](https://gtk.org/)
  [![Platform](https://img.shields.io/badge/Platform-Linux%20%28X11%20%26%20Wayland%20compatible%29-informational.svg)]()

  ---

  [Features](#-key-features) •
  [Installation](#-installation--dependencies) •
  [Building](#-building-from-source) •
  [Original Project](#-original-project) •
  [License](#-license)

</div>

<br />

## 📌 About Awayra

**Awayra** is a lightweight, non-intrusive break scheduler for Linux desktop users. Designed to combat visual fatigue and physical strain caused by prolonged screen exposure, Awayra enforces healthier workstation ergonomics by prompting timely eye resets and micro-stretches.

> ℹ️ **Port Notice**: This project is the official **Linux port** (built with **Rust & GTK4**) of the original Windows application [**AWAYRA-WPF**](https://github.com/AWAYRA/AWAYRA-WPF).

---

## ✨ Key Features

- 👁️ **Eye Reset Reminders (20-20-20 Rule)**  
  Guided step-by-step eye relaxation exercises to reduce digital eye strain and prevent myopia onset.
- 🧘 **Move & Stretch Breaks**  
  Interactive physical activity prompts with animated progress indicators to prevent Repetitive Strain Injury (RSI) and stiffness.
- 🪟 **Immersive Fullscreen Overlay**  
  Distraction-free break window equipped with **Skip**, **Snooze**, and **Complete** actions.
- 📸 **Dynamic Glass & Screenshot Backgrounds**  
  Overlay background modes including live desktop snapshotting (via `xdg-desktop-portal` on Wayland and `XGetImage` on X11), custom image backgrounds, or frosted glass tinting.
- 💤 **Smart Idle Detection**  
  Automatically detects user inactivity (keyboard/mouse) on both **X11** and **Wayland** sessions, automatically pausing timer countdowns while you are away.
- 🔔 **System Tray Integration (`StatusNotifierItem`)**  
  Real-time remaining time status, quick toggle controls, and "Pause until tomorrow" presets directly accessible from your top panel or system tray.
- 🎵 **Synthesized Audio Notifications**  
  Soothing bell and chime audio alerts powered by `rodio` with adjustable volume and preview support.
- 📅 **Customizable Work Hours & Schedules**  
  Define active daily working hours so reminders freeze outside of your office schedule.
- 📊 **Daily Statistics Tracking**  
  Keep track of completed, snoozed, and skipped breaks over time to build lasting ergonomics habits.
- 🌐 **Multi-Language Support**  
  Built-in support for English and Persian (Farsi) localizations out of the box.
- 🚀 **Autostart Integration**  
  Seamless autostart on boot support managed via XDG standard `.desktop` entries.

---

## 🛠️ Tech Stack & Architecture

- **Core Engine:** Written in pure [Rust 2021 Edition](https://www.rust-lang.org/) for memory safety, concurrency, and minimal CPU footprint.
- **GUI Toolkit:** Native [GTK4](https://gtk.org/) binding (`gtk4-rs`) styled with custom CSS for fluid desktop integration.
- **System Tray:** [ksni](https://crates.io/crates/ksni) (`StatusNotifierItem` protocol compatible with GNOME AppIndicator, KDE Plasma, XFCE, Hyprland, Waybar, etc.).
- **Audio Engine:** [rodio](https://crates.io/crates/rodio) audio playback library.
- **System Portals & Display Protocols:** `ashpd` (Wayland Desktop Portal), `x11rb` (X11 ScreenSaver & image extensions).

---

## 📦 Installation & Dependencies

### Prerequisites

Awayra requires **Rust 1.75+** and **GTK4** runtime / development libraries.

#### Installing Dependencies:

- **Arch Linux / Manjaro:**
  ```bash
  sudo pacman -S gtk4 gcc pkg-config
  ```

- **Ubuntu / Debian / Linux Mint:**
  ```bash
  sudo apt update
  sudo apt install libgtk-4-dev build-essential pkg-config
  ```

- **Fedora / RHEL:**
  ```bash
  sudo dnf install gtk4-devel gcc pkgconf-pkg-config
  ```

---

## 🔨 Building from Source

```bash
# Clone the repository
git clone https://github.com/wolf1999h/AWAYRA-Linux.git
cd AWAYRA-Linux

# Build debug binary
cargo build

# Run Awayra
cargo run

# Build optimized release binary
cargo build --release
```

The compiled binary will be located at `target/release/awayra`. You can copy it to `/usr/local/bin` or `~/.local/bin` for system-wide access.

---

## 🌐 Original Project

Awayra for Linux was built to bring the beloved user experience of Awayra Windows to the Linux desktop ecosystem. 
- Original Windows WPF Repository: [**https://github.com/AWAYRA/AWAYRA-WPF**](https://github.com/AWAYRA/AWAYRA-WPF)

---

## 📄 License

This project is licensed under the [MIT License](LICENSE). Feel free to use, modify, and distribute it.
