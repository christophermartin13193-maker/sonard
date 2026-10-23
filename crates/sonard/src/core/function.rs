//! # Function Module
//!
//! Handles PipeWire event processing, proxy bindings, and structures
//! required to track connected and disconnected audio devices.

use super::{MiniConfig, OptimizedConfig};
use pipewire_native::proxy::registry::Registry;
use std::{
    collections::HashMap,
    sync::{Arc, Mutex},
};

mod global;
mod node;
mod remove;
mod wrapper;

// Internal API
type SharedOptimized = Arc<OptimizedConfig>;
type SharedMinimized = Arc<MiniConfig>;
type SharedRegistry = Arc<Registry>;
type Index = Arc<Mutex<HashMap<u32, GlobalRemoveInfo>>>;

use node::get_func;

// Defines the operational duty or state of a tracked device.
pub enum Duty {
    // The device is a sound output sink that was unplugged.
    UnpluggedSink,
    // The device is a microphone source that was unplugged.
    UnpluggedMic,
    // The device is tracked internally without triggering standard unplug events.
    Hidden,
}

// Both API

// Stores metadata associated with a tracked device for removal and notification handling.
pub struct GlobalRemoveInfo {
    // Human-readable description of the device.
    pub description: String,
    // Icon associated with the device event.
    pub icon: String,
    // Specific duty assigned to the device.
    pub duty: Duty,
    // Internal hook reference (private implementation detail)
    pub _hook: Option<u32>,
}

// External API
pub use global::get_global;
pub use remove::get_remove;
pub use wrapper::Wrapper;
