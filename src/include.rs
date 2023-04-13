use std::collections::HashSet;
use std::sync::{Arc, Mutex};
use std::{thread, fs};

use crate::config;

pub fn run(config: &config::Config, set: &HashSet<String>, playlist: &mut Vec<String>) {
    let config_arc = Arc::new(config);
    let playlist_arc = Arc::new(Mutex::new(playlist));
    
    for dir in config.other_directories() {
        let config = Arc::clone(&config_arc);
        let playlist = Arc::clone(&playlist_arc);
        thread::scope(|s| {
            s.spawn(move || {
                let mut t = get_directory_files(&set, dir, config.keep_duplicates());
                let mut playlist = playlist.lock().unwrap();
                playlist.append(&mut t);
            });
        });
    };
}

fn get_directory_files(set: &HashSet<String>, dir: &String, keep_duplicates: bool) -> Vec<String> {
    if let Ok(dir) = fs::read_dir(dir) {
        let v: Vec<_> = dir
            .into_iter()
            .filter_map(|item| {
                let dir = item.ok()?;
                let md = dir.metadata().ok()?;
                let path = dir.path().to_str()?.to_string();
                let should_add = md.is_file() && (keep_duplicates || !set.contains(&path));
                should_add.then_some(path)
            })
            .collect();
        return v
    };
    Vec::new()
}