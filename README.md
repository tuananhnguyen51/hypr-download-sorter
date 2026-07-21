# hypr-download-sorter

> A lightweight download management daemon for Linux.
>
> Automatically organize your Downloads folder in the background.

![Platform](https://img.shields.io/badge/Linux-Wayland-blue)
![Rust](https://img.shields.io/badge/Rust-stable-orange)
![License](https://img.shields.io/github/license/tuananhnguyen51/hypr-download-sorter)

---

## ✨ Features

- 📂 Automatically organize downloaded files
- 🦀 Written in pure Rust
- ⚡ Lightweight background daemon
- ⏳ Smart debounce prevents duplicate events
- 🛡 Waits until downloads are completely finished
- 📑 MIME + extension based classification
- 🔔 Native desktop notifications
- ⚙️ TOML configuration
- 📝 Structured logging (`tracing`)
- 🔧 systemd user service
- 💻 Built for Wayland desktops

---

## 🎥 Demo

Coming soon.

A short GIF demonstrating automatic file organization will be added in a future release.

---

## 🚀 Installation

### Requirements

- Linux
- Rust (stable)

Install Rust if needed:

```bash
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
```

Clone and install:

```bash
git clone https://github.com/tuananhnguyen51/hypr-download-sorter.git \
&& cd hypr-download-sorter \
&& bash scripts/install.sh
```

The installer automatically:

- builds the release binary
- installs the default configuration
- installs the systemd user service

Start the daemon:

```bash
systemctl --user enable --now hypr-download-sorter
```

---

## ⚙ Configuration

Configuration file:

```text
~/.config/hypr-download-sorter/config/default.toml
```

Example:

```toml
watch_dir = "~/Downloads"

documents = "~/Documents/Downloads"
images = "~/Pictures/Downloads"
videos = "~/Videos/Downloads"
music = "~/Music/Downloads"
archives = "~/Archives/Downloads"
executables = "~/Applications/Downloads"
```

---

## 📂 Supported File Types

| Category | Examples |
|----------|----------|
| Images | jpg png jpeg gif webp svg |
| Videos | mp4 mkv mov avi webm |
| Audio | mp3 flac wav opus ogg |
| Documents | pdf docx xlsx pptx txt |
| Archives | zip rar 7z tar gz |
| Executables | AppImage deb rpm |

Unknown files remain untouched.

---

## 🏗 Architecture

```
Filesystem Events
        │
        ▼
  WatchService
        │
        ▼
    Debouncer
        │
        ▼
 StabilityChecker
        │
        ▼
    Classifier
        │
        ▼
    Rule Engine
        │
        ▼
      Mover
        │
        ▼
    Notifier
```

Each downloaded file flows through the pipeline before being moved.

This guarantees that partially downloaded files are never processed.

---

## 💻 Compatibility

### Desktop

| Environment | Status |
|-------------|--------|
| Hyprland | ✅ Fully Supported |
| Sway | ✅ Supported |
| GNOME Wayland | ⚠ Untested |
| KDE Plasma Wayland | ⚠ Untested |

### Linux

Tested on:

- Arch Linux
- EndeavourOS

Expected to work on:

- Fedora
- Ubuntu
- Debian
- NixOS

---

## 🎯 Philosophy

Unlike a small shell script, **hypr-download-sorter** is designed as a long-running desktop daemon.

It focuses on:

- Reliability
- Correctness
- Maintainability
- Real-world desktop usage

The goal is simple:

> Download files.
>
> Forget about organizing them.

---

## 🛣 Roadmap

### v1.0

- ✅ Filesystem watcher
- ✅ Debouncer
- ✅ Stability checker
- ✅ Rule engine
- ✅ Notifications
- ✅ TOML configuration
- ✅ Install script
- ✅ systemd integration

### Future

- Custom rules
- Duplicate detection
- Plugin system
- GitHub Release binaries
- AUR package
- Nix package

---

## 🤝 Contributing

Issues, bug reports and feature requests are always welcome.

---

## 📜 License

Released under the MIT License.

See the LICENSE file for details.

---

Built with ❤️ in Rust.

Designed for Linux desktop users.