use crate::config;

pub mod m3u;

#[derive(Debug, PartialEq)]
pub enum Format {
    M3U,
}

pub trait Playlist {
    fn parse_file(&self, config: &config::Config) -> Vec<String>;
    fn write_file(&self);
}