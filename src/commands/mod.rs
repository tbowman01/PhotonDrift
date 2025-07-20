pub mod diff;
pub mod index;
pub mod init;
pub mod inventory;
pub mod propose;

use crate::{config::Config, error::AdrscanError};
type Result<T> = std::result::Result<T, AdrscanError>;

/// Trait for command execution
pub trait Command {
    fn execute(&self, config: &Config) -> Result<()>;
}