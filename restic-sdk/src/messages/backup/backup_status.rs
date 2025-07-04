use serde::Deserialize;

/// A backup status message from restic
#[derive(Deserialize, Debug, Clone, PartialEq)]
pub struct BackupStatus {
    /// Time since backup started
    #[serde(default)]
    pub seconds_elapsed: u64,
    /// Estimated time remaining
    #[serde(default)]
    pub seconds_remaining: u64,
    /// Fraction of data backed up (bytes_done/total_bytes)
    pub percent_done: f64,
    /// Total number of files detected
    pub total_files: u64,
    /// Files completed (backed up to repo)
    pub files_done: u64,
    /// Total number of bytes in backup set
    pub total_bytes: u64,
    /// Number of bytes completed (backed up to repo)
    pub bytes_done: u64,
    /// Number of errors
    #[serde(default)]
    pub error_count: u64,
    /// List of files currently being backed up
    #[serde(default)]
    pub current_files: Vec<String>,
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::messages::ResticBackupMessage;
    use crate::parsing::ResticMessage;

    #[test]
    fn can_parse() {
        let json = r#"{
            "message_type": "status",
            "seconds_elapsed": 120,
            "seconds_remaining": 300,
            "percent_done": 0.75,
            "total_files": 1000,
            "files_done": 750,
            "total_bytes": 5000000,
            "bytes_done": 3750000,
            "error_count": 0,
            "current_files": ["file1.txt", "file2.txt"]
        }"#;
        let message = ResticBackupMessage::parse_message(json).expect("parse should succeed");

        let result = BackupStatus::try_from(message).expect("should convert");
        assert_eq!(result.seconds_elapsed, 120);
        assert_eq!(result.seconds_remaining, 300);
        assert_eq!(result.percent_done, 0.75);
        assert_eq!(result.total_files, 1000);
        assert_eq!(result.files_done, 750);
        assert_eq!(result.total_bytes, 5000000);
        assert_eq!(result.bytes_done, 3750000);
        assert_eq!(result.error_count, 0);
        assert_eq!(result.current_files, vec!["file1.txt", "file2.txt"]);
    }

    #[test]
    fn can_parse_defaults() {
        let json = r#"{
          "message_type": "status",
          "percent_done": 1,
          "total_files": 1,
          "files_done": 1,
          "total_bytes": 16,
          "bytes_done": 16
        }"#;
        let message = ResticBackupMessage::parse_message(json).expect("parse should succeed");

        let result = BackupStatus::try_from(message).expect("should convert");
        assert_eq!(result.seconds_elapsed, 0);
        assert_eq!(result.seconds_remaining, 0);
        assert_eq!(result.error_count, 0);
    }
}
