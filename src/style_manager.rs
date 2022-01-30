use rand::Rng;
use std::collections::{hash_map::DefaultHasher, HashMap};
use std::hash::{Hash, Hasher};
use std::rc::Rc;

use crate::style::{DynamicStyleBuilder, Style};

pub struct StyleManager {
    cache: HashMap<u64, Rc<Style>>,
}

impl StyleManager {
    pub fn new() -> StyleManager {
        StyleManager {
            cache: HashMap::<u64, Rc<Style>>::new(),
        }
    }

    pub fn get_style(&mut self, data: &String) -> Rc<Style> {
        let hash = StyleManager::calculate_hash(data);
        match self.cache.get(&hash) {
            Some(style) => Rc::clone(style),
            None => {
                let new_style = Rc::new(StyleManager::generate_style());
                self.cache.insert(hash, Rc::clone(&new_style));
                new_style
            }
        }
    }

    fn calculate_hash(data: &String) -> u64 {
        let mut hasher = DefaultHasher::new();
        data.hash(&mut hasher);
        hasher.finish()
    }

    fn generate_style() -> Style {
        Style::new(
            DynamicStyleBuilder::new()
                .add_background(StyleManager::generate_background())
                .add_foreground(StyleManager::generate_foreground())
                .add_modifier(StyleManager::generate_modifier()),
        )
    }

    fn generate_random() -> u8 {
        let mut rng = rand::thread_rng();
        rng.gen()
    }

    fn generate_foreground() -> String {
        format!("48;5;{}", StyleManager::generate_random())
    }

    fn generate_background() -> String {
        format!("38;5;{}", StyleManager::generate_random())
    }

    fn generate_modifier() -> String {
        let num = StyleManager::generate_random() % 9;
        format!("{}", if num == 0 { 1 } else { num })
    }
}
