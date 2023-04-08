use regex::Regex;

use super::PlaylistReaderWriter;
use crate::config;
use std::{collections::HashSet, fs};

pub struct ASXReaderWriter;

impl PlaylistReaderWriter for ASXReaderWriter {
    fn parse_file(&self, config: &config::Config) -> Vec<String> {
        let mut set = HashSet::new();
        let file = fs::read_to_string(config.playlist()).expect("Error while reading playlist");
        let mut list = Vec::with_capacity(1000);
        let re = Regex::new(r#"href\s*=\s*"(.+?)""#).unwrap();
        for line in file.lines() {
            if let Some(path) = super::extract_regex(line, &re) {
                if config.keep_duplicates() == false && set.contains(&path) {
                    continue;
                }
                self.add_files_to_list(config, &mut set, &path, &mut list);
            }
        }
        println!("{}", list.len());
        list
    }

    fn write_file(&self, files: &Vec<String>, config: &config::Config) -> Result<String, &'static str> {
        todo!()
    }
}
