// external dependencies
use thiserror::Error; 

#[derive(Error, Debug)]
pub enum ProjectError {
	#[error("Standard Error encountered.")]
	StandardError,
}