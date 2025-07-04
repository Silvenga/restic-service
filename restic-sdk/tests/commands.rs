use restic_sdk::{Restic, ResticConfig};
use std::env::temp_dir;
use std::fs::{create_dir, create_dir_all, remove_dir_all, write};
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

#[tokio::test]
async fn command_init_if_not_exists() {
    let repository = VirtualRepository::new();

    let restic = repository.get_client();
    restic.init_if_not_exists().await.unwrap();

    let result = restic.can_open().await.unwrap();

    assert!(result)
}

#[tokio::test]
async fn command_backup() {
    let repository = VirtualRepository::new();

    let restic = repository.get_client();
    restic.init().await.unwrap();

    let summary = restic
        .backup(vec![repository.get_random_data_path().as_str()])
        .await;

    assert!(summary.unwrap().snapshot_id.is_some());
}

struct VirtualRepository {
    path: PathBuf,
}

impl VirtualRepository {
    pub fn new() -> Self {
        colog::init();

        let id: String = Uuid::new_v4().into();
        let path = Path::join(&temp_dir(), "restic-sdk-test-repo").join(id);
        create_dir_all(&path).unwrap();
        Self { path }
    }

    pub fn get_client(&self) -> Restic {
        let repository = self.path.join("repo");
        let config = ResticConfig::default()
            .with_repository(repository.to_str().unwrap())
            .with_password("test_password");

        Restic::default().with_config(config)
    }

    pub fn get_random_data_path(&self) -> String {
        let random_data_path = self.path.join("ran_data");
        create_dir(random_data_path.clone()).unwrap();

        write(
            random_data_path.join("fake_file"),
            Uuid::new_v4().into_bytes(),
        )
        .unwrap();

        random_data_path.to_str().unwrap().to_owned()
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
