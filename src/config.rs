use std::{env::Args, fmt::Error, path::Path};
use crate::utils;

pub struct Config {
    playlist: String,
    format: String,
    delete_duplicates: bool,
    preserve_order: bool
}

impl Config {
    pub fn new(mut args: Args) -> Config {
        args.next();
        let delete_duplicates = true;
        let preserve_order = true;

        let mut playlist = match args.next() {
            Some(arg) => arg,
            None => utils::get_input("Please enter the path to the playlist".to_string())
        };
        while !utils::file_exists(&playlist) {
            playlist = utils::get_input("File not found. Please enter a valid path. \nHint on Windows right-click on the playlist file while holding shift, then select copy path".to_string());
        };
        Config { playlist, format: String::new(), delete_duplicates, preserve_order }
    }

}