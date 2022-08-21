//! #norp

// modules
mod error;
mod location;

// internal dependencies
use crate::error::ProjectError;
use crate::error::ProjectError::*;
use crate::location::Location;

// external dependencies
use serde::Deserialize;
use serde_json;
use text_io::read;
use toml;
use uuid::Uuid;

// standard dependencies
use std::collections::HashMap;
use std::fs;
use std::path::*;

/// The runtime for the program called in main.rs
///
/// # Returns
/// A `Result` which is:
///
/// - `Ok`: Ran without issue
/// - `Err`: Encountered an issue
pub fn run() -> Result<(), ProjectError> {
    let config = Config::new(None)?;
    let mut state = State::new(&config)?;

    loop {
        // State present location
        state.get_current_location()?.print();
        // Read inputs or skip loop if none valid
        let input: String = read!("{}\n");
        let command = match Command::new(input.as_str()) {
            Err(project_error) => match project_error {
                CommandUnrecognizedError(_) => {
                    println!("{}", project_error);
                    continue;
                }
                _ => return Err(project_error),
            },
            Ok(command) => command,
        };
        // Execute command
        match command.execute(&mut state) {
            Err(project_error) => match project_error {
                MoveNoLocationError | LocationInvalidIdError => println!("{}", project_error),
                _ => return Err(project_error),
            },
            _ => (),
        };
    }
}

pub enum Command {
    Move(Vec<String>),
    AddLocation(Vec<String>),
}

impl Command {
    /// Returns a Command
    pub fn new(input: &str) -> Result<Command, ProjectError> {
        // Split by whitespace
        let mut input_iter = input.split(" ");
        // Separate out command identifier from the arguments
        let input_first = input_iter.next().ok_or_else(|| CommandNotGivenError)?;
        let input_rest: Vec<String> = input_iter.map(|s| String::from(s)).collect();
        // Match on command identifier and pass arguments
        match input_first {
            "move" => Ok(Command::Move(input_rest)),
            "add_location" => Ok(Command::AddLocation(input_rest)),
            _ => Err(CommandUnrecognizedError(String::from(input_first))),
        }
    }

    pub fn execute(&self, state: &mut State) -> Result<(), ProjectError> {
        match self {
            Command::Move(args) => {
                let mut id: Uuid =
                    Uuid::parse_str(args.get(0).ok_or_else(|| MoveNoLocationError)?.as_str())
                        .map_err(|_| LocationInvalidIdError)?;
                state.set_current_location(&mut id)
            }
            Command::AddLocation(args) => Err(StandardError),
        }
    }
}

/// Represents the configuration set in `data/norp.config.toml. Should contain all of the
/// configuration information necessary for the program to run.
#[derive(Deserialize)]
pub struct Config {
    /// The location of the JSON file containing all of the Location objects in a HashMap
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

    /// Checks if the Location corresponding to the provided `id` exists and sets
    /// `id_current_location` to `id` if so.
    pub fn set_current_location(&mut self, id: &mut Uuid) -> Result<(), ProjectError> {
        match self.check_location_exists(id) {
            true => {
                self.id_current_location = id.to_owned();
                Ok(())
            }
            false => Err(LocationNotFoundError),
        }
    }

    /// Checks if the provided `id` is an existent location in self.locations
    pub fn check_location_exists(&self, id: &Uuid) -> bool {
        self.locations.contains_key(&id)
    }

    /// Returns a reference to the current location
    pub fn get_current_location(&self) -> Result<&Location, ProjectError> {
        self.locations
            .get(&self.id_current_location)
            .ok_or_else(|| HashMapGetError(self.id_current_location.to_string()))
    }

    /// Returns a reference to the current location
    pub fn get_location(&self, id: &Uuid) -> Result<&Location, ProjectError> {
        self.locations
            .get(id)
            .ok_or_else(|| HashMapGetError(self.id_current_location.to_string()))
    }
}
