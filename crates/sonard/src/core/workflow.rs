//! # Workflow Module
//!
//! Implements the core finite state machine (`Automaton`) that drives
//! the daemon lifecycle, initialization, event loop, and error handling.

use super::{GlobalRemoveInfo, Wrapper, get_config, get_global, get_remove};
use super::{MiniConfig, OptimizedConfig};
use pipewire_native::proxy::registry::RegistryEvents;
use sonar::code::{self, ExitCode};
use sonar::{Output, SonarError, Verbosity};
use std::{
    collections::HashMap,
    sync::{Arc, Mutex},
};

// Represents the internal states of the finite state machine.
enum Step {
    Init,
    Loop(OptimizedConfig, MiniConfig),
    Exit,
}

/// Finite state machine managing the daemon lifecycle and execution loop.
pub struct Automaton {
    step: Step,
    v: Verbosity,
}

impl Automaton {
    /// Creates a new instance of [Automaton] set to the initial state.
    pub fn new() -> Self {
        Automaton {
            step: Step::Init,
            v: Verbosity::Normal,
        }
    }

    /// Executes the current step of the automaton and transitions to the next one.
    pub fn run(&mut self) -> Result<Output, SonarError> {
        let current = std::mem::replace(&mut self.step, Step::Exit);
        self.step = match current {
            Step::Init => init()?,
            Step::Loop(opti, mini) => loop_(opti, mini)?,
            Step::Exit => return Ok(Output::Exit(code::STANDARD)),
        };
        Ok(Output::Continue)
    }

    /// Maps application errors to standardized process exit codes.
    pub fn handle_error(&self, err: SonarError) -> ExitCode {
        self.v.err(&err.to_string());
        match err {
            SonarError::Bind
            | SonarError::Context(_)
            | SonarError::Core(_)
            | SonarError::Registry(_)
            | SonarError::MainLoop => code::PIPEWIRE,
            SonarError::IO(_)
            | SonarError::Toml(_)
            | SonarError::InvalidConfig
            | SonarError::NoConfigFile => code::CONFIG,
        }
    }
}

impl Default for Automaton {
    fn default() -> Self {
        Self::new()
    }
}

// Initializes configurations and prepares the loop state.
fn init() -> Result<Step, SonarError> {
    let configuration = get_config()?;
    let valid_configuration = configuration.get_valid_configuration()?;

    let optimized_configuration = valid_configuration.get_optimized_configuration();
    let minimized_configuration = optimized_configuration.get_minimized_configuration();

    Ok(Step::Loop(optimized_configuration, minimized_configuration))
}

// Runs the main PipeWire event loop and listens for registry events.
fn loop_(optimized: OptimizedConfig, minimized: MiniConfig) -> Result<Step, SonarError> {
    let Wrapper(main_loop, _context, _core, registry) = Wrapper::new()?;

    let index: HashMap<u32, GlobalRemoveInfo> = HashMap::new();
    let index = Arc::new(Mutex::new(index));

    let remove = get_remove(Arc::clone(&index), &optimized);

    let optimized = Arc::new(optimized);
    let minimized = Arc::new(minimized);
    let registry = Arc::new(registry);

    let global = get_global(
        Arc::clone(&optimized),
        Arc::clone(&minimized),
        Arc::clone(&index),
        Arc::clone(&registry),
    );

    registry.add_listener(RegistryEvents {
        global,
        global_remove: remove,
    });

    main_loop.run();

    Ok(Step::Exit)
}
