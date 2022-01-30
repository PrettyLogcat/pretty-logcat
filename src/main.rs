mod pretty;
mod style;

use std::collections::{hash_map::DefaultHasher, HashMap};
use std::hash::{Hash, Hasher};
use std::io::{self};
use std::rc::Rc;

use pretty::Pretty;
use style::{DynamicStyleBuilder, Style};

fn main() {
    let mut hash_style = HashMap::<u64, Rc<Style>>::new();

    let generate_hash = |text: &String| -> u64 {
        let mut hasher = DefaultHasher::new();
        text.hash(&mut hasher);
        hasher.finish()
    };

    let generate_new_style = || -> Style {
        Style::new(
            DynamicStyleBuilder::new()
                .add_background("38;5;1".to_string())
                .add_foreground("48;5;2".to_string())
                .add_set_item("1".to_string())
                .add_set_item("4".to_string())
                .add_set_item("5".to_string())
                .add_set_item("7".to_string()),
        )
    };

    let text = "Another beautiful text".to_string();

    for _ in 1..5 {
        let hash = generate_hash(&text);

        let style = match hash_style.get(&hash) {
            Some(style) => Rc::clone(style),
            None => {
                let new_style = Rc::new(generate_new_style());
                hash_style.insert(hash, Rc::clone(&new_style));
                new_style
            },
        };

        let pretty = Pretty::new(text.clone(), style);
        println!("{}", pretty);
    }

    // let stdin = io::stdin();

    // let mut buffer = String::new();

    // match stdin.read_line(&mut buffer) {
    //     Err(err) => panic!("Some error occurred: {}", err),
    //     Ok(size) => println!("{} {}", buffer, size)
    // };
}
