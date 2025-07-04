use restic_sdk::errors::ResticError;
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
async fn command_cat_missing_repository() {
    let repository = VirtualRepository::new();

    let restic = repository.get_client();
    let result = restic.cat("config").await;

    match result {
        Err(ResticError::RepositoryDoesNotExist) => {}
        Err(err) => panic!("Expected RepositoryDoesNotExist error, got: {err:?}"),
        Ok(_) => panic!("Expected an error, but got a successful response"),
    };
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
        let path_str = self.path.to_str().unwrap();

        let env = vec![
            ("RESTIC_REPOSITORY".to_owned(), path_str.to_owned()),
            ("RESTIC_PASSWORD".to_owned(), "test_password".to_owned()),
        ];

        let config = ResticConfig {
            environment: env.into_iter().collect(),
        };

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
