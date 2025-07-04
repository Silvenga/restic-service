use std::collections::HashMap;

/// Configuration for the Restic client.
#[derive(Clone, Debug, Default)]
pub struct ResticConfig {
    /// Set of environment variables to set when running restic commands.
    /// See [supported environment variables](https://restic.readthedocs.io/en/latest/040_backup.html#environment-variables).
    pub environment: HashMap<String, String>,
}

impl ResticConfig {
    /// Location of repository.
    /// This will set the `RESTIC_REPOSITORY` environment variable on the spawned process.
    pub fn with_repository(self, repository: &str) -> Self {
        self.with_env_var("RESTIC_REPOSITORY", repository)
    }

    /// The actual password for the repository.
    /// This will set the `RESTIC_PASSWORD` environment variable on the spawned process.
    pub fn with_password(self, password: &str) -> Self {
        self.with_env_var("RESTIC_PASSWORD", password)
    }

    /// Concurrency for file reads.
    /// This will set the `RESTIC_READ_CONCURRENCY` environment variable on the spawned process.
    pub fn with_read_concurrency(self, concurrency: usize) -> Self {
        self.with_env_var("RESTIC_READ_CONCURRENCY", &concurrency.to_string())
    }

    /// Set an environment variable for the Restic client.
    pub fn with_env_var(mut self, key: &str, value: &str) -> Self {
        self.environment.insert(key.to_owned(), value.to_owned());
        self
    }
}
