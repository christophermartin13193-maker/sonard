//! # Control Module
//!
//! Manages global application errors, exit codes, as well as
//! the main program control flows.

/// Represents all critical errors that can occur
/// during the execution of `sonard`.
pub enum SonarError {
    /// No configuration file could be found.
    NoConfigFile,
    /// Standard Input/Output (I/O) error.
    IO(std::io::Error),
    /// Error encountered while parsing the TOML configuration file.
    Toml(toml::de::Error),
    /// The provided configuration file is invalid.
    InvalidConfig,
    /// Failure while creating the MainLoop structure.
    MainLoop,
    /// Failure while initializing the PipeWire context.
    Context(std::io::Error),
    /// Failure while connecting the PipeWire context to the daemon.
    Core(std::io::Error),
    /// Failure while creating the PipeWire registry.
    Registry(std::io::Error),
    /// Failure while binding the node to the registry.
    Bind,
}

/// Determines the behavior to adopt following an operation or command.
pub enum Output {
    /// Continue normal program execution.
    Continue,
    /// Exit the process with a specific exit code.
    Exit(code::ExitCode),
}

/// Groups types and constants related to process exit codes.
pub mod code {
    /// Numerical type representing a process exit code.
    pub type ExitCode = i32;

    /// Successful program termination (exit code 0).
    pub const STANDARD: ExitCode = 0;
    /// Exit code indicating a configuration-related error (exit code 1)
    pub const CONFIG: ExitCode = 1;
    /// Exit code indicating a PipeWire-related error (exit code 2)
    pub const PIPEWIRE: ExitCode = 2;
}

impl From<std::io::Error> for SonarError {
    fn from(err: std::io::Error) -> Self {
        Self::IO(err)
    }
}
impl From<toml::de::Error> for SonarError {
    fn from(err: toml::de::Error) -> Self {
        Self::Toml(err)
    }
}

impl std::fmt::Display for SonarError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::NoConfigFile => writeln!(f, "Unable to find a configuration file"),
            Self::Toml(error) => writeln!(
                f,
                "Encountered an error during the parsing of the configuration file: {error}"
            ),
            Self::IO(error) => writeln!(
                f,
                "Encountered an error trying to retrieve the configuration file's informations: {error}"
            ),
            Self::InvalidConfig => writeln!(f, "Invalid configuration file"),
            Self::MainLoop => {
                writeln!(f, "Couldn't create MainLoop structure from pipewire_native")
            }
            Self::Context(error) => writeln!(
                f,
                "Couldn't create Context structure from pipewire_native and got the error: {error}"
            ),
            Self::Core(error) => writeln!(
                f,
                "Couldn't connect the Context structure from pipewire_native and got the error: {error}"
            ),
            Self::Registry(error) => writeln!(
                f,
                "Couldn't create Registry structure from pipewire_native and got the error: {error}"
            ),
            Self::Bind => writeln!(f, "Couldn't bind the node to the registry"),
        }
    }
}
