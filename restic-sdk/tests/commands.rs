use restic_sdk::{Restic, ResticConfig};
use std::env::temp_dir;
use std::fs::{create_dir_all, remove_dir_all};
use std::path::{Path, PathBuf};
use uuid::Uuid;

#[tokio::test]
async fn command_version() {
    let repository = VirtualRepository::new();

    let restic = repository.get_client();
    let result = restic.version().await;

    _ = result.unwrap();
}

#[tokio::test]
async fn command_can_open_missing() {
    let repository = VirtualRepository::new();

    let restic = repository.get_client();
    let result = restic.can_open().await.unwrap();

    assert!(!result)
}

#[tokio::test]
async fn command_can_open_existing() {
    let repository = VirtualRepository::new();

    let restic = repository.get_client();
    restic.init().await.unwrap();

    let result = restic.can_open().await.unwrap();

    assert!(result)
}

struct VirtualRepository {
    path: PathBuf,
}

impl VirtualRepository {
    pub fn new() -> Self {
        let id: String = Uuid::new_v4().into();
        let path = Path::join(&temp_dir(), "restic-sdk-test-repo").join(id);
        create_dir_all(&path).unwrap();
        Self { path }
    }

    pub fn get_client(&self) -> Restic {
        let repository = self.path.to_str().unwrap();

        let config = ResticConfig::default()
            .with_repository(repository)
            .with_password("test_password");

        Restic::default().with_config(config)
    }
}

impl Drop for VirtualRepository {
    fn drop(&mut self) {
        match remove_dir_all(&self.path) {
            Ok(_) => {}
            Err(e) => {
                eprintln!(
                    "Failed to remove test repository '{}' due to error: {}",
                    self.path.display(),
                    e
                )
            }
        }
    }
}
