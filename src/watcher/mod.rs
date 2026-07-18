//! Filesystem watching service.
//!
//! This module hides the underlying watcher implementation and exposes
//! application-level filesystem events.

pub mod convert;
pub mod debounce;
pub mod event;
pub mod service;
pub mod stability;

mod engine;

pub use debounce::Debouncer;
pub use engine::WatcherEngine;
pub use event::FileEvent;
pub use service::WatchService;
pub use stability::StabilityChecker;
