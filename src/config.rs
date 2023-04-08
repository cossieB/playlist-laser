use std::{env::Args};
use crate::format;

#[derive(Debug, PartialEq)]
pub struct Config {
    playlist: String,
    format: format::Format,
    keep_duplicates: bool,
    shuffle: bool,
    output_format: format::Format,
}
impl Config {
    pub fn new(mut args: Args) -> Result<Config, &'static str> {
        args.next();
        let mut playlist = match args.next() {
            Some(arg) => arg,
            None => super::get_input("Please enter the path to the playlist".to_string()),
        };
        while !crate::file_exists(&playlist) {
            playlist = super::get_input("File not found. Please enter a valid path. \nHint on Windows right-click on the playlist file while holding shift, then select \"copy as path\"".to_string());
        }
        let format = format::Format::get_format(&playlist)?;

        let output_format = match args.next() {
            Some(val) => {
                format::Format::get_format_from_ext(&val).unwrap_or_else(|_| {
                    println!("Unrecognized output format (2nd argument). Defaulting to original playlist format");
                    format.clone()
                })
            },
            None => format.clone()
        };
        let options = args.next();
        let keep_duplicates = match &options {
            None => false,
            Some(opt) => opt.contains('d')
        };
        let shuffle = match &options {
            None => false,
            Some(opt) => opt.contains('s')
        };

        Ok(Config {
            playlist,
            format,
            keep_duplicates,
            shuffle,
            output_format
        })
    }
    // Getters
    pub fn playlist(&self) -> &String {
        &self.playlist
    }
    pub fn format(&self) -> &format::Format {
        &self.format
    }
    pub fn keep_duplicates(&self) -> bool {
        self.keep_duplicates
    }
    pub fn shuffle(&self) -> bool {
        self.shuffle
    }
    pub fn output_format(&self) -> &format::Format {
        &self.output_format
    }
}

#[cfg(test)]
mod tests {
    use crate::format::Format;

    #[test]
    fn file_does_exist() {
        assert!(crate::file_exists("./test.txt"))
    }
    #[test]
    fn file_doesnt_exist() {
        assert!(!crate::file_exists("./fake.txt"))
    }
    #[test]
    fn format_is_m3u() {
        let f = Format::get_format("./test.m3u8");
        assert!(f.is_ok());
        assert_eq!(f.unwrap(), Format::M3U)
    }
    #[test]
    fn format_unsupported() {
        let f = Format::get_format("./test.txt");
        println!("{:?}", f);
        assert!(f.is_err());
        assert_eq!(f.unwrap_err(), "Playlist format currently not supported")
    }
}