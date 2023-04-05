use playser::config::Config;
use playser::format::Playlist;
use std::{env, process};

fn main() {
    let args = env::args();
    let config = Config::new(args).unwrap_or_else(|err| {
        println!("{err}");
        process::exit(1)
    });
    let playlist = &playser::format::m3u::M3U;
    let v = playlist.parse_file(&config);
}
