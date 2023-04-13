use regex::Regex;

use super::PlaylistReaderWriter;
use crate::config;
use std::{
    collections::HashSet,
    fs,
};

pub struct MPCPLReaderWriter;

impl PlaylistReaderWriter for MPCPLReaderWriter {
    fn parse_file(&self, config: &config::Config) -> (Vec<String>, HashSet<String>) {
        let mut set = HashSet::new();
        let file = fs::read_to_string(config.playlist()).expect("Error while reading playlist");
        let mut list = Vec::with_capacity(1000);
        let re = Regex::new(r"filename,(.+)").unwrap();
        for line in file.lines() {
            if let Some(path) = super::extract_regex(line, &re) {
                if config.keep_duplicates() == false && set.contains(&path) {
                    continue;
                }
                self.add_files_to_list(config, &mut set, &path, &mut list);
            }
        }
        (list, set)
    }
    fn write_file(&self, files: &Vec<String>, config: &config::Config) -> Result<String, &'static str> {
        let path = self.generate_new_filename(&config);
        let mut contents = String::from("MPCPLAYLIST\naudio,-1\nsubtitles,-1\n");
        files.iter().enumerate().for_each(|(i, file)| {
            contents.push_str(&format!("{},type,0\n", i+1));
            contents.push_str(&format!("{},filename,{file}\n", i+1));
        });
        if let Err(_) = fs::write(&path, contents) {
            return Err("Unable to write to file");
        };
        Ok(path)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn no_duplicates() {
        let config = crate::config::Config::new(["".to_string(),"./test_assets/test.mpcpl".to_string(), "m3u".to_string()].into_iter()).unwrap();
        let mock = MPCPLReaderWriter;
        let v = mock.parse_file(&config).0;
        assert_eq!(v, vec!["./test_assets/test.txt".to_string()])
    }
    #[test]
    fn yes_duplicates() {
        let config = crate::config::Config::new(["".to_string(),"./test_assets/test.mpcpl".to_string(), "m3u".to_string(), "d".to_string()].into_iter()).unwrap();
        let mock = MPCPLReaderWriter;
        let v = mock.parse_file(&config).0;
        assert_eq!(v, vec!["./test_assets/test.txt".to_string(), "./test_assets/test.txt".to_string()])
    }
}