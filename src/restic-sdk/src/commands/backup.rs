use crate::errors::ResticError;
use crate::messages::{BackupSummary, ResticBackupMessage};
use crate::{ArgumentsBuilder, BuilderValue, Restic};
use log::{debug, warn};
use tokio_util::sync::CancellationToken;

impl Restic {
    pub async fn backup(
        &self,
        paths: impl IntoIterator<Item = impl Into<String>>,
        options: BackupOptions,
        cancellation_token: &CancellationToken,
    ) -> Result<BackupResult, ResticError> {
        let arguments = options
            .builder
            .with_values(paths);

        let mut summary: Option<BackupSummary> = None;

        let result = self
            .exec_json(
                arguments,
                |message: ResticBackupMessage| match message {
                    ResticBackupMessage::BackupSummary(message) => summary = Some(message),
                    ResticBackupMessage::BackupStatus(_) => {
                        debug!("Backup status: {message:?}");
                    }
                    ResticBackupMessage::BackupError(error) => {
                        debug!("Backup error: {error}")
                    }
                    ResticBackupMessage::ExitError(error) => {
                        warn!(
                            "Restic will exit with: {error} (code: {code})",
                            error = error.message,
                            code = error.code
                        );
                    }
                    ResticBackupMessage::BackupVerboseStatus(_) => {
                        // Ignored.
                    }
                },
                cancellation_token,
            )
            .await;

        match (result, summary) {
            (Ok(_), Some(summary)) => Ok(BackupResult {
                failed_to_read_some_data: false,
                summary,
            }),
            (Err(ResticError::BackupFailedToReadSomeSourceData), Some(summary)) => {
                Ok(BackupResult {
                    failed_to_read_some_data: true,
                    summary,
                })
            }
            (Err(e), _) => Err(e),
            (Ok(_), None) => Err(ResticError::UnexpectedResponse(
                "Backup did not return a summary".to_string(),
            )),
        }
    }
}

#[derive(Debug, Clone)]
pub struct BackupResult {
    /// Non-fatal error that denotes that one or more files could not be read during backup.
    pub failed_to_read_some_data: bool,

    /// The summary of the backup operation.
    pub summary: BackupSummary,
}

#[derive(Debug, Clone)]
pub struct BackupOptions {
    builder: ArgumentsBuilder,
}

impl Default for BackupOptions {
    fn default() -> Self {
        Self {
            builder: ArgumentsBuilder::new().with_verb("backup"),
        }
    }
}

impl BackupOptions {
    pub fn new() -> Self {
        Self::default()
    }

    /// Exclude other file systems, don't cross filesystem boundaries and subvolumes.
    /// Not supported on Windows.
    #[cfg(not(windows))]
    pub fn with_one_file_system(self) -> Self {
        self.with_flag("one-file-system")
    }

    /// Use filesystem snapshot where possible (currently only Windows VSS).
    pub fn use_fs_snapshot(self) -> Self {
        self.with_flag("use-fs-snapshot")
    }

    /// Be verbose.
    pub fn with_verbose(self) -> Self {
        self.with_flag("verbose")
    }

    /// Auto remove old cache directories.
    pub fn with_cleanup_cache(self) -> Self {
        self.with_flag("cleanup-cache")
    }

    /// Excludes cache directories that are marked with a CACHEDIR.TAG file.
    /// See https://bford.info/cachedir/ for the Cache Directory Tagging Standard.
    pub fn with_exclude_caches(self) -> Self {
        self.with_flag("exclude-caches")
    }

    pub fn with_flag(mut self, name: &str) -> Self {
        self.builder = self.builder.with_flag(name);
        self
    }

    pub fn with_flag_and_value<V: BuilderValue>(mut self, name: &str, value: V) -> Self {
        self.builder = self.builder.with_flag_and_value(name, value);
        self
    }
}
