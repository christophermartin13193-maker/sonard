//! # Removal Event Module
//!
//! Handles cleanup and disconnection notifications when audio devices are removed from the registry.

use super::{Duty, Index, OptimizedConfig};
use notify_rust::Notification;
type GlobalRemove = Option<Box<dyn FnMut(u32) + Send + 'static>>;

/// Creates and returns a closure handling global registry object removals.
pub fn get_remove(index: Index, optimized: &OptimizedConfig) -> GlobalRemove {
    if optimized.optimized_sinks.unplugged_flag
        && !optimized.optimized_mic.unplugged_flag
        && !optimized.optimized_mic.unmuted_flag
        && !optimized.optimized_mic.muted_flag
    {
        return None;
    }

    Some(Box::new(move |id| {
        if let Ok(mut map) = index.lock()
            && let Some(info) = map.remove(&id)
        {
            let description = info.description;
            let icon = info.icon;

            match info.duty {
                Duty::UnpluggedSink => {
                    if let Err(error) = Notification::new()
                        .summary("Audio Sink")
                        .body(&format!("Device disconnected: {description}"))
                        .icon(&icon)
                        .show()
                    {
                        eprintln!("Notification error: {error}")
                    }
                }
                Duty::UnpluggedMic => {
                    if let Err(error) = Notification::new()
                        .summary("Microphone")
                        .body(&format!("Device disconnected: {description}"))
                        .icon(&icon)
                        .show()
                    {
                        eprintln!("Notification error: {error}")
                    }
                }
                Duty::Hidden => (),
            }
        }
    }))
}
