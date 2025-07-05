use serde::{Deserialize, Serialize};
use serde_inline_default::serde_inline_default;
use std::collections::HashMap;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ServiceConfiguration {
    version: u32,
    jobs: HashMap<String, ResticJob>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ResticJob {
    pub cron: String,
    pub sources: Vec<String>,
    pub repository: Repository,
    #[serde(default)]
    pub environment: HashMap<String, String>,
    pub backup: BackupOptions,
    pub forget_and_purge: ForgetAndPurgeOptions,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Repository {
    pub url: String,
    pub password: String,
}

#[serde_inline_default]
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BackupOptions {
    #[serde_inline_default(true)]
    pub one_file_system: bool,

    #[serde_inline_default(true)]
    pub use_fs_snapshot: bool,

    #[serde_inline_default(false)]
    pub verbose: bool,

    #[serde_inline_default(false)]
    pub exclude_caches: bool,

    #[serde(default)]
    pub additional_flags: Vec<String>,
}

#[serde_inline_default]
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ForgetAndPurgeOptions {
    #[serde_inline_default(false)]
    pub enabled: bool,

    #[serde(default)]
    pub additional_flags: Vec<String>,
}
