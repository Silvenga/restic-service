use std::io;
use thiserror::Error;

#[derive(Error, Debug)]
pub enum ResticError {
    // Exec errors.
    #[error("failed to execute restic {0}")]
    FailedToExecute(#[from] io::Error),
    #[error("restic was killed by signal")]
    Killed,
    // Real errors from restic.
    #[error("restic exit(1): Command failed, see command help for more details")]
    GenericError,
    #[error("restic exit(2): Go runtime error")]
    GoRuntimeError,
    #[error("restic exit(3): Backup command could not read some source data")]
    BackupFailedToReadSomeSourceData,
    #[error("restic exit(10): Repository does not exist")]
    RepositoryDoesNotExist,
    #[error("restic exit(11): Failed to lock repository")]
    FailedToLockRepository,
    #[error("restic exit(12): Wrong password")]
    WrongPassword,
    #[error("restic exit(130): Restic was interrupted using SIGINT or SIGSTOP")]
    Interrupted,
    #[error("restic exit({0}): Unexpected exit code")]
    UnexpectedExitCode(i32),
    // Handling errors (after successful results from restic).
    #[error(transparent)]
    ErrorDuringProcessing(#[from] Box<dyn std::error::Error + Send + Sync>),
    #[error("unexpected response from restic: {0}")]
    UnexpectedResponse(String),
}

pub(crate) fn map_exit_code_to_error(code: i32) -> Result<(), ResticError> {
    match code {
        0 => Ok(()),
        1 => Err(ResticError::GenericError),
        2 => Err(ResticError::GoRuntimeError),
        3 => Err(ResticError::BackupFailedToReadSomeSourceData),
        10 => Err(ResticError::RepositoryDoesNotExist),
        11 => Err(ResticError::FailedToLockRepository),
        12 => Err(ResticError::WrongPassword),
        130 => Err(ResticError::Interrupted),
        _ => Err(ResticError::UnexpectedExitCode(code)),
    }
}
