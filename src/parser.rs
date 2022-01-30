use regex::Regex;

use crate::data::{Data, DataBuilder};

pub struct Parser {
    regex: Regex,
}

pub struct ParserData {
    data: [String; 7],
}

impl Parser {
    pub fn new() -> Parser {
        Parser {
            regex: Regex::new(r"(\d{2}-\d{2})\s(\d{2}:\d{2}:\d{2}\.\d{3})\s+(\d+)\s+(\d+)\s(\w)\s(\w+\s{0,}:)\s+(.+)").unwrap()
        }
    }

    pub fn parse(&self, data: &String) -> Option<ParserData> {
        match self.regex.captures(&data[..]) {
            Some(capture) => Some(
                ParserData {
                    data: [
                        String::from(&capture[1]),
                        String::from(&capture[2]),
                        String::from(&capture[3]),
                        String::from(&capture[4]),
                        String::from(&capture[5]),
                        String::from(&capture[6]),
                        String::from(&capture[7]),
                    ],
                }
            ),
            None => None
        }
    }
}

impl DataBuilder for ParserData {
    fn build(&self) -> Data {
        let data = &self.data;
        Data {
            date: String::from(&data[0]),
            time: String::from(&data[1]),
            pid: String::from(&data[2]),
            tid: String::from(&data[3]),
            package_priority: String::from(&data[4]),
            tag: String::from(&data[5]),
            message: String::from(&data[6]),
        }
    }
}
