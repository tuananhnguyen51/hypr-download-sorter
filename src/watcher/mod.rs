//! Filesystem watching service.
//!
//! This module hides the underlying watcher implementation and exposes
//! application-level filesystem events.

mod convert;
mod debounce;
mod engine;
mod event;
mod service;
mod stability;

pub use debounce::Debouncer;
pub use engine::WatcherEngine;
pub use event::{FileEvent, FileEventKind};
pub use service::WatchService;
pub use stability::StabilityChecker;
