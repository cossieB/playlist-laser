use playser::{
    config,
    format::{self, Playlist},
};

use std::{env, process};

fn main() {
    let args = env::args();
    let config = config::Config::new(args).unwrap_or_else(|err| {
        println!("{err}");
        process::exit(1)
    });
    let reader_writer = match &config.format() {
        format::Format::M3U => format::M3UReaderWriter,
    };
    let v = reader_writer.parse_file(&config);
    reader_writer.write_file()
}
