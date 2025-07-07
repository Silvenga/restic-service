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
    pub source_fixed_drives: bool,
    #[serde(default)]
    pub sources: Vec<String>,
    #[serde(default)]
    pub environment: HashMap<String, String>,
    #[serde(default)]
    pub backup: BackupOptions,
    #[serde(default)]
    pub forget_and_purge: ForgetAndPurgeOptions,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(default)]
pub struct BackupOptions {
    pub use_fs_snapshot: bool,
    pub verbose: bool,
    pub exclude_caches: bool,
}

impl Default for BackupOptions {
    fn default() -> Self {
        BackupOptions {
            use_fs_snapshot: true,
            verbose: false,
            exclude_caches: false,
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(default)]
pub struct ForgetAndPurgeOptions {
    pub enabled: bool,
    pub additional_flags: Vec<String>,
}
