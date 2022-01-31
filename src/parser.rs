use regex::Regex;

use crate::data::{Data, DataBuilder};

pub struct Parser {
    regex: Regex,
}

pub struct ParserData {
    data: Vec<String>,
}

impl Parser {
    pub fn new(regex: &str) -> Parser {
        Parser {
            regex: Regex::new(regex).unwrap(),
        }
    }

    pub fn parse(&self, data: &String) -> Option<ParserData> {
        match self.regex.captures(&data) {
            Some(capture) => {
                let len = capture.len();
                let mut data = Vec::new();
                for offset in 1..len {
                    data.push(String::from(&capture[offset as usize]));
                }
                Some(ParserData { data })
            }
            None => None,
        }
    }
}

impl DataBuilder for ParserData {
    fn build(self) -> Data {
        Data {
            contents: self.data,
        }
    }
}
