//! # Sonard
//!
//! `sonard` is a lightweight Linux daemon built on top of [`pipewire-native`](https://docs.rs/pipewire-native) and [`pipewire-native-spa`](https://docs.rs/pipewire-native-spa) .
//! It monitors the state of sound cards and microphones (activation, mute, plug, unplug) in real time
//! and emits notification instructions.
//!
//! ## Architecture
//!
//! The core of the application relies on [`Automaton`], a finite state machine (FSM)
//! which orchestrates:
//!
//! 1. Resolution and loading of XDG configuration files.
//! 2. Connection to the PipeWire event loop via a C-API wrapper.
//! 3. An execution loop running without unnecessary heap allocations.
//!
//! ## Integration
//!
//! ```no_run
//! use sonar::Output;
//! use sonard::{Automaton};
//!
//! let mut auto = Automaton::default();
//! loop {
//!     match auto.run() {
//!         Ok(Output::Continue) => continue,
//!         Ok(Output::Exit(code)) => std::process::exit(code),
//!         Err(err) => std::process::exit(auto.handle_error(err)),
//!     }
//! }
//! ```

mod core;

// API
pub use core::Automaton;
