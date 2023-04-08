use playzer::{
    config,
    format
};

use std::{env, io, process};

fn main() {
    let mut buf = String::new();
    let args = env::args();
    let config = config::Config::new(args).unwrap_or_else(|err| {
        println!("{err}. Press any 'Enter' to quit.");
        io::stdin().read_line(&mut buf).unwrap();
        process::exit(1)
    });
    let reader = format::get_reader_writer(&config.format());
    let writer = format::get_reader_writer(&config.output_format());
    let v = reader.parse_file(&config);
    let path = writer.write_file(&v, &config).unwrap_or_else(|err| {
        println!("{err}. Press any 'Enter' to quit.");
        io::stdin().read_line(&mut buf).unwrap();
        process::exit(1)
    });
    println!("Done. Playlist saved as \"{path}\". Press \"Enter\" to exit.");
    io::stdin().read_line(&mut buf).unwrap();
}
