use std::io;
use thiserror::Error;

#[derive(Error, Debug)]
pub enum ResticError {
    #[error("failed to execute command {0}")]
    ExecuteFailure(#[from] io::Error),
}
