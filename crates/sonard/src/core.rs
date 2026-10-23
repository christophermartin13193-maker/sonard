//! # Sonard Core Module
//!
//! Centralizes internal submodules related to workflows, functions, configurations,
//! and icons, while exposing the main automaton to external modules.

mod config;
mod function;
mod icons;
mod workflow;

// Internal API
use config::{MiniConfig, OptimizedConfig, get_config};
use function::{GlobalRemoveInfo, Wrapper, get_global, get_remove};

// External API
pub use workflow::Automaton;
