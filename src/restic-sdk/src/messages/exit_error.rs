use serde::Deserialize;

/// An exit error message from restic.
/// This should be converted to a ResticError automatically, so this type exists for completeness.
#[derive(Deserialize, Debug, Clone, Eq, PartialEq)]
pub struct ExitError {
    /// Exit code, see [exit codes](https://restic.readthedocs.io/en/stable/075_scripting.html#exit-codes)
    pub code: i32,
    /// Error message
    pub message: String,
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::parsing::ResticMessage;
    use crate::restic_message;

    restic_message! {
        enum ResticExitMessage {
        }
    }

    #[test]
    fn can_parse() {
        let json = r#"{
            "message_type": "exit_error",
            "code": 1,
            "message": "An error occurred"
        }"#;
        let message = ResticExitMessage::parse_message(json).expect("parse should succeed");

        let result = ExitError::try_from(message).expect("should convert");
        assert_eq!(result.code, 1);
        assert_eq!(result.message, "An error occurred");
    }
}
