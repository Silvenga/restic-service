use crate::config::ResticConfig;
use std::fmt::Debug;

#[derive(Clone, Debug, Default)]
pub struct Restic {
    pub(crate) config: ResticConfig,
}

impl Restic {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn with_config(mut self, config: ResticConfig) -> Self {
        self.config = config;
        self
    }
}
