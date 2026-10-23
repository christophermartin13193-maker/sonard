//! # Optimized Configuration Module
//!
//! Processes raw valid configurations into optimized structures with resolved default icons
//! and pre-calculated feature flags for efficient runtime checks.

use super::icons::*;
use super::{MicConfig, SinksConfig, ValidConfig};

// Fully processed and optimized configuration containing resolved flags and icons.
pub struct OptimizedConfig {
    // Optimized settings for audio sinks.
    pub optimized_sinks: OptimizedSinks,
    // Optimized settings for microphones.
    pub optimized_mic: OptimizedMic,
}

// Optimized configuration properties for audio sinks.
pub struct OptimizedSinks {
    // Flag indicating if sink plugged notifications are active.
    pub plugged_flag: bool,
    // Flag indicating if sink unplugged notifications are active.
    pub unplugged_flag: bool,
    // Internal flag for current sink status
    pub _current_flag: bool,

    // Resolved icon name for sink plugged events.
    pub plugged_icon: String,
    // Resolved icon name for sink unplugged events.
    pub unplugged_icon: String,
    // Internal icon for current sink status
    pub _current_icon: String,
}

// Optimized configuration properties for microphones.
pub struct OptimizedMic {
    // Flag indicating if microphone plugged notifications are active.
    pub plugged_flag: bool,
    // Flag indicating if microphone unplugged notifications are active.
    pub unplugged_flag: bool,
    // Flag indicating if microphone muted notifications are active.
    pub muted_flag: bool,
    // Flag indicating if microphone unmuted notifications are active.
    pub unmuted_flag: bool,

    // Resolved icon name for microphone plugged events.
    pub plugged_icon: String,
    // Resolved icon name for microphone unplugged events.
    pub unplugged_icon: String,
    // Resolved icon name for microphone muted events.
    pub muted_icon: String,
    // Resolved icon name for microphone unmuted events.
    pub unmuted_icon: String,
}

impl ValidConfig {
    // Transforms a valid configuration into an optimized runtime configuration.
    pub fn get_optimized_configuration(self) -> OptimizedConfig {
        let optimized_sinks = get_optimized_sinks(self.sinks);
        let optimized_mic = get_optimized_mic(self.mic);

        OptimizedConfig {
            optimized_sinks,
            optimized_mic,
        }
    }
}

// Helper function to build optimized sink configurations and fallback icons.
fn get_optimized_sinks(sinks: Option<SinksConfig>) -> OptimizedSinks {
    match sinks {
        None => OptimizedSinks {
            plugged_flag: false,
            unplugged_flag: false,
            _current_flag: false,
            plugged_icon: DEFAULT_SINK_PLUGGED.to_string(),
            unplugged_icon: DEFAULT_SINK_UNPLUGGED.to_string(),
            _current_icon: DEFAULT_SINK_CURRENT.to_string(),
        },
        Some(sinks) => {
            let plugged_flag = sinks.plugged.is_some();
            let plugged_icon = sinks
                .plugged
                .filter(|s| !s.is_empty())
                .unwrap_or(DEFAULT_SINK_PLUGGED.to_string());

            let unplugged_flag = sinks.unplugged.is_some();
            let unplugged_icon = sinks
                .unplugged
                .filter(|s| !s.is_empty())
                .unwrap_or(DEFAULT_SINK_UNPLUGGED.to_string());

            let current_flag = sinks.current.is_some();
            let current_icon = sinks
                .current
                .filter(|s| !s.is_empty())
                .unwrap_or(DEFAULT_SINK_CURRENT.to_string());

            OptimizedSinks {
                plugged_flag,
                unplugged_flag,
                _current_flag: current_flag,
                plugged_icon,
                unplugged_icon,
                _current_icon: current_icon,
            }
        }
    }
}

// Helper function to build optimized microphone configurations and fallback icons.
fn get_optimized_mic(mic: Option<MicConfig>) -> OptimizedMic {
    match mic {
        None => OptimizedMic {
            plugged_flag: false,
            unplugged_flag: false,
            muted_flag: false,
            unmuted_flag: false,
            plugged_icon: DEFAULT_MIC_PLUGGED.to_string(),
            unplugged_icon: DEFAULT_MIC_UNPLUGGED.to_string(),
            muted_icon: DEFAULT_MIC_MUTED.to_string(),
            unmuted_icon: DEFAULT_MIC_UNMUTED.to_string(),
        },
        Some(mic) => {
            let plugged_flag = mic.plugged.is_some();
            let plugged_icon = mic
                .plugged
                .filter(|s| !s.is_empty())
                .unwrap_or(DEFAULT_MIC_PLUGGED.to_string());

            let unplugged_flag = mic.unplugged.is_some();
            let unplugged_icon = mic
                .unplugged
                .filter(|s| !s.is_empty())
                .unwrap_or(DEFAULT_MIC_UNPLUGGED.to_string());

            let muted_flag = mic.muted.is_some();
            let muted_icon = mic
                .muted
                .filter(|s| !s.is_empty())
                .unwrap_or(DEFAULT_MIC_MUTED.to_string());

            let unmuted_flag = mic.unmuted.is_some();
            let unmuted_icon = mic
                .unmuted
                .filter(|s| !s.is_empty())
                .unwrap_or(DEFAULT_MIC_UNMUTED.to_string());

            OptimizedMic {
                plugged_flag,
                unplugged_flag,
                muted_flag,
                unmuted_flag,
                plugged_icon,
                unplugged_icon,
                muted_icon,
                unmuted_icon,
            }
        }
    }
}
