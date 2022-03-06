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
                let len = capture.len();
                let mut data = Vec::new();
                for offset in 1..len {
                    data.push(capture.get(offset).unwrap().as_str());
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
