mod virtual_repository;

use log::info;
pub use virtual_repository::*;

#[cfg(test)]
#[ctor::ctor]
fn init() {
    colog::init();
    info!("Initialized logging for tests");
}
