use regex::Regex;

const BASE_URL           : &str = "https://boards.4channel.org";
const REGEX_URL          : &str = r#"^((?:https?://)?[^./]+(?:\.[^./]+)+(?:/.*)?)$"#;
const REGEX_HTTPS        : &str = r"https?://";
const REGEX_WWW          : &str = r"^www\."
const REGEX_BOARD_THREAD : &str = r"/[a-zA-Z]+/thread/[0-9]+";

pub struct UrlHandler {
    pub url: String
}

impl UrlHandler {
    fn test_against(&self, re: &str) -> bool {
        match Regex::new(&re) {
            Ok(regexp) => regexp.is_match(&self.url),
            Err(err) => panic!("couldn't parse regex:\n{}", err)
        }
    }

    fn handle_http(&mut self) {
        match Regex::new(REGEX_HTTPS) {
            Ok(regexp) => regexp.replace(&self.url, ""),
            Err(err) => panic!("couldn't not remove http from url\n{}", err)
        };
    }

    fn handle_www(&mut self) {
        match Regex::new(REGEX_WWW) {
            Ok(regexp) => regexp.replace(&self.url, ""),
            Err(err) => panic!("couldn't not remove http from url\n{}", err)
        };
    }

    fn is_valid_url(&self) -> bool {
        match Regex::new(&REGEX_URL) {
            Ok(regexp) => regexp.is_match(&self.url),
            Err(err) => panic!("panicked whilst checking URL:\n{}", err)
        }
    }

    fn is_4chan_url(&self) -> bool {
        true
    }

    pub fn extract(&self) { 
        if !self.is_valid_url() {
            panic!("supplied URL is not valid!");
        } 

        match &self.url {
            _url if self.test_against(r"^https?//") => self.handle_http(),
            _url if self.test_against(r"^www\.")    => self.handle_www(),
            _ => {}
        }
    }

    pub fn new(url: String) -> Self {
        UrlHandler { url: url }
    }
}