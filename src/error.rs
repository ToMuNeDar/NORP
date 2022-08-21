// external dependencies
use thiserror::Error;

#[derive(Error, Debug)]
pub enum ProjectError {
    #[error("Standard Error encountered.")]
    StandardError,
    #[error("Failed to read locations file.")]
    LocationsFileReadError,
    #[error("Failed to write locations file.")]
    LocationsFileWriteError,
    #[error("Failed to deserialize locations file.")]
    LocationsFileDeserializeError,
    #[error("Failed to serialize locations HashMap.")]
    LocationsFileSerializeError,
    #[error("Failed to retrieve value from key {0} in HashMap.")]
    HashMapGetError(String),
    #[error("Failed to retrieve first key from HashMap.")]
    HashMapFirstKeyError,
    #[error("Command not provided.")]
    CommandNotGivenError,
    #[error("Unrecognized command: \"{0}\".")]
    CommandUnrecognizedError(String),
}
