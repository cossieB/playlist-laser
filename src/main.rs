use playzer::{
    config,
    format::{self, PlaylistReaderWriter},
};

use std::{env, process, io};

fn main() {
   
    let mut buf = String::new();
    let args = env::args();
    let config = config::Config::new(args).unwrap_or_else(|err| {
        println!("{err}. Press any 'Enter' to quit.");
        io::stdin().read_line(&mut buf).unwrap();
        process::exit(1)
    });
    let reader_writer: Box<dyn PlaylistReaderWriter> = match &config.format() {
        format::Format::M3U => Box::new(format::M3UReaderWriter),
    };
    let v = reader_writer.parse_file(&config);
    let path = reader_writer.write_file(&v, &config).unwrap_or_else(|err| {
        println!("{err}. Press any 'Enter' to quit.");
        io::stdin().read_line(&mut buf).unwrap();
        process::exit(1)
    });
    println!("Done. Playlist saved as \"{path}\". Press \"Enter\" to exit.");
    io::stdin().read_line(&mut buf).unwrap();
}

