//! #norp

// modules
mod error;

// internal dependencies
use crate::error::ProjectError;
// use crate::error::ProjectError::*;

// external dependencies
use text_io::read;

// Testing objects
fn location_home() -> Location {
    Location {
        name: String::from("Home"),
        description: String::from("This is your home, it has some nice lighting"),
    }
}

pub fn run() -> Result<(), ProjectError> {
    let this_state = &mut State {
        current_location: location_home(),
    };

    loop {
        this_state.current_location.print_location();
        let _i: String = read!();
    }
}

pub struct State {
    /// Current Location
    current_location: Location,
}

pub struct Location {
    /// Identifier for the Location
    name: String,
    /// Description of the Location
    description: String,
}

impl Location {
    pub fn print_location(&mut self) {
        println!("<--- {} --->", self.name);
        println!("{}", self.description);
    }
}
