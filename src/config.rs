use std::{env::Args, path};
use crate::format;

#[derive(Debug)]
pub struct Config {
    playlist: String,
    format: format::Format,
    delete_duplicates: bool,
    preserve_order: bool,
}
impl Config {
    pub fn new(mut args: Args) -> Result<Config, &'static str> {
        args.next();
        let delete_duplicates = true;
        let preserve_order = true;

        let mut playlist = match args.next() {
            Some(arg) => arg,
            None => Self::get_input("Please enter the path to the playlist".to_string()),
        };
        while !Self::file_exists(&playlist) {
            playlist = Self::get_input("File not found. Please enter a valid path. \nHint on Windows right-click on the playlist file while holding shift, then select \"copy as path\"".to_string());
        }
        let format = Self::get_format(&playlist)?;

        Ok(Config {
            playlist,
            format,
            delete_duplicates,
            preserve_order,
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
                    println!("Unable to read input. Please try again.")
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
            None => Err("Playlist not supported"),
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

