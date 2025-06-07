use crate::parsing::ParseError;
use serde::Deserialize;

pub fn parse_restic_message(message: &str) -> Result<ResticMessage, ParseError> {
    serde_json::from_str(message).map_err(|e| ParseError::SerdeError(e))
}

#[derive(Deserialize, Debug)]
#[serde(tag = "message_type")]
pub enum ResticMessage {
    #[serde(rename = "version")]
    Version {
        version: String,
        go_version: String,
        go_os: String,
        go_arch: String,
    },
    #[serde(rename = "exit_error")]
    ExitError {
        // code: i32, message: String
    },
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::parsing::ParseError;

    #[test]
    fn type_version() -> Result<(), ParseError> {
        let json = r#"{
            "message_type": "version",
            "version": "0.12.0",
            "go_version": "go1.20.3",
            "go_os": "linux",
            "go_arch": "amd64"
        }"#;
        let result = parse_restic_message(json)?;

        if let ResticMessage::Version {
            version,
            go_version,
            go_os,
            go_arch,
        } = result
        {
            assert_eq!(version, "0.12.0");
            assert_eq!(go_version, "go1.20.3");
            assert_eq!(go_os, "linux");
            assert_eq!(go_arch, "amd64");

            Ok(())
        } else {
            panic!("Expected Version message");
        }
    }

    #[test]
    fn type_exist_code() -> Result<(), ParseError> {
        let json = r#"{
            "message_type": "exit_error",
            "code": 1,
            "message": "An error occurred"
        }"#;
        let result = parse_restic_message(json)?;

        if let ResticMessage::ExitError {} = result {
            Ok(())
        } else {
            panic!("Expected Version message");
        }
    }
}
