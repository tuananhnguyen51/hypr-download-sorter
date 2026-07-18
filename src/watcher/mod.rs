//! Filesystem watching service.
//!
//! This module hides the underlying watcher implementation and exposes
//! application-level filesystem events.

mod convert;
mod debounce;
mod event;
mod service;

pub use debounce::Debouncer;
pub use event::{FileEvent, FileEventKind};
pub use service::WatchService;
