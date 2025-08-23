use crate::config::watcher::ConfigurationWithWatcher;
use log::{debug, warn};
use std::path::{Path, PathBuf};
use std::{env, io, path};
use thiserror::Error;
use tokio::fs::{canonicalize, try_exists};

#[derive(Default, Debug, Clone)]
pub struct ServiceConfigurationManager {}

impl ServiceConfigurationManager {
    pub fn new() -> Self {
        ServiceConfigurationManager {}
    }

    pub async fn watch_configuration(
        &self,
    ) -> Result<ConfigurationWithWatcher, ConfigurationError> {
        let config_path = self.locate_configuration_file().await?;
        Ok(ConfigurationWithWatcher::new(config_path))
    }

    // pub async fn read_configuration(&self) -> Result<ServiceConfiguration, ConfigurationError> {
    //     let config_path = self.locate_configuration_file().await?;
    //     let toml = read_to_string(config_path).await?;
    //     let config = parse_configuration(&toml)?;
    //     Ok(config)
    // }

    pub async fn locate_configuration_file(&self) -> Result<String, ConfigurationError> {
        let paths = self.get_config_paths();

        for path in paths.iter() {
            if try_exists(&path).await? {
                let full_path = canonicalize(&path).await?.to_str().unwrap().to_owned();
                debug!("Using configuration file found at '{full_path}'.");
                return Ok(full_path);
            }
            debug!(
                "No configuration found for '{}'.",
                path.to_str().unwrap_or("<invalid path>")
            );
        }

        // Failure path:
        let mut resolved_paths = Vec::with_capacity(paths.len());
        for path in paths.into_iter() {
            let path = path::absolute(path)
                .unwrap_or("<failed to resolve>".into())
                .to_str()
                .unwrap_or("<failed to convert>")
                .to_owned();
            resolved_paths.push(path);
        }
        let resolved_paths = resolved_paths.join(", ");
        warn!("No configuration in [{resolved_paths}].");

        Err(ConfigurationError::ConfigurationFileMissing)
    }

    fn get_config_paths(&self) -> Vec<PathBuf> {
        let current_exe = &env::current_exe().expect("current_exe should return a valid path");
        let exe_dir = current_exe
            .parent()
            .expect("current_exe should be in a directory");

        let current_dir = &env::current_dir().expect("current_dir should return a valid path");

        vec![
            Path::join(exe_dir, "service_config.toml"),
            Path::join(current_dir, "service_config.toml"),
        ]
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
