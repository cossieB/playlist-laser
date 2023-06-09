use std::path;

use playzer::*;

#[test]
fn it_works() {
    let args = ["".to_string(),"./test_assets/test.pls".to_string(), "asx".to_string()].into_iter();
    let config = config::Config::new(args).unwrap_or_else(|err| {
        panic!("{err}")
    });
    let reader = format::get_reader_writer(&config.format());
    let writer = format::get_reader_writer(&config.output_format());
    let v = reader.read_file(&config).0;
    let path = writer.write_file(&v, &config).unwrap_or_else(|err| {
        panic!("{err}")
    });
    let path = path::Path::new(&path);
    assert!(path.exists());
}