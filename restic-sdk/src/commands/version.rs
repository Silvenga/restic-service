use crate::errors::ResticError;
use crate::extensions::errors::IntoResticError;
use crate::extensions::vec::VecHelpers;
use crate::messages::{ResticVersionMessage, Version};
use crate::{ArgumentsBuilder, Restic};
use tokio_util::sync::CancellationToken;

impl Restic {
    /// Retrieves the version of the Restic client.
    pub async fn version(
        &self,
        cancellation_token: &CancellationToken,
    ) -> Result<Version, ResticError> {
        let mut messages = Vec::new();
        self.exec_json(
            ArgumentsBuilder::new().with_verb("version"),
            |message: ResticVersionMessage| {
                messages.push(message);
            },
            cancellation_token,
        )
        .await?;

        let message = messages.into_single().or_restic_processing_error()?;

        match Version::try_from(message) {
            Ok(result) => Ok(result),
            Err(error) => Err(ResticError::UnexpectedResponse(error)),
        }
    }
}
