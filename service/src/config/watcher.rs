use crate::config::parser::parse_configuration;
use crate::config::{ConfigurationError, ServiceConfiguration};
use log::{info, warn};
use notify_debouncer_full::notify::{RecommendedWatcher, RecursiveMode};
use notify_debouncer_full::{DebounceEventResult, Debouncer, RecommendedCache, new_debouncer};
use std::sync::{Arc, Mutex};
use std::time::Duration;
use tokio::fs::read_to_string;
use tokio_util::sync::CancellationToken;

pub struct ConfigurationWithWatcher {
    // pub config: ServiceConfiguration,
    path: String,
    debouncer: Option<Debouncer<RecommendedWatcher, RecommendedCache>>,
    tokens: Arc<Mutex<Vec<CancellationToken>>>,
}

impl ConfigurationWithWatcher {
    pub fn new(path: String) -> Self {
        let callbacks = Arc::new(Mutex::new(Vec::<CancellationToken>::new()));

        let handler = {
            let path = path.clone();
            let tokens = callbacks.clone();
            move |result: DebounceEventResult| match result {
                Ok(_) => {
                    info!("Configuration file '{path}' changed.");
                    let mut tokens = tokens.lock().unwrap();
                    for token in tokens.iter() {
                        token.cancel();
                    }
                    tokens.clear();
                }
                Err(errors) => errors
                    .iter()
                    .for_each(|error| warn!("Error while watching {path}: {error:?}")),
            }
        };

        let debouncer = match new_debouncer(Duration::from_secs(2), None, handler) {
            Ok(mut watcher) => match watcher.watch(path.clone(), RecursiveMode::NonRecursive) {
                Ok(_) => Some(watcher),
                Err(e) => {
                    warn!(
                        "Failed to watch '{path}', configuration will not be watched. Error: {e:?}"
                    );
                    None
                }
            },
            Err(e) => {
                warn!(
                    "Failed to create configuration watcher, configuration will not be watched. Error: {e:?}"
                );
                None
            }
        };

        ConfigurationWithWatcher {
            path,
            debouncer,
            tokens: callbacks,
        }
    }

    pub fn register_cancellation_token(&self, token: &CancellationToken) {
        let mut callbacks = self.tokens.lock().unwrap();
        callbacks.push(token.clone());
    }

    pub async fn read_configuration(&self) -> Result<ServiceConfiguration, ConfigurationError> {
        let toml = read_to_string(&self.path).await?;
        let config = parse_configuration(&toml)?;
        Ok(config)
    }
}

impl Drop for ConfigurationWithWatcher {
    fn drop(&mut self) {
        if let Some(debouncer) = self.debouncer.take() {
            debouncer.stop_nonblocking();
        }
    }
}
