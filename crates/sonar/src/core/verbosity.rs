//! # Verbosity Module
//!
//! Manages verbosity levels and console message outputs
//! according to user preferences (silent mode, debug mode, etc.).

/// Represents the different verbosity levels available for the application.
#[derive(Debug, Clone, Copy)]
pub enum Verbosity {
    /// Default behavior: standard display of informational messages.
    Normal,
    /// Quiet mode: suppresses most text outputs.
    Quiet,
    /// Debug mode: enables verbose outputs and diagnostic information.
    Debug,
}

impl Verbosity {
    /// Prints a message only if the verbosity level is set to [Verbosity::Debug].
    pub fn dbg(&self, s: &str) {
        if let Self::Debug = self {
            println!("{s}")
        }
    }

    /// Prints a standard message, unless [Verbosity::Quiet] mode is enabled.
    pub fn msg(&self, s: &str) {
        match self {
            Self::Quiet => (),
            _ => println!("{s}"),
        }
    }

    /// Prints an error message to standard error (`stderr`),
    /// unless [Verbosity::Quiet] mode is enabled.
    pub fn err(&self, s: &str) {
        match self {
            Self::Quiet => (),
            _ => eprintln!("{s}"),
        }
    }

    /// Initializes the verbosity level based on command-line flags.
    pub fn init(debug: bool, quiet: bool) -> Self {
        match (debug, quiet) {
            (true, _) => Self::Debug,
            (_, true) => Self::Quiet,
            _ => Self::Normal,
        }
    }
}
