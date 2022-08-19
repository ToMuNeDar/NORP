// internal dependencies
use crate::error::ProjectError;

// external dependencies
use serde::{Deserialize, Serialize};
use uuid::Uuid;

// standard dependencies
use std::fmt;

#[derive(Debug, Deserialize, Serialize)]
pub struct Location {
    pub id: Uuid,
    pub name: String,
    pub description: String,
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
