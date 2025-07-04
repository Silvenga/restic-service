use std::collections::HashMap;

#[derive(Clone, Debug, Default)]
pub struct ResticConfig {
    pub environment: HashMap<String, String>,
}

impl ResticConfig {
    /// Sets the repository to init or access.
    /// This will set the `RESTIC_REPOSITORY` environment variable on the spawned process.
    pub fn with_repository(mut self, repository: &str) -> Self {
        self.environment
            .insert("RESTIC_REPOSITORY".to_string(), repository.to_string());
        self
    }

    /// Sets the password to init or access the Restic repository.
    /// This will set the `RESTIC_PASSWORD` environment variable on the spawned process.
    pub fn with_password(mut self, password: &str) -> Self {
        self.environment
            .insert("RESTIC_PASSWORD".to_string(), password.to_string());
        self
    }
}
