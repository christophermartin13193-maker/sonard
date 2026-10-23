//! # Standard Configuration Module
//!
//! Handles file path discovery, TOML deserialization, and raw configuration structure mapping.

use serde::Deserialize;
use sonar::SonarError;
use std::path::PathBuf;

// Raw configuration structure mapped directly from the TOML file.
#[derive(Deserialize, Default)]
pub struct Config {
    pub(super) sinks: Option<SinksConfig>,
    pub(super) mic: Option<MicConfig>,
}

// Raw configuration schema for audio sinks.
#[derive(Deserialize, Default)]
pub struct SinksConfig {
    pub(super) current: Option<String>,
    pub(super) plugged: Option<String>,
    pub(super) unplugged: Option<String>,
}

// Raw configuration schema for microphones.
#[derive(Deserialize, Default)]
pub struct MicConfig {
    pub(super) unmuted: Option<String>,
    pub(super) muted: Option<String>,
    pub(super) plugged: Option<String>,
    pub(super) unplugged: Option<String>,
}

// Retrieves the primary local data configuration file path.
fn get_path_1() -> Option<PathBuf> {
    dirs::data_local_dir().map(|mut p| {
        p.push("sonard");
        p.push("config.toml");
        p
    })
}

// Retrieves the secondary standard user configuration file path.
fn get_path_2() -> Option<PathBuf> {
    dirs::config_dir().map(|mut p| {
        p.push("sonard");
        p.push("config.toml");
        p
    })
}

// Parses a configuration file from disk using TOML deserialization.
fn parse(path: PathBuf) -> Result<Config, SonarError> {
    Ok(toml::from_str(&std::fs::read_to_string(path)?)?)
}

// Searches standard XDG paths for a configuration file, reads it, and parses it.
pub fn get_config() -> Result<Config, SonarError> {
    let paths = [get_path_1(), get_path_2()];

    for path in paths {
        if let Some(path) = path
            && path.is_file()
        {
            return parse(path);
        }
    }
    Err(SonarError::NoConfigFile)
}
