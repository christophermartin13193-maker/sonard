//! # Sonar Library
//!
//! Main entry point for the `sonar` library. It exposes structures,
//! enumerations, and codes required for the proper execution and control of the program.

mod core;

// API
pub use core::{Output, SonarError, Verbosity, code};
