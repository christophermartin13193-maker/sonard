//! # Valid Configuration Module
//!
//! Validates raw configurations to ensure at least one active option or section is present.

use super::{Config, MicConfig, SinksConfig};
use sonar::SonarError;

// Represents a validated configuration structure.
pub struct ValidConfig {
    pub(super) sinks: Option<SinksConfig>,
    pub(super) mic: Option<MicConfig>,
}


impl SinksConfig {
    // Validates that at least one sink configuration property is populated.
    fn valid(self) -> Option<Self> {
        if let None = self.current
            && let None = self.plugged
            && let None = self.unplugged
        {
            None
        } else {
            Some(self)
        }
    }
}

impl MicConfig {
    // Validates that at least one microphone configuration property is populated.
    fn valid(self) -> Option<Self> {
        if let None = self.muted
            && let None = self.unmuted
            && let None = self.plugged
            && let None = self.unplugged
        {
            None
        } else {
            Some(self)
        }
    }
}

impl Config {
    // Validates the raw configuration and ensures it contains meaningful entries.
    pub fn get_valid_configuration(self) -> Result<ValidConfig, SonarError> {
        let sinks = match self.sinks {
            None => None,
            Some(sinks) => sinks.valid(),
        };
        let mic = match self.mic {
            None => None,
            Some(mic) => mic.valid(),
        };

        if sinks.is_none() && mic.is_none() {
            Err(SonarError::InvalidConfig)
        } else {
            Ok(ValidConfig { sinks, mic })
        }
    }
}
