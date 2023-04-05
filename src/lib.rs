use std::path;
pub mod config;
pub mod format;

fn file_exists(path: &str) -> bool {
    let path = path::Path::new(path);
    path.exists()
}
