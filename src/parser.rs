use regex::Regex;

use crate::data::{Data, DataBuilder};

pub struct Parser(Regex);

pub struct ParserData(Vec<String>);

impl Parser {
    pub fn new(regex: &str) -> Parser {
        Parser(Regex::new(regex).unwrap())
    }

    pub fn parse(&self, data: &String) -> Option<ParserData> {
        match self.0.captures(&data) {
            Some(capture) => {
                let len = capture.len();
                let mut data = Vec::new();
                for offset in 1..len {
                    data.push(String::from(&capture[offset as usize]));
                }
                Some(ParserData(data))
            }
            None => None,
        }
    }
}

impl DataBuilder for ParserData {
    fn build(self) -> Data {
        Data(self.0)
    }
}
