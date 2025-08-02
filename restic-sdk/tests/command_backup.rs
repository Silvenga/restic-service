mod common;

use common::VirtualRepository;
use restic_sdk::backup::BackupOptions;
use tokio_util::sync::CancellationToken;

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
