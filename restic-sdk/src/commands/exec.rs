use crate::Restic;
use crate::errors::ResticError;
use crate::parsing::{ResticMessage, parse_restic_message};
use log::warn;
use std::ffi::OsStr;
use std::io;
use std::process::{ExitStatus, Stdio};
use tokio::io::{AsyncBufReadExt, BufReader};
use tokio::process::Command;

impl Restic {
    pub(crate) async fn exec<I, S>(&self, arguments: I) -> Result<ExecStatus, ResticError>
    where
        I: IntoIterator<Item = S>,
        S: AsRef<OsStr>,
    {
        let start = async || -> Result<ExecStatus, io::Error> {
            let binary_path = Self::get_binary_path()?;
            let mut process = Command::new(binary_path)
                .args(arguments)
                .stdin(Stdio::null())
                .stdout(Stdio::piped())
                .stderr(Stdio::piped())
                .envs(self.config.environment.clone())
                .kill_on_drop(true)
                .spawn()?;

            let mut messages: Vec<ResticMessage> = Vec::new();
            let stdout = process.stdout.take().unwrap();
            let mut lines = BufReader::new(stdout).lines();
            while let Some(line) = lines.next_line().await? {
                match parse_restic_message(&line) {
                    Ok(e) => messages.push(e),
                    Err(e) => warn!("Failed to parse message: {:?}", e),
                }
            }

            let mut errors: Vec<ResticMessage> = Vec::new();
            let stderr = process.stderr.take().unwrap();
            let mut lines = BufReader::new(stderr).lines();
            while let Some(line) = lines.next_line().await? {
                match parse_restic_message(&line) {
                    Ok(e) => errors.push(e),
                    Err(e) => warn!("Failed to parse message: {:?}", e),
                }
            }

            let status = process.wait().await?;
            Ok(ExecStatus {
                status,
                messages,
                errors,
            })
        };

        match start().await {
            Ok(result) => Ok(result),
            Err(io_error) => Err(ResticError::ExecuteFailure(io_error)),
        }
    }

    fn get_binary_path() -> Result<String, std::io::Error> {
        Ok(String::from("restic"))
    }
}

pub struct ExecStatus {
    pub status: ExitStatus,
    pub messages: Vec<ResticMessage>,
    pub errors: Vec<ResticMessage>,
}
