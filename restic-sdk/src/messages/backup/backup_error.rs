use serde::Deserialize;

/// A backup error message from restic (e.g. a file disappeared during backup)
#[derive(Deserialize, Debug, Clone, Eq, PartialEq)]
pub struct BackupError {
    /// Error message
    pub message: String,
    // What restic was trying to do
    pub during: String,
    /// Usually, the path of the problematic file
    pub item: String,
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::messages::ResticBackupMessage;
    use crate::parsing::ResticMessage;

    #[test]
    fn can_parse() {
        let json = r#"{
            "message_type": "error",
            "message": "Failed to open file",
            "during": "backup",
            "item": "/path/to/file.txt"
        }"#;
        let message = ResticBackupMessage::parse_message(json).expect("parse should succeed");

        let result = BackupError::try_from(message).expect("should convert");
        assert_eq!(result.message, "Failed to open file");
        assert_eq!(result.during, "backup");
        assert_eq!(result.item, "/path/to/file.txt");
    }
}
