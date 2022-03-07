use regex::Regex;

use crate::data::{Data, DataBuilder};

pub struct Parser(Regex);

pub struct ParserData<'a>(Vec<&'a str>);

impl Parser {
    pub fn new(regex: &str) -> Parser {
        Parser(Regex::new(regex).unwrap())
    }

    pub fn parse<'a>(&self, data: &'a str) -> Option<ParserData<'a>> {
        match self.0.captures(&data) {
            Some(capture) => {
                let mut data = Vec::new();
                let mut sub_capture_matches = capture.iter();
                //Skip first capture because it's always full string
                sub_capture_matches.next();
                while let Some(option_match) = sub_capture_matches.next() {
                    if let Some(some_match) = option_match {
                        data.push(some_match.as_str())
                    }
                }
                Some(ParserData(data))
            }
            None => None,
        }
    }
}

impl<'a> DataBuilder<'a> for ParserData<'a> {
    fn build(self) -> Data<'a> {
        Data(self.0)
    }
}
