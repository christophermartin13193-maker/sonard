//! # Minimized Configuration Module
//!
//! Defines a lightweight configuration structure focusing exclusively on microphone
//! mute/unmute states and icons for performance optimization.

use super::OptimizedConfig;

// Lightweight representation of microphone configuration settings.
pub struct MiniConfig {
    // Flag indicating whether microphone mute notifications are enabled.
    pub muted_flag: bool,
    // Flag indicating whether microphone unmute notifications are enabled.
    pub unmuted_flag: bool,

    // Custom icon name for microphone muted events.
    pub muted_icon: String,
    // Custom icon name for microphone unmuted events.
    pub unmuted_icon: String,
}

impl OptimizedConfig {
    // Extracts and returns a minimized configuration subset from the optimized configuration.
    pub fn get_minimized_configuration(&self) -> MiniConfig {
        MiniConfig {
            muted_flag: self.optimized_mic.muted_flag,
            unmuted_flag: self.optimized_mic.unmuted_flag,
            muted_icon: self.optimized_mic.muted_icon.clone(),
            unmuted_icon: self.optimized_mic.unmuted_icon.clone(),
        }
    }
}
