use super::PlaylistReaderWriter;
use crate::config;
use std::{
    collections::HashSet,
    fs,
};

pub struct M3UReaderWriter;

impl PlaylistReaderWriter for M3UReaderWriter {
    fn read_file(&self, config: &config::Config) -> (Vec<String>, HashSet<String>) {
        let mut set = HashSet::new();
        let file = fs::read_to_string(config.playlist()).expect("Error while reading playlist");
        let mut list = Vec::with_capacity(1000);

        for line in file.lines() {
            if config.keep_duplicates() == false && set.contains(line) {
                continue;
            }
            self.add_files_to_list(config, &mut set, line, &mut list);
        }
        (list, set)
    }
    fn write_file(&self, files: &Vec<String>, config: &config::Config) -> Result<String, &'static str> {
        let path = self.generate_new_filename(&config);
        let mut contents = String::new();
        files.iter().for_each(|file| {
            contents.push_str(file);
            contents.push('\n')
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
        let config = crate::config::Config::new(["".to_string(),"./test_assets/test.m3u8".to_string(), "m3u".to_string()].into_iter()).unwrap();
        let mock = M3UReaderWriter;
        let v = mock.read_file(&config).0;
        assert_eq!(v, vec!["./test_assets/test.txt".to_string()])
    }
    #[test]
    fn yes_duplicates() {
        let config = crate::config::Config::new(["".to_string(),"./test_assets/test.m3u8".to_string(), "m3u".to_string(), "d".to_string()].into_iter()).unwrap();
        let mock = M3UReaderWriter;
        let v = mock.read_file(&config).0;
        assert_eq!(v, vec!["./test_assets/test.txt".to_string(), "./test_assets/test.txt".to_string()])
    }
}
