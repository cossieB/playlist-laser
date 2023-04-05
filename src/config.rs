use std::{env::Args, path};
use crate::format;

#[derive(Debug)]
pub struct Config {
    playlist: String,
    format: format::Format,
    keep_duplicates: bool,
    shuffle: bool,
}
impl Config {
    pub fn new(mut args: Args) -> Result<Config, &'static str> {
        args.next();
        let mut playlist = match args.next() {
            Some(arg) => arg,
            None => Self::get_input("Please enter the path to the playlist".to_string()),
        };
        while !Self::file_exists(&playlist) {
            playlist = Self::get_input("File not found. Please enter a valid path. \nHint on Windows right-click on the playlist file while holding shift, then select \"copy as path\"".to_string());
        }
        let format = Self::get_format(&playlist)?;
        let keep_duplicates;
        let shuffle;

        if let Some(options) = args.next() {
            keep_duplicates = options.contains('d');
            shuffle = options.contains('s');
        }
        else {
            keep_duplicates = false;
            shuffle = false;
        }

        Ok(Config {
            playlist,
            format,
            keep_duplicates,
            shuffle,
        })
    }
    fn get_input(prompt: String) -> String {
        println!("{prompt}");
        let mut input = String::new();
        loop {
            match std::io::stdin().read_line(&mut input) {
                Ok(_) => {
                    break;
                }
                Err(e) => {
                    println!("Unable to read input. Please try again. {}", e)
                }
            };
        }
        input.trim().to_string()
    }
    
    fn file_exists(path: &str) -> bool {
        let path = path::Path::new(path);
        path.exists()
    }
    
    fn get_format(path: &str) -> Result<format::Format, &'static str> {
        let path = path::Path::new(path);
        let ext = path.extension();
        
        match ext {
            None => Err("Unknown file format"),
            Some(os_str) => {
                match os_str.to_str() {
                    None => Err("Error reading playlist format"),
                    Some("m3u8" | "m3u") => Ok(format::Format::M3U),
                    _ => Err("Playlist format currently not supported")
                }
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::format::Format;
    use super::*;

    #[test]
    fn file_does_exist() {
        assert!(Config::file_exists("./test.txt"))
    }
    #[test]
    fn file_doesnt_exist() {
        assert!(!Config::file_exists("./fake.txt"))
    }
    #[test]
    fn format_is_m3u() {
        let f = Config::get_format("./test.m3u8");
        assert!(f.is_ok());
        assert_eq!(f.unwrap(), Format::M3U)
    }
    #[test]
    fn format_unsupported() {
        let f = Config::get_format("./test.txt");
        println!("{:?}", f);
        assert!(f.is_err());
        assert_eq!(f.unwrap_err(), "Playlist format currently not supported")
    }
}