mod configuration;
mod data;
mod parser;
mod pretty;
mod pretty_manager;
mod style;

use std::{fs, io};

use configuration::Configuration;
use data::Data;
use parser::Parser;
use pretty_manager::PrettyManager;

fn main() {
    let file = fs::read_to_string("./configuration.json").unwrap();
    let configuration: Configuration = serde_json::from_str(&file).unwrap();

    let stdin = io::stdin();
    let mut buffer = String::new();

    let parser = Parser::new(&configuration.regex.text, configuration.regex.group_count);
    let mut pretty_manager = PrettyManager::new(
        configuration.themes,
        configuration.randomic_formated as usize,
        configuration.combined_formated,
        configuration.repeated_formated,
        configuration.fixed_formated
    );

    loop {
        match stdin.read_line(&mut buffer) {
            Err(err) => panic!("Some error occurred: {}", err),
            Ok(_) => match parser.parse(&buffer) {
                Some(parsed) => {
                    let data = Data::new(parsed);
                    let pretties = pretty_manager.generate_pretties(&data);
                    let mapped: Vec<String> = pretties.into_iter().map(|p| p.to_string()).collect();
                    let result = mapped.join(" ");
                    println!("{}", result);
                }
                None => println!("{}", buffer),
            },
        };
        buffer.clear();
    }
}
