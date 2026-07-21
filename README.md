<p align="center">
    <img src="assets/logo.svg" width="140" alt="hypr-download-sorter">
</p>

<h1 align="center">hypr-download-sorter</h1>

<p align="center">
    <strong>A native download management daemon for Hyprland & Wayland.</strong>
</p>

<p align="center">
Automatically organize your Downloads folder without thinking about it.
</p>

<p align="center">

![Rust](https://img.shields.io/badge/Rust-1.89%2B-orange?logo=rust)
![Platform](https://img.shields.io/badge/Linux-Wayland-blue)
![License](https://img.shields.io/badge/License-MIT-green)
![Status](https://img.shields.io/badge/status-v1.0.0-success)

</p>

---

# Demo

> *(GIF will be added in the first release.)*

<p align="center">
<img src="assets/demo.gif" width="900">
</p>

---

# Overview

**hypr-download-sorter** is a lightweight daemon that continuously watches your Downloads directory and automatically organizes newly downloaded files into predefined folders.

Unlike simple shell scripts, it is designed to run reliably in the background for long periods while ensuring files are **never moved before downloads finish**.

Built specifically for Linux desktop users who want a clean Downloads folder without manual work.

---

# Features

- 📂 Automatic file organization
- 🦀 Native Rust implementation
- ⚡ Lightweight background daemon
- 🛡 Stability checker prevents incomplete file moves
- ⏳ Smart debounce to avoid duplicate events
- 📑 MIME + extension classification
- 🔔 Desktop notifications
- ⚙️ TOML configuration
- 📝 Structured logging (`tracing`)
- 🧩 Modular architecture
- 🚀 Low memory usage
- 💻 Designed for Wayland & Hyprland

---

# Why hypr-download-sorter?

Many users solve download organization with a short shell script.

While that works for basic cases, it usually lacks features required for a long-running desktop daemon.

| Shell Script | hypr-download-sorter |
|--------------|----------------------|
| Basic automation | Native daemon |
| Immediate file moving | Waits until downloads are stable |
| No debounce | Smart debounce |
| Minimal logging | Structured logging |
| Difficult to extend | Modular architecture |
| Usually no notifications | Desktop notifications |
| Limited error handling | Robust error handling |

This project focuses on **real-world desktop usage** instead of synthetic benchmarks.

The goal is simple:

> Download a file and never think about organizing it again.

---

# Architecture

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
    RuleEngine
        │
        ▼
      Mover
        │
        ▼
    Notifier
```

Each file passes through a small processing pipeline before being moved.

This guarantees correctness while keeping the architecture simple and maintainable.

---

# Supported Categories

| Category | Examples |
|----------|----------|
| Images | jpg, png, jpeg, gif, webp, svg |
| Videos | mp4, mkv, avi, mov |
| Audio | mp3, wav, flac, opus |
| Documents | pdf, docx, xlsx, pptx, txt |
| Archives | zip, rar, 7z, tar.gz |
| Executables | AppImage, deb, rpm |

Unknown files remain untouched.

---

# Configuration

Configuration is stored in:

```
config/default.toml
```

Example:

```toml
[paths]

images = "~/Pictures/Downloads"
videos = "~/Videos/Downloads"
documents = "~/Documents/Downloads"
audio = "~/Music/Downloads"
archives = "~/Archives"
executables = "~/Applications"
unknown = "~/Downloads"
```

The daemon automatically expands `~` to your home directory.

---

# Installation

## Build from source

```bash
git clone https://github.com/YOUR_USERNAME/hypr-download-sorter.git

cd hypr-download-sorter

cargo build --release
```

The compiled binary will be located at:

```text
target/release/hypr-download-sorter
```

---

## Planned installation methods

- install.sh
- systemd user service
- GitHub Release binaries
- Arch Linux (AUR)
- Nix package

---

# Usage

Start the daemon:

```bash
hypr-download-sorter
```

The daemon runs in the background and automatically processes files as they appear.

---

# Compatibility

## Desktop Environments

| Environment | Status |
|------------|--------|
| Hyprland | ✅ Fully Supported |
| Sway | ✅ Supported |
| River | ⚠ Planned |
| GNOME (Wayland) | ⚠ Experimental |
| KDE Plasma (Wayland) | ⚠ Experimental |

---

## Linux Distributions

Tested on:

- Arch Linux
- EndeavourOS

Expected to work on:

- Fedora
- Ubuntu
- Debian
- NixOS

---

## Dotfiles

Official integration is planned for:

- JaKooLit Hyprland
- ML4W
- HyDE
- end-4

---

# Logging

The daemon uses **tracing** for structured logging.

Logs include:

- Filesystem events
- Debounce operations
- Stability checks
- File classification
- Destination resolution
- Successful moves
- Errors

---

# Design Philosophy

hypr-download-sorter prioritizes:

- Reliability
- Correctness
- Maintainability
- Predictable behavior

It intentionally favors real-world desktop experience over synthetic benchmark numbers.

---

# Roadmap

## v1.0

- ✅ Filesystem watcher
- ✅ Debouncer
- ✅ Stability checker
- ✅ Rule engine
- ✅ Classifier
- ✅ Notifications
- ✅ TOML configuration
- ✅ Structured logging

---

## v1.1

- install.sh
- uninstall.sh
- systemd user service
- GitHub Release binaries

---

## Future

- Plugin system
- Advanced rule engine
- Duplicate detection
- Custom rules
- Dotfile integration
- Package manager support

---

# Contributing

Contributions are welcome.

Please read **CONTRIBUTING.md** before opening an Issue or Pull Request.

---

# License

This project is licensed under the MIT License.

See **LICENSE** for details.

---

<p align="center">

Built with ❤️ using Rust.

Designed for Linux desktop users.

</p>