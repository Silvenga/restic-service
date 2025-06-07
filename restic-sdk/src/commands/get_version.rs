use crate::extensions::errors::IntoResticError;
use crate::extensions::vec::VecHelpers;
use crate::parsing::ResticMessage;
use crate::{Restic, ResticError};

impl Restic {
    pub async fn get_version(&self) -> Result<ResticVersion, ResticError> {
        let mut messages = Vec::new();

        self.exec(["version", "--json"], |message: ResticMessage| {
            messages.push(message);
        })
        .await?;

        let message = messages.into_single().or_restic_processing_error()?;

        if let ResticMessage::Version {
            version,
            go_version,
            go_os,
            go_arch,
            ..
        } = message
        {
            Ok(ResticVersion {
                version,
                go_version,
                go_os,
                go_arch,
            })
        } else {
            Err(ResticError::UnexpectedResponse(
                "Expected version message".to_string(),
            ))
        }
    }
}

#[derive(Clone, Debug)]
pub struct ResticVersion {
    pub version: String,
    pub go_version: String,
    pub go_os: String,
    pub go_arch: String,
}
