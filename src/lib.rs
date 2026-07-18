//! Core library for hypr-download-sorter.

#![forbid(unsafe_code)]
#![deny(clippy::unwrap_used, clippy::expect_used, clippy::panic, clippy::todo)]
#![warn(missing_debug_implementations, rust_2018_idioms, unused_qualifications)]

pub mod classifier;
pub mod config;
pub mod error;
pub mod models;
pub mod mover;
pub mod notifier;
pub mod paths;
pub mod rules;
pub mod watcher;

pub use error::{AppError, Result};
