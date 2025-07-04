use crate::errors::ResticError;
use crate::extensions::errors::IntoResticError;
use crate::extensions::vec::VecHelpers;
use crate::messages::{ResticVersionMessage, Version};
use crate::{CommandBuilder, Restic};

impl Restic {
    /// Retrieves the version of the Restic client.
    pub async fn version(&self) -> Result<Version, ResticError> {
        let mut messages = Vec::new();

        let arguments = CommandBuilder::new().with_verb("version").build();

        self.exec_json(arguments, |message: ResticVersionMessage| {
            messages.push(message);
        })
        .await?;

        let message = messages.into_single().or_restic_processing_error()?;

        match Version::try_from(message) {
            Ok(result) => Ok(result),
            Err(error) => Err(ResticError::UnexpectedResponse(error)),
        }
    }
}
