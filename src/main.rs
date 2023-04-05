use std::{env, process};
use playser::config::Config;

fn main() {
    let args = env::args();
    let config = Config::new(args).unwrap_or_else(|err| {
        println!("{err}");
        process::exit(1)
    });
}
