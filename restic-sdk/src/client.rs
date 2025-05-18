use crate::config::ResticConfig;
use std::fmt::Debug;

#[derive(Clone, Debug)]
pub struct Restic {
    pub(crate) config: ResticConfig,
}

impl Restic {
    pub fn new(config: ResticConfig) -> Self {
        Restic { config }
    }
}
