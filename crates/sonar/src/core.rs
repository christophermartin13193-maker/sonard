//! # Core Module
//!
//! This module centralizes and re-exports fundamental elements of the library's API,
//! whether intended for internal use or external consumption.

mod control;
mod verbosity;

// Both API
pub use control::SonarError;
pub use verbosity::Verbosity;

// External API
pub use control::{Output, code};
