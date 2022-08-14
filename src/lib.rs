//! #norp

// modules
mod error;

// internal dependencies
use crate::error::ProjectError;
use crate::error::ProjectError::*;

// external dependencies
use serde::Deserialize;
use serde_json;
use text_io::read;
use toml;

// standard dependencies
use std::collections::HashMap;
use std::fs;
use std::path::*;

pub fn run() -> Result<(), ProjectError> {
    let config = Config::new(None)?;
    let mut state = State {
        config: &config,
        location_current: 1,
        locations: HashMap::new(),
    };

    state.add_location("home", 1)?;
    state.save()?;

    loop {
        // this_state.current_location.print_location();
        let _i: String = read!();
    }
}

/// Configuration object
#[derive(Deserialize)]
pub struct Config {
    locations_file: PathBuf,
}

impl Config {
    /// Returns the default config file path
    fn config_file_default() -> &'static Path {
        Path::new("./data/norp.config.toml")
    }

    /// Returns a Config object
    pub fn new(config_file: Option<&Path>) -> Result<Config, ProjectError> {
        // Unwrap file path and use defaults if needed
        let config_file = config_file.unwrap_or_else(|| Config::config_file_default());
        // Read file
        let config_data = fs::read_to_string(config_file).map_err(|_| StandardError)?;
        // Return type inferred results
        Ok(toml::from_str(&config_data).map_err(|_| StandardError)?)
    }
}

/// State object
pub struct State<'a> {
    config: &'a Config,
    location_current: u8,
    locations: HashMap<String, u8>,
}

impl State<'_> {
    /// Returns a State object
    pub fn new<'a>(config: &'a Config) -> Result<State<'a>, ProjectError> {
        // Get owned file path
        let locations_file = &config.locations_file;
        // Read file
        let locations_data = fs::read_to_string(locations_file).map_err(|_| StandardError)?;
        // Deserialize and store locations
        let locations = serde_json::from_str(&locations_data).map_err(|_| StandardError)?;
        let location_current: u8 = 1;

        Ok(State::<'a> {
            config: config,
            location_current,
            locations,
        })
    }

    /// Saves locations to the file
    pub fn save(&mut self) -> Result<(), ProjectError> {
        // Get owned file path
        let locations_file = &self.config.locations_file;
        // Serialize locations
        let locations_data = serde_json::to_string(&self.locations).map_err(|_| StandardError)?;
        // Write to file
        fs::write(locations_file, locations_data).map_err(|_| StandardError)
    }

    pub fn add_location(
        &mut self,
        location_name: &str,
        location_value: u8,
    ) -> Result<(), ProjectError> {
        // Insert into HashMap
        self.locations
            .insert(String::from(location_name), location_value);
        Ok(())
    }
}

// scratch notes:

// pub struct Config
//	* reads from norp_config.toml
//	* holds file_locations

// pub struct State
//	* reads from locations.json
//	* received a Config on construction
//	* holds location_current
//	* holds locations

// pub struct Location
//	* holds a GUID
//	* holds a name
//	* holds a description
//	* has a to_string() method
