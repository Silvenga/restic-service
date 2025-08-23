mod common;

use common::VirtualRepository;
use tokio_util::sync::CancellationToken;

#[tokio::test]
async fn command_version() {
    let repository = VirtualRepository::new();

    let restic = repository.get_client();
    let result = restic.version(&CancellationToken::new()).await;

    _ = result.unwrap();
}
