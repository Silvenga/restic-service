use crate::errors::ResticError;
use crate::{ArgumentsBuilder, Restic};
use log::warn;
use tokio_util::sync::CancellationToken;

impl Restic {
    /// Attempts to unlock the Restic repository by removing stale locks.
    ///
    /// Performs `restic unlock`.
    pub async fn unlock(&self, cancellation_token: &CancellationToken) -> Result<(), ResticError> {
        self.exec(
            ArgumentsBuilder::new().with_verb("unlock"),
            |message, output_type| warn!("{output_type}: {message}"),
            cancellation_token,
        )
        .await?;
        Ok(())
    }
}
