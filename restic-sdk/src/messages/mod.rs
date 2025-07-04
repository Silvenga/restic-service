use crate::enum_union;
use serde::Deserialize;

mod backup;
mod exit_error;
mod version;
mod helpers;

pub use backup::*;
pub use exit_error::*;
pub use version::*;

enum_union! {
    #[derive(Deserialize, Debug)]
    #[serde(tag = "message_type")]
    pub enum ResticMessage {
        #[serde(rename = "version")]
        Version,
        #[serde(rename = "exit_error")]
        ExitError,
    }
}
