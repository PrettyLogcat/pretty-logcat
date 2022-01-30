mod data;
mod parser;
mod pretty;
mod style;
mod style_manager;

use std::io;

use data::Data;
use parser::Parser;
use pretty::Pretty;
use style_manager::StyleManager;

fn main() {
    let stdin = io::stdin();
    let mut buffer = String::new();

    let parser = Parser::new();
    let mut style_manager = StyleManager::new();

    loop {
        match stdin.read_line(&mut buffer) {
            Err(err) => panic!("Some error occurred: {}", err),
            Ok(_) => match parser.parse(&buffer) {
                Some(parsed) => {
                    let data = Data::new(&parsed);
                    let style = style_manager.get_style(&data.tag);
                    let mut pretty = Pretty::new(style);
                    pretty.add_text(data.to_string());
                    println!("{}", pretty);
                }
                None => println!("NOT: {}", buffer),
            },
        };
        buffer.clear();
    }
}
