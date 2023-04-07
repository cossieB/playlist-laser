use std::path;

use crate::{config, file_exists};
mod m3u;
pub use m3u::M3UReaderWriter;

#[derive(Debug, PartialEq, Clone)]
pub enum Format {
    M3U,
}

impl Format {
    pub fn get_format(path: &str) -> Result<Self, &'static str> {
        let path = path::Path::new(path);
        let ext = path.extension();
        
        match ext {
            None => Err("Unknown file format"),
            Some(os_str) => {
                match os_str.to_str() {
                    None => Err("Error reading playlist format"),
                    Some(ext) => Self::get_format_from_ext(ext),
                }
            }
        }
    }
    pub fn get_format_from_ext(ext: &str) -> Result<Self, &'static str> {
        match ext.to_lowercase().as_str() {
            "m3u" | "m3u8" => Ok(Self::M3U),
            _ => Err("Playlist format currently not supported")
        }
    }
    pub fn get_extension(format: &Self) -> String {
        match format {
            Format::M3U => "m3u8".to_string(),
        }
    }
}

pub trait PlaylistReaderWriter {
    fn parse_file(&self, config: &config::Config) -> Vec<String>;
    fn write_file(&self, files: &Vec<String>, config: &config::Config) -> Result<(), &'static str> ;
    
    fn generate_new_filename(&self, config: &config::Config) -> String {
        let name = get_filename(config.playlist());
        let extension = Format::get_extension(config.format());
        let mut rng = rand::thread_rng(); //random number to prevent filename clashes.
        let mut path = format!("{name} CLEANED {}.{extension}", rand::Rng::gen_range(&mut rng, 1000..99999));
        while file_exists(&path) {
            path = format!("{name} CLEANED {}.{extension}", rand::Rng::gen_range(&mut rng, 1000..99999));
        }
        path
    }
}

fn get_filename(path: &str) -> String {
    let re = regex::Regex::new(r"(^(?:\.(?:/|\\))?[^\.]+)(?:\.\w+)?$").unwrap();
    extract_regex(path, re).unwrap_or("playzer_generated".to_owned())
}

fn extract_regex(text: &str, re: regex::Regex) -> Option<String> {
    let captures = re.captures(text)?; 
    let mtch = captures.get(1)?;
    Some(String::from(&text[0..mtch.end()]))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn filename_tests() {
        assert_eq!(get_filename("hello.txt"), "hello");
        assert_eq!(get_filename("h3ll0.mp3"), "h3ll0");
        assert_eq!(get_filename(".txt"), "playzer_generated");
        assert_eq!(get_filename("hello"), "hello");
        assert_eq!(get_filename("hello..txt"), "playzer_generated");
        assert_eq!(get_filename("./hello.txt"), "./hello");
        assert_eq!(get_filename(".\\hello.txt"), ".\\hello");
    }
}