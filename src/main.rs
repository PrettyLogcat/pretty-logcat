mod configuration;
mod data;
mod parser;
mod pretty;
mod pretty_manager;
mod style;

use std::{fs::read_to_string, io::stdin};

use serde_json;

use configuration::Configuration;
use data::Data;
use parser::Parser;
use pretty_manager::PrettyManager;

fn main() {
    let file = read_to_string("./configuration.json").unwrap();
    let configuration: Configuration = serde_json::from_str(&file).unwrap();

    let stdin = stdin();
    let mut buffer = String::new();

    let parser = Parser::new(&configuration.regex);

    let mut pretty_manager = PrettyManager::new(
        configuration.colors,
        configuration.themes,
        configuration.randomic_formated,
        configuration.conditional_formated,
        configuration.fixed_formated,
    );

    loop {
        match stdin.read_line(&mut buffer) {
            Err(err) => panic!("Some error occurred: {}", err),
            Ok(bytes) => {
                if bytes == 0 {
                    break;
                }
                match parser.parse(&buffer) {
                    Some(parsed) => {
                        let data = Data::new(parsed);
                        let pretties = pretty_manager.generate_pretties(&data);
                        let mapped: Vec<String> =
                            pretties.into_iter().map(|p| p.to_string()).collect();
                        println!("{}", mapped.join(" "));
                    }
                    None => println!("{}", buffer),
                };
                buffer.clear()
            }
        };
    }
}
