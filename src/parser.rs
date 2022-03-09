use regex::Regex;

use std::ops::Deref;

use crate::data::{Data, DataBuilder};

pub struct Parser(Regex);

impl Deref for Parser {
    type Target = Regex;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

pub struct ParserData<'a>(Vec<&'a str>);

impl<'a> Deref for ParserData<'a> {
    type Target = Vec<&'a str>;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl<'a> From<ParserData<'a>> for Vec<&'a str> {
    fn from(parser_data: ParserData<'a>) -> Self {
        parser_data.0
    }
}

impl Parser {
    pub fn new(regex: &str) -> Parser {
        Parser(Regex::new(regex).unwrap())
    }

    pub fn parse<'a>(&self, data: &'a str) -> Option<ParserData<'a>> {
        match self.captures(&data) {
            Some(capture) => {
                let mut vec = Vec::new();
                let mut sub_capture_matches = capture.iter();
                //Skip first capture because it's always full string
                sub_capture_matches.next();
                while let Some(Some(some_match)) = sub_capture_matches.next() {
                    vec.push(some_match.as_str());
                }
                Some(ParserData(vec))
            }
            None => None,
        }
    }
}

impl<'a> DataBuilder<'a> for ParserData<'a> {
    fn build(self) -> Data<'a> {
        //I dont know why From its not working
        Data(From::from(self))
    }
}
