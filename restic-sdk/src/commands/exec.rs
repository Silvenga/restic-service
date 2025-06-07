use crate::Restic;
use crate::errors::{ResticError, map_exit_code_to_error};
use crate::parsing::{ResticMessage, parse_restic_message};
use log::warn;
use pathsearch::find_executable_in_path;
use std::ffi::{OsStr, OsString};
use std::io;
use std::process::{ExitStatus, Stdio};
use tokio::io::{AsyncBufReadExt, BufReader};
use tokio::process::Command;

impl Restic {
    pub(crate) async fn exec<I, S, F>(
        &self,
        arguments: I,
        mut on_message: F,
    ) -> Result<(), ResticError>
    where
        I: IntoIterator<Item = S>,
        S: AsRef<OsStr>,
        F: FnMut(ResticMessage),
    {
        let start = async || -> Result<ExitStatus, io::Error> {
            let binary_path = Self::get_binary_path()?;
            let mut process = Command::new(binary_path)
                .args(arguments)
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
                            Some(line) => match parse_restic_message(&line) {
                                Ok(msg) => on_message(msg),
                                Err(e) => warn!("Ignored failure to parse stdout message: {:?}", e),
                            },
                        }
                    },
                    stderr_lines = stderr_lines.next_line(), if !stderr_complete => {
                        match stderr_lines? {
                            None => {
                                stderr_complete = true;
                                continue;
                            }
                            Some(line) => match parse_restic_message(&line) {
                                Ok(msg) => on_message(msg),
                                Err(e) => warn!("Ignored failure to parse stderr message: {:?}", e),
                            },
                        }
                    },
                    else => {
                        break;
                    }
                }
            }

            let status = process.wait().await?;
            Ok(status)
        };

        let status = start()
            .await
            .map_err(|io_error| ResticError::FailedToExecute(io_error))?;

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
}
