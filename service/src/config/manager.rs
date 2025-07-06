use crate::config::ServiceConfiguration;
use crate::config::parser::parse_configuration;
use log::debug;
use std::io;
use thiserror::Error;
use tokio::fs::{canonicalize, read_to_string, try_exists};

#[derive(Default, Debug, Clone)]
pub struct ServiceConfigurationManager {}

impl ServiceConfigurationManager {
    pub fn new() -> Self {
        ServiceConfigurationManager {}
    }

    pub async fn read_configuration(&self) -> Result<ServiceConfiguration, ConfigurationError> {
        let config_path = self.locate_configuration_file().await?;
        let toml = read_to_string(config_path).await?;
        let config = parse_configuration(&toml)?;
        Ok(config)
    }

    pub async fn locate_configuration_file(&self) -> Result<String, ConfigurationError> {
        let paths_to_probe = ["service_config.toml"];
        for path in &paths_to_probe {
            if try_exists(path).await? {
                let full_path = canonicalize(path).await?.to_str().unwrap().to_owned();
                debug!("Using configuration file found at '{full_path}'.");
                return Ok(full_path);
            }
        }
        Err(ConfigurationError::ConfigurationFileMissing)
    }
}

#[derive(Error, Debug)]
pub enum ConfigurationError {
    #[error("Failed to locate configuration file")]
    ConfigurationFileMissing,
    #[error("Failed to read configuration file: {0}")]
    IoError(#[from] io::Error),
    #[error("Failed to parse configuration: {0}")]
    ParseError(#[from] toml::de::Error),
}
