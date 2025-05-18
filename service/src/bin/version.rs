use restic_sdk::{Restic, ResticConfig};
use std::collections::HashMap;

#[tokio::main]
async fn main() {
    let config = ResticConfig {
        environment: HashMap::from([("Mercury".to_string(), "".to_string())]),
    };
    let restic = Restic::new(config);
    match restic.get_version().await {
        Ok(a) => println!("Ok: {}", a.version),
        Err(e) => println!("Error: {}", e),
    }
}
