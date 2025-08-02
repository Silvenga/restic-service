mod common;

use common::VirtualRepository;
use tokio_util::sync::CancellationToken;

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
