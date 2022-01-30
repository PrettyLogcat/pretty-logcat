mod data;
mod parser;
mod pretty;
mod style;

use std::collections::{hash_map::DefaultHasher, HashMap};
use std::hash::{Hash, Hasher};
use std::io::{self};
use std::rc::Rc;

use regex::Regex;

use data::Data;
use parser::{Parser, ParserData};
use pretty::Pretty;
use style::{DynamicStyleBuilder, Style};

fn main() {
    let parser = Parser::new();
    let mut hash_style = HashMap::<u64, Rc<Style>>::new();

    let stdin = io::stdin();
    let mut buffer = String::new();

    loop {
        match stdin.read_line(&mut buffer) {
            Err(err) => panic!("Some error occurred: {}", err),
            Ok(_) => match parser.parse(&buffer) {
                Some(parsed) => {
                    let data = Data::new(&parsed);
                    println!("{:?}", data);
                }
                None => println!("{}", buffer),
            },
        };
        buffer.clear();
    }
}

fn calculate_hash(text: &String) -> u64 {
    let mut hasher = DefaultHasher::new();
    text.hash(&mut hasher);
    hasher.finish()
}

fn generate_style() -> Style {
    Style::new(DynamicStyleBuilder::new().add_foreground("48;5;2".to_string()))
}
