use crate::errors::ResticError;
use crate::messages::{BackupSummary, ResticBackupMessage};
use crate::{CommandBuilder, Restic};
use log::{debug, warn};

impl Restic {
    pub async fn backup_with_options<'a>(
        &self,
        paths: impl IntoIterator<Item = &'a str>,
        options: BackupOptions<'a>,
    ) -> Result<BackupSummary, ResticError> {
        let arguments = options
            .builder
            .with_verb("backup")
            .with_values(paths.into_iter())
            .build();

        let mut summary: Option<BackupSummary> = None;

        self.exec_json(arguments, |message: ResticBackupMessage| match message {
            ResticBackupMessage::BackupSummary(message) => summary = Some(message),
            ResticBackupMessage::BackupStatus(_) => {
                debug!("Backup status: {message:?}");
            }
            ResticBackupMessage::BackupError(error) => {
                warn!("Error reported by restic during backup: {error:?}")
            }
            ResticBackupMessage::ExitError(error) => {
                warn!("Exit error reported by restic: {error:?}")
            }
            ResticBackupMessage::BackupVerboseStatus(_) => {
                // Ignored.
            }
        })
        .await?;

        match summary {
            Some(summary) => Ok(summary),
            None => Err(ResticError::UnexpectedResponse(
                "Backup did not return a summary".to_string(),
            )),
        }
    }

    pub async fn backup(
        &self,
        paths: impl IntoIterator<Item = &str>,
    ) -> Result<BackupSummary, ResticError> {
        self.backup_with_options(paths, BackupOptions::default())
            .await
    }
}

#[derive(Debug, Clone, Default)]
pub struct BackupOptions<'a> {
    builder: CommandBuilder<'a>,
}

impl<'a> BackupOptions<'a> {
    pub fn new() -> Self {
        Self::default()
    }

    /// Exclude other file systems, don't cross filesystem boundaries and subvolumes.
    pub fn with_one_file_system(mut self) -> Self {
        self.builder = self.builder.with_flag("one-file-system");
        self
    }

    /// Use filesystem snapshot where possible (currently only Windows VSS).
    pub fn use_fs_snapshot(mut self) -> Self {
        self.builder = self.builder.with_flag("use-fs-snapshot");
        self
    }

    /// Be verbose.
    pub fn with_verbose(mut self) -> Self {
        self.builder = self.builder.with_flag("verbose");
        self
    }

    /// Auto remove old cache directories.
    pub fn with_cleanup_cache(mut self) -> Self {
        self.builder = self.builder.with_flag("cleanup-cache");
        self
    }

    /// Excludes cache directories that are marked with a CACHEDIR.TAG file.
    /// See https://bford.info/cachedir/ for the Cache Directory Tagging Standard.
    pub fn with_exclude_caches(mut self) -> Self {
        self.builder = self.builder.with_flag("exclude-caches");
        self
    }

    pub fn with_flag_and_value(mut self, name: &'a str, value: &'a str) -> Self {
        self.builder = self.builder.with_flag_and_value(name, value);
        self
    }

    pub fn with_flag(mut self, name: &'a str) -> Self {
        self.builder = self.builder.with_flag(name);
        self
    }
}
