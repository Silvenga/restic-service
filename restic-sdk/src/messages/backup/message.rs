use super::{BackupError, BackupStatus, BackupSummary, BackupVerboseStatus};
use crate::messages::ExitError;
use crate::restic_message;

restic_message! {
    pub enum ResticBackupMessage {
        #[serde(rename = "status")]
        BackupStatus,
        #[serde(rename = "verbose_status")]
        BackupVerboseStatus,
        #[serde(rename = "summary")]
        BackupSummary,
        #[serde(rename = "error")]
        BackupError,
    }
}
