use regex::Regex;

const BASE_URL: &str = "https://boards.4channel.org";
const URL_REGEX: &str = r"^((?:https?:\/\/)?[^./]+(?:\.[^./]+)+(?:\/.*)?)$";
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

    fn handle_http(&self) -> Option<&str> {
        Some("asdf") 
    }

    pub fn is_valid_url(&self) -> bool {
        match Regex::new(&URL_REGEX) {
            Ok(regexp) => regexp.is_match(&self.url),
            Err(err) => panic!("panicked whilst checking URL:\n{}", err)
        }
    }

    pub fn extract(&self) ->  Option<&str> { 

        match &self.url {
            _url if self.test_against(r"^https?\/\/") => self.handle_http(),
            _ => None
        }
    }
}