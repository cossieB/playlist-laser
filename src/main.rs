use playser::{
    config,
    format::{self, PlaylistReaderWriter},
};

use std::{env, process};

fn main() {
   
    let args = env::args();
    let config = config::Config::new(args).unwrap_or_else(|err| {
        println!("{err}");
        process::exit(1)
    });
    let reader_writer: Box<dyn PlaylistReaderWriter> = match &config.format() {
        format::Format::M3U => Box::new(format::M3UReaderWriter),
    };
    let v = reader_writer.parse_file(&config);
    reader_writer.write_file(&v, &config).unwrap_or_else(|err| {
        println!("{err}");
        process::exit(1)
    })
}

