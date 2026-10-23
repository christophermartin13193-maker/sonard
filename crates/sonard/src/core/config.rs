//! # Configuration Module
//!
//! Centralizes configuration handling, parsing, validation, and optimization
//! for the `sonard` daemon.

mod minimized;
mod optimized;
mod standard;
mod valid;

// Internal API
use super::icons;
use standard::{Config, MicConfig, SinksConfig};
use valid::ValidConfig;

// Both API
pub use optimized::OptimizedConfig;

// External API
pub use minimized::MiniConfig;
pub use standard::get_config;
