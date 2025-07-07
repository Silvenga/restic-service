use restic_sdk::backup::BackupOptions;
use restic_sdk::forget::ForgetOptions;
use restic_sdk::{Restic, ResticConfig};
use std::env::temp_dir;
use std::fs::{create_dir, create_dir_all, remove_dir_all, write};
use std::path::{Path, PathBuf};
use tokio_util::sync::CancellationToken;
use uuid::Uuid;

#[cfg(test)]
#[ctor::ctor]
fn init() {
    colog::init();
}

#[tokio::test]
async fn command_version() {
    let repository = VirtualRepository::new();

    let restic = repository.get_client();
    let result = restic.version(&CancellationToken::new()).await;

    _ = result.unwrap();
}

#[tokio::test]
async fn command_can_open_missing() {
    let repository = VirtualRepository::new();

    let restic = repository.get_client();
    let result = restic.can_open(&CancellationToken::new()).await.unwrap();

    assert!(!result)
}

#[tokio::test]
async fn command_can_open_existing() {
    let repository = VirtualRepository::new();

    let restic = repository.get_client();
    restic.init(&CancellationToken::new()).await.unwrap();

    let result = restic.can_open(&CancellationToken::new()).await.unwrap();

    assert!(result)
}

#[tokio::test]
async fn command_init_if_not_exists() {
    let repository = VirtualRepository::new();

    let restic = repository.get_client();
    restic
        .init_if_not_exists(&CancellationToken::new())
        .await
        .unwrap();

    let result = restic.can_open(&CancellationToken::new()).await.unwrap();

    assert!(result)
}

#[tokio::test]
async fn command_backup() {
    let repository = VirtualRepository::new();

    let restic = repository.get_client();
    restic.init(&CancellationToken::new()).await.unwrap();

    let summary = restic
        .backup(
            vec![repository.get_random_data_path().as_str()],
            BackupOptions::new(),
            &CancellationToken::new(),
        )
        .await;

    assert!(summary.unwrap().snapshot_id.is_some());
}

#[tokio::test]
async fn command_forget() {
    let repository = VirtualRepository::new();

    let restic = repository.get_client();
    restic.init(&CancellationToken::new()).await.unwrap();

    let random_data_path = repository.get_random_data_path();

    _ = restic
        .backup(
            vec![random_data_path.as_str()],
            BackupOptions::new(),
            &CancellationToken::new(),
        )
        .await;
    _ = restic
        .backup(
            vec![random_data_path.as_str()],
            BackupOptions::new(),
            &CancellationToken::new(),
        )
        .await;

    _ = restic
        .forget(
            ForgetOptions::new().keep_last(0).unsafe_allow_remove_all(),
            &CancellationToken::new(),
        )
        .await;
}

struct VirtualRepository {
    random_data_path: PathBuf,
    repository_path: PathBuf,
}

impl VirtualRepository {
    pub fn new() -> Self {
        let id: String = Uuid::new_v4().into();
        let root_path = Path::join(&temp_dir(), "restic-sdk-test-repo").join(id);
        let repository_path = root_path.join("repo");
        let random_data_path = root_path.join("ran_data");

        create_dir_all(&root_path).unwrap();
        create_dir(random_data_path.clone()).unwrap();

        write(
            random_data_path.join("fake_file"),
            Uuid::new_v4().into_bytes(),
        )
        .unwrap();

        Self {
            random_data_path,
            repository_path,
        }
    }

    pub fn get_client(&self) -> Restic {
        let config = ResticConfig::default()
            .with_repository(self.repository_path.to_str().unwrap())
            .with_password("test_password");

        Restic::default().with_config(config)
    }

    pub fn get_random_data_path(&self) -> String {
        self.random_data_path.to_str().unwrap().to_owned()
    }
}

impl Drop for VirtualRepository {
    fn drop(&mut self) {
        match remove_dir_all(&self.random_data_path) {
            Ok(_) => {}
            Err(e) => {
                eprintln!(
                    "Failed to remove test repository '{}' due to error: {}",
                    self.random_data_path.display(),
                    e
                )
            }
        }
    }
}
