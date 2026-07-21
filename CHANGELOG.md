# Changelog

All notable changes to this project will be documented in this file.

The format is based on **Keep a Changelog**, and this project follows **Semantic Versioning**.

## [1.0.0] - 2026-07-21

### Added

* Initial public release.
* Filesystem watcher based on `notify`.
* Debounce system to merge duplicated filesystem events.
* Stability checker to avoid moving incomplete downloads.
* MIME and extension based file classification.
* Rule engine for category-based destination directories.
* Configurable paths via `config/default.toml`.
* Asynchronous processing pipeline built with Tokio.
* Desktop notifications for successful and failed operations.
* Structured logging using `tracing`.

### Changed

* Refactored the application into independent modules:

  * Classifier
  * RuleEngine
  * Pipeline
  * Mover
  * Notifier
  * WatchEngine
* Replaced hardcoded directories with configurable destinations.
* Improved error handling using a unified `AppError` type.

### Performance

* Reduced duplicate processing with event debouncing.
* Added file stability verification before moving files.
* Optimized filesystem polling for lower CPU usage.

### Fixed

* Fixed duplicate filesystem events.
* Fixed moving files before downloads were fully completed.
* Fixed destination path resolution.
* Fixed notification error propagation.
* Fixed Clippy warnings and formatting issues.

---

Future releases will be documented here.
