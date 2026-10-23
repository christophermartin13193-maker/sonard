//! # PipeWire Wrapper Module
//!
//! Initializes the PipeWire library and wraps core primitives (MainLoop, Context, Core, Registry)
//! into a unified structure for easier application integration.

use pipewire_native::{
    context::Context, core::Core, main_loop::MainLoop, properties::Properties,
    proxy::registry::Registry,
};
use sonar::SonarError;

// Encapsulates fundamental PipeWire handles required for client operations.
pub struct Wrapper(pub MainLoop, pub Context, pub Core, pub Registry);

impl Wrapper {
    /// Initializes PipeWire and establishes connection context and registry handles.
    pub fn new() -> Result<Self, SonarError> {
        pipewire_native::init();

        let v = vec![
            (String::from("application.name"), String::from("sonard")),
            (String::from("media.role"), String::from("notification")),
        ];

        let properties = Properties::new_vec(v);
        let main_loop = MainLoop::new(&properties).ok_or(SonarError::MainLoop)?;
        let context = match Context::new(&main_loop, Properties::new()) {
            Ok(context) => context,
            Err(error) => return Err(SonarError::Context(error)),
        };
        let core = match context.connect(Some(properties)) {
            Ok(core) => core,
            Err(error) => return Err(SonarError::Core(error)),
        };
        let registry = match core.registry() {
            Ok(registry) => registry,
            Err(error) => return Err(SonarError::Registry(error)),
        };

        Ok(Self(main_loop, context, core, registry))
    }
}
