# Awayra — Linux Port

A calm break reminder for healthier computer use.  
Originally a WPF/.NET Windows app, now ported to **Rust + GTK4** for Linux.

## Features

- ⏰ **Break Scheduler** — Eye reset (20s) and Move break (60s) reminders
- 🖥️ **Dashboard** — Real-time countdown, status, and daily statistics
- 🪟 **Break Overlay** — Fullscreen window during breaks with Skip/Snooze/Complete
- ⚙️ **Settings** — Configure intervals, durations, behavior, and screenshot capture
- 🔔 **System Tray** — Quick access to controls (requires AppIndicator on GNOME)
- 🖱️ **Idle Detection** — Pauses reminders when you're away (X11 + Wayland)
- 📸 **Screenshot Capture** — Optional overlay background (portal on Wayland, XGetImage on X11)
- 🚀 **Autostart** — Via `.desktop` file in `~/.config/autostart/`

## Build & Run

```bash
# Build
cargo build

# Run
cargo run

# Release build
cargo build --release
```

## Dependencies

- **Rust** 1.75+ (install via `rustup`)
- **GTK4** development libraries:
  - Arch: `sudo pacman -S gtk4`
  - Ubuntu/Debian: `sudo apt install libgtk-4-dev`
  - Fedora: `sudo dnf install gtk4-devel`

## Project Structure

```
src/
├── main.rs              # Entry point
├── core/                # Platform-independent logic
│   ├── models/          # Data types (settings, state, events)
│   ├── services/        # BreakScheduler, validators, etc.
│   ├── persistence/     # JSON file storage
│   └── localization/    # String resources
└── ui/                  # GTK4 interface
    ├── services/        # AppHost, idle monitor, screenshot
    └── views/           # Dashboard, Overlay, Settings windows
```

## License

MIT