use crate::errors::{ResticError, map_exit_code_to_error};
use crate::parsing::ResticMessage;
use crate::{ArgumentsBuilder, Restic};
use log::{debug, warn};
use pathsearch::find_executable_in_path;
use std::ffi::OsString;
use std::fmt::Display;
use std::io;
use std::process::{ExitStatus, Stdio};
use tokio::io::{AsyncBufReadExt, BufReader};
use tokio::process::Command;
use tokio_util::sync::CancellationToken;

impl Restic {
    /// Low-level command execution method that allows for custom handling of output messages.
    pub(crate) async fn exec<F>(
        &self,
        arguments: ArgumentsBuilder,
        mut on_message: F,
        cancellation_token: &CancellationToken,
    ) -> Result<(), ResticError>
    where
        F: FnMut(String, MessageOutputType),
    {
        let start = async || -> Result<ExitStatus, io::Error> {
            let binary_path = Self::get_binary_path()?;
            let mut process = Command::new(binary_path)
                .args(arguments.build())
                .stdin(Stdio::null())
                .stdout(Stdio::piped())
                .stderr(Stdio::piped())
                .envs(self.config.environment.clone())
                .kill_on_drop(true)
                .spawn()?;

            let stdout = process.stdout.take().unwrap();
            let stderr = process.stderr.take().unwrap();
            let mut stdout_lines = BufReader::new(stdout).lines();
            let mut stderr_lines = BufReader::new(stderr).lines();
            let mut stdout_complete = false;
            let mut stderr_complete = false;

            loop {
                tokio::select! {
                    stdout_line = stdout_lines.next_line(), if !stdout_complete => {
                        match stdout_line? {
                            None => {
                                stdout_complete = true;
                                continue;
                            }
                            Some(line) => on_message(line, MessageOutputType::Stdout),
                        }
                    },
                    stderr_lines = stderr_lines.next_line(), if !stderr_complete => {
                        match stderr_lines? {
                            None => {
                                stderr_complete = true;
                                continue;
                            }
                            Some(line) => on_message(line, MessageOutputType::Stderr),
                        }
                    },
                    _ = cancellation_token.cancelled(), if !stderr_complete && !stdout_complete => {
                        debug!("Cancellation token triggered, killing process.");
                        process.start_kill()?;
                        break;
                    },
                    else => {
                        break;
                    }
                }
            }

            let status = process.wait().await?;
            Ok(status)
        };

        let status = start().await.map_err(ResticError::FailedToExecute)?;
        let code = status.code().ok_or(ResticError::Killed)?;

        match map_exit_code_to_error(code) {
            Ok(_) => Ok(()),
            Err(e) => Err(e),
        }
    }

    fn get_binary_path() -> Result<OsString, io::Error> {
        let Some(exe) = find_executable_in_path("restic") else {
            return Err(io::Error::new(
                io::ErrorKind::NotFound,
                "The restic binary not found in PATH",
            ));
        };
        Ok(exe.into_os_string())
    }

    /// Low-level command execution method to invoke Restic with JSON handling.
    pub(crate) async fn exec_json<P, F>(
        &self,
        arguments: ArgumentsBuilder,
        mut on_message: F,
        cancellation_token: &CancellationToken,
    ) -> Result<(), ResticError>
    where
        P: ResticMessage,
        F: FnMut(P),
    {
        self.exec(
            arguments.with_flag("json"),
            |line, output_type| {
                if line.is_empty() {
                    return;
                }
                if line.starts_with("{") {
                    match P::parse_message(&line) {
                        Ok(message) => on_message(message),
                        Err(err) => {
                            warn!("Failed to parse {output_type} message '{line}' due to '{err}'")
                        }
                    }
                } else {
                    debug!("Ignored non-JSON {output_type} message: '{line}'");
                }
            },
            cancellation_token,
        )
        .await
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum MessageOutputType {
    Stdout,
    Stderr,
}

impl Display for MessageOutputType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            MessageOutputType::Stdout => write!(f, "stdout"),
            MessageOutputType::Stderr => write!(f, "stderr"),
        }
    }
}
