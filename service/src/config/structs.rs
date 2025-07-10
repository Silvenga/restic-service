use serde::{Deserialize, Serialize};
use std::collections::HashMap;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ServiceConfiguration {
    pub version: u32,
    #[serde(default)]
    pub jobs: HashMap<String, ResticJob>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ResticJob {
    /// Required
    pub cron: String,
    pub repository: String,
    pub password: String,

    // Optional
    #[serde(default)]
    pub environment: HashMap<String, String>,
    #[serde(default)]
    pub backup: BackupJobConfiguration,
    #[serde(default)]
    pub forget_and_purge: ForgetConfiguration,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(default)]
pub struct BackupJobConfiguration {
    pub use_fs_snapshot: bool,
    pub verbose: bool,
    pub exclude_caches: bool,
    pub source_fixed_drives: bool,
    pub sources: Vec<String>,
}

impl Default for BackupJobConfiguration {
    fn default() -> Self {
        BackupJobConfiguration {
            use_fs_snapshot: true,
            verbose: false,
            exclude_caches: false,
            source_fixed_drives: false,
            sources: Vec::default(),
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(default)]
pub struct ForgetConfiguration {
    pub enabled: bool,
    pub additional_flags: Vec<String>,
}
