use std::{collections::HashSet, fs};
use crate::config;
use super::Playlist;

pub struct M3U;

impl Playlist for M3U {
    fn parse_file(&self, config: &config::Config) -> Vec<String> {
        let mut set = HashSet::new();
        let file = fs::read_to_string(config.playlist()).expect("Unknown Error");
        let mut list = Vec::with_capacity(1000);
        
        for line in file.lines() {
            if config.keep_duplicates() == false && set.contains(line) {
                continue;
            }
            if crate::file_exists(line) {
                if config.keep_duplicates() == false {
                    set.insert(line.to_string());
                }
                list.push(line.to_string());
            }
        };
        list
    }
    fn write_file(&self) {
        
    }
}