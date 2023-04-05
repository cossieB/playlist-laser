use std::env;
use playser::config::Config;

fn main() {
    let args = env::args();
    Config::new(args);
}
