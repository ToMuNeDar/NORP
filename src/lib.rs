//! #norp

// modules
mod error;

// internal dependencies
use crate::error::ProjectError;
use crate::error::ProjectError::*;

// external dependencies
use serde::{Deserialize, Serialize};
use serde_json;
use text_io::read;
use toml;
use uuid::Uuid;

// standard dependencies
use std::collections::HashMap;
use std::fmt;
use std::fs;
use std::path::*;

pub fn run() -> Result<(), ProjectError> {
    let config = Config::new(None)?;
    let state = State::new(&config)?;

    loop {
        state.get_current_location()?.print();
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
    id_current_location: Uuid,
    locations: HashMap<Uuid, Location>,
}

impl State<'_> {
    /// Returns a State object
    pub fn new<'a>(config: &'a Config) -> Result<State<'a>, ProjectError> {
        // Get owned file path
        let locations_file = &config.locations_file;
        // Read file
        let locations_data =
            fs::read_to_string(locations_file).map_err(|_| LocationsFileReadError)?;
        // Deserialize and store locations
        let locations: HashMap<Uuid, Location> =
            serde_json::from_str(&locations_data).map_err(|_| LocationsFileDeserializeError)?;
        let id_current_location = locations
            .keys()
            .next()
            .ok_or_else(|| HashMapFirstKeyError)?
            .to_owned();

        Ok(State::<'a> {
            config,
            id_current_location,
            locations,
        })
    }

    /// Saves locations to the file
    pub fn save(&self) -> Result<(), ProjectError> {
        // Get owned file path
        let locations_file = &self.config.locations_file;
        // Serialize locations
        let locations_data =
            serde_json::to_string(&self.locations).map_err(|_| LocationsFileSerializeError)?;
        // Write to file
        fs::write(locations_file, locations_data).map_err(|_| LocationsFileWriteError)
    }

    /// Adds a new location to locations
    pub fn add_location(&mut self, location: Location) -> Result<(), ProjectError> {
        // Insert into HashMap
        self.locations.insert(location.id, location);
        Ok(())
    }

    /// Returns a reference to the current location
    pub fn get_current_location(&self) -> Result<&Location, ProjectError> {
        self.locations
            .get(&self.id_current_location)
            .ok_or_else(|| HashMapGetError(self.id_current_location.to_string()))
    }
}

#[derive(Debug, Deserialize, Serialize)]
pub struct Location {
    id: Uuid,
    name: String,
    description: String,
}

impl Location {
    const DEFAULT_DESCRIPTION: &'static str = "A new location.";

    pub fn new(name: &str, description: Option<&str>) -> Result<Location, ProjectError> {
        let id = Uuid::new_v4();
        let name = String::from(name);
        let description =
            String::from(description.unwrap_or_else(|| Location::DEFAULT_DESCRIPTION));

        Ok(Location {
            id,
            name,
            description,
        })
    }

    pub fn print(&self) {
        println!("{}", self);
    }
}

impl fmt::Display for Location {
    fn fmt(&self, fmt: &mut fmt::Formatter) -> fmt::Result {
        let s = format!("<---{}--->\n{}", self.name, self.description);
        fmt.write_str(s.as_str())?;
        Ok(())
    }
}
