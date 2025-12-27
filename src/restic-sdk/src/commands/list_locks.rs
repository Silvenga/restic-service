use crate::errors::ResticError;
use crate::exec::MessageOutputType;
use crate::{ArgumentsBuilder, Restic};
use chrono::{DateTime, Utc};
use log::warn;
use std::collections::HashSet;
use tokio_util::sync::CancellationToken;

impl Restic {
    /// Gets all the locks in the Restic repository.
    ///
    /// Helper method around `get_lock_ids` and `get_lock_by_id`.
    pub async fn get_locks(
        &self,
        cancellation_token: &CancellationToken,
    ) -> Result<Vec<String>, ResticError> {
        let lock_ids = self.get_lock_ids(cancellation_token).await?;

        let mut locks = Vec::with_capacity(lock_ids.len());

        for id in lock_ids.iter() {
            let lock = self.get_lock_by_id(id, cancellation_token).await?;
            locks.push(lock.id);
        }

        Ok(locks)
    }

    /// Gets the ids of all locks in the Restic repository.
    ///
    /// Performs `restic list locks`
    pub async fn get_lock_ids(
        &self,
        cancellation_token: &CancellationToken,
    ) -> Result<HashSet<String>, ResticError> {
        let mut locks = HashSet::new();

        self.exec(
            ArgumentsBuilder::new()
                .with_verb("list")
                .with_value("locks")
                .with_flag("json"),
            |message, output_type| match output_type {
                MessageOutputType::Stdout => {
                    if message.len() == 64 {
                        locks.insert(message);
                    } else {
                        warn!("Ignored lock id with unexpected length: '{message}'");
                    }
                }
                MessageOutputType::Stderr => {
                    warn!("{output_type}: {message}");
                }
            },
            cancellation_token,
        )
        .await?;

        Ok(locks)
    }

    /// Retrieves the details of a specific lock by its ID.
    ///
    /// Performs `restic cat locks {lock_id}`.
    pub async fn get_lock_by_id(
        &self,
        id: &str,
        cancellation_token: &CancellationToken,
    ) -> Result<ResticLockWithId, ResticError> {
        let mut json = String::new();

        self.exec(
            ArgumentsBuilder::new()
                .with_verb("cat")
                .with_values(["lock", id])
                .with_flag("json"),
            |message, output_type| {
                if output_type == MessageOutputType::Stdout {
                    json += &message;
                }
            },
            cancellation_token,
        )
        .await?;

        parse_lock_json(id, &json)
            .map_err(|e| ResticError::UnexpectedResponse(format!("Failed to parse lock JSON: {e}")))
    }
}

fn parse_lock_json(
    id: impl Into<String>,
    json: &str,
) -> Result<ResticLockWithId, serde_json::Error> {
    serde_json::from_str::<ResticLock>(json).map(|x| x.with_id(id.into()))
}

#[derive(serde::Deserialize)]
struct ResticLock {
    time: DateTime<Utc>,
    exclusive: bool,
    hostname: String,
    username: String,
    pid: u32,
}

impl ResticLock {
    fn with_id(self, id: String) -> ResticLockWithId {
        ResticLockWithId {
            id,
            time: self.time,
            exclusive: self.exclusive,
            hostname: self.hostname,
            username: self.username,
            pid: self.pid,
        }
    }
}

#[derive(Debug, Clone)]
pub struct ResticLockWithId {
    pub id: String,
    pub time: DateTime<Utc>,
    pub exclusive: bool,
    pub hostname: String,
    pub username: String,
    pub pid: u32,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_lock_json() {
        let json = r#"{
            "time": "2025-07-06T18:56:28.8860478-05:00",
            "exclusive": true,
            "hostname": "myhost",
            "username": "myuser",
            "pid": 12345
        }"#;

        let lock = parse_lock_json(
            "abcdef1234567890abcdef1234567890abcdef1234567890abcdef1234567890",
            json,
        )
        .expect("should parse lock");

        assert_eq!(
            lock.id,
            "abcdef1234567890abcdef1234567890abcdef1234567890abcdef1234567890"
        );
        assert_eq!(lock.exclusive, true);
        assert_eq!(lock.hostname, "myhost");
        assert_eq!(lock.username, "myuser");
        assert_eq!(lock.pid, 12345);
        assert_eq!(
            lock.time.to_rfc3339(),
            "2025-07-06T23:56:28.886047800+00:00"
        );
    }
}
