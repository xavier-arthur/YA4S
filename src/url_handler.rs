use regex::Regex;

pub struct UrlHandler {
    pub url: String
}

impl UrlHandler {
    fn test_against(&self, re: &str) -> bool {
        match Regex::new(&re) {
            Ok(regexp) => regexp.is_match(&self.url),
            _ => panic!("couldn't parse regex")
        }
    }

    fn handle_http(&self) -> Option<&str> {
        Some("asdf") 
    }

    pub fn extract(&self) ->  Option<&str> { 

        match &self.url {
            _url if self.test_against("^https?\\/\\/") => self.handle_http(),
            _ => None
        }
    }
}