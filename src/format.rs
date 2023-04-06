use std::path;

use crate::config;
mod m3u;
pub use m3u::M3UReaderWriter;

#[derive(Debug, PartialEq)]
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
                    Some("m3u8" | "m3u") => Ok(Self::M3U),
                    _ => Err("Playlist format currently not supported")
                }
            }
        }
    }
}

pub trait Playlist {
    fn parse_file(&self, config: &config::Config) -> Vec<String>;
    fn write_file(&self);
}
