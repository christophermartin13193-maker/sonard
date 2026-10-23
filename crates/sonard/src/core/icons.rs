//! # Icons Module
//!
//! Defines default icon names used for audio device notifications (sinks and microphones).

// --- Default Sinks Icons ---
pub const DEFAULT_SINK_PLUGGED: &str = "audio-headset";
pub const DEFAULT_SINK_UNPLUGGED: &str = "dialog-warning";
pub const DEFAULT_SINK_CURRENT: &str = "audio-speakers";

// --- Default Microphones Icons ---
pub const DEFAULT_MIC_PLUGGED: &str = "audio-input-microphone";
pub const DEFAULT_MIC_UNPLUGGED: &str = "microphone-disabled";
pub const DEFAULT_MIC_UNMUTED: &str = "microphone-sensitivity-high";
pub const DEFAULT_MIC_MUTED: &str = "microphone-sensitivity-muted";
