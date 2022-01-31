use regex::Regex;

use crate::data::{Data, DataBuilder};

pub struct Parser {
    regex: Regex,
    group_count: u8,
}

pub struct ParserData {
    data: Vec<String>,
}

impl Parser {
    pub fn new(regex: &str, group_count: u8) -> Parser {
        Parser {
            regex: Regex::new(regex).unwrap(),
            group_count: group_count,
        }
    }

    pub fn parse(&self, data: &String) -> Option<ParserData> {
        match self.regex.captures(&data) {
            Some(capture) => {
                let mut data = Vec::new();
                for offset in 1..self.group_count + 1 {
                    data.push(String::from(&capture[offset as usize]));
                }
                Some(ParserData { data: data })
            }
            None => None,
        }
    }
}

impl DataBuilder for ParserData {
    fn build(self) -> Data {
        Data { contents: self.data }
    }
}
