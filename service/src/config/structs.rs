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
    pub cleanup_cache: bool,
    #[cfg(not(windows))]
    pub one_file_system: bool,
    pub additional_flags: Vec<String>,
}

impl Default for BackupJobConfiguration {
    fn default() -> Self {
        BackupJobConfiguration {
            use_fs_snapshot: true,
            verbose: false,
            exclude_caches: false,
            source_fixed_drives: false,
            sources: Vec::default(),
            cleanup_cache: false,
            #[cfg(not(windows))]
            one_file_system: false,
            additional_flags: Vec::default(),
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(default)]
pub struct ForgetConfiguration {
    pub enabled: bool,
    pub additional_flags: Vec<String>,

    // Retention policy options
    pub group_by: Option<String>,
    pub keep_last: Option<u32>,
    pub keep_hourly: Option<u32>,
    pub keep_daily: Option<u32>,
    pub keep_weekly: Option<u32>,
    pub keep_monthly: Option<u32>,
    pub keep_yearly: Option<u32>,
    pub keep_within: Option<String>,
    pub keep_within_hourly: Option<u32>,
    pub keep_within_daily: Option<u32>,
    pub keep_within_weekly: Option<u32>,
    pub keep_within_monthly: Option<u32>,
    pub keep_within_yearly: Option<u32>,
    pub keep_tag: Option<String>,

    // Filtering options
    pub host: Option<String>,
    pub tag: Option<String>,
    pub path: Option<String>,

    // Behavior options
    pub unsafe_allow_remove_all: bool,
    pub compact: bool,
    pub dry_run: bool,
    pub prune: bool,

    // Prune-specific options
    pub max_unused: Option<String>,
    pub max_repack_size: Option<String>,
    pub repack_cacheable_only: bool,
    pub repack_small: bool,
    pub repack_uncompressed: bool,
    pub repack_smaller_than: Option<String>,
}
