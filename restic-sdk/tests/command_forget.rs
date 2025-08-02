mod common;

use common::VirtualRepository;
use restic_sdk::backup::BackupOptions;
use restic_sdk::forget::ForgetOptions;
use tokio_util::sync::CancellationToken;

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
