use std::env;
use std::path::PathBuf;

pub fn get_exe_directory() -> PathBuf {
    let current_exe = env::current_exe().expect("current_exe should return a valid path");
    let exe_dir = current_exe
        .parent()
        .expect("current_exe should be in a directory");
    exe_dir.to_path_buf()
}
