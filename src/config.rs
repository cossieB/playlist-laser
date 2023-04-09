use crate::format;

#[derive(Debug, PartialEq)]
pub struct Config {
    playlist: String,
    format: format::Format,
    keep_duplicates: bool,
    shuffle: bool,
    output_format: format::Format,
}
impl Config {
    pub fn new(mut args: impl Iterator<Item = String>) -> Result<Config, &'static str> {
        let tst = args.next(); println!("{:#?}", &tst.unwrap());
        let mut playlist = match args.next() {
            Some(arg) => arg,
            None => super::get_input("Please enter the path to the playlist".to_string()),
        };
        while !crate::file_exists(&playlist) {
            playlist = super::get_input("File not found. Please enter a valid path. \nHint on Windows right-click on the playlist file while holding shift, then select \"copy as path\"".to_string());
        }
        let format = format::Format::get_format(&playlist)?;

        let output_format = match args.next() {
            Some(val) => {
                format::Format::get_format_from_ext(&val).unwrap_or_else(|_| {
                    println!("Unrecognized output format (2nd argument). Defaulting to original playlist format");
                    format.clone()
                })
            },
            None => format.clone()
        };
        let options = args.next();
        let keep_duplicates = match &options {
            None => false,
            Some(opt) => opt.contains('d')
        };
        let shuffle = match &options {
            None => false,
            Some(opt) => opt.contains('s')
        };

        Ok(Config {
            playlist,
            format,
            keep_duplicates,
            shuffle,
            output_format
        })
    }
    // Getters
    pub fn playlist(&self) -> &String {
        &self.playlist
    }
    pub fn format(&self) -> &format::Format {
        &self.format
    }
    pub fn keep_duplicates(&self) -> bool {
        self.keep_duplicates
    }
    pub fn shuffle(&self) -> bool {
        self.shuffle
    }
    pub fn output_format(&self) -> &format::Format {
        &self.output_format
    }
}

#[cfg(test)]
mod tests {
    use crate::format::Format;
    use super::*;

    #[test]
    fn valid_config_1() {
        let config = Config::new(["".to_string(),"./test_assets/test.m3u8".to_string(), ".".to_string(), "s".to_string()].into_iter()).unwrap();
        assert_eq!(*config.format(), Format::M3U);
        assert_eq!(*config.output_format(), Format::M3U);
        assert_eq!(config.shuffle(), true);
        assert_eq!(config.keep_duplicates(), false);
    }
    #[test]
    fn valid_config_2() {
        let config = Config::new(["".to_string(),"./test_assets/test.pls".to_string(), "m3u".to_string(), "d".to_string()].into_iter()).unwrap();
        assert_eq!(*config.format(), Format::PLS);
        assert_eq!(*config.output_format(), Format::M3U);
        assert_eq!(config.shuffle(), false);
        assert_eq!(config.keep_duplicates(), true);
    }
    #[test]
    fn valid_config_3() {
        let config = Config::new(["".to_string(),"./test_assets/test.asx".to_string(), "pls".to_string()].into_iter()).unwrap();
        assert_eq!(*config.format(), Format::ASX);
        assert_eq!(*config.output_format(), Format::PLS);
        assert_eq!(config.shuffle(), false);
        assert_eq!(config.keep_duplicates(), false);
    }
    #[test]
    fn valid_config_4() {
        let config = Config::new(["".to_string(),"./test_assets/test.m3u8".to_string(), "asx".to_string(), "ds".to_string()].into_iter()).unwrap();
        assert_eq!(*config.format(), Format::M3U);
        assert_eq!(*config.output_format(), Format::ASX);
        assert_eq!(config.shuffle(), true);
        assert_eq!(config.keep_duplicates(), true);
    }
}