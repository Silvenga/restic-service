use restic_sdk::{Restic, ResticConfig};
use std::env::temp_dir;
use std::fs::{create_dir, create_dir_all, remove_dir_all, write};
use std::path::{Path, PathBuf};
use uuid::Uuid;

pub struct VirtualRepository {
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

    #[allow(dead_code)]
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
