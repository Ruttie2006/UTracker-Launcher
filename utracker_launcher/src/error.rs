use crate::error;
use std::process::exit;

#[derive(Debug, thiserror::Error)]
pub enum Error {
    #[error("Failed an I/O call: {0}")]
    IO(#[from] std::io::Error),
    #[error("Failed to deserialize a .yaml file: {0}")]
    Yaml(#[from] serde_yml::Error),
    #[error("Failed to prompt user for value: {0}")]
    Prompt(#[from] dialoguer::Error),
    #[error("Failed to deserialize the config file: {0}")]
    Toml(#[from] toml::de::Error),
    #[error("Something is wrong with your setup: {0}")]
    Validation(ValidationError),
}

#[derive(Debug, thiserror::Error)]
#[allow(clippy::enum_variant_names)]
#[non_exhaustive]
pub enum ValidationError {
    #[error("Could not find a '{0}' directory", crate::consts::PLAYERS_DIR)]
    NoPlayerDir,
    #[error("Found no players, please make sure at least one player is present")]
    NoPlayers,
    #[error("Could not find the {0} file", crate::consts::LAUNCHER_NAME)]
    NoArchipelago,
}

impl Error {
    pub fn consume(self) -> ! {
        error!("{}", self);

        exit(match self {
            Self::IO(_) => 1,
            Self::Yaml(_) => 2,
            Self::Prompt(_) => 3,
            Self::Toml(_) => 4,
            Self::Validation(_) => 5,
        })
    }
}
