use rand::Rng;
use std::collections::{hash_map::DefaultHasher, HashMap};
use std::hash::{Hash, Hasher};
use std::rc::Rc;

use crate::configuration::PreFormated;
use crate::data::Data;
use crate::pretty::Pretty;
use crate::style::{DynamicStyleBuilder, Style};

pub struct PrettyManager {
    cache: HashMap<u64, Rc<Style>>,
    pre_formated: HashMap<String, PreFormated>,
    data_offset_lookup: usize,
}

impl PrettyManager {
    pub fn new(
        pre_formated: HashMap<String, PreFormated>,
        data_offset_lookup: usize,
    ) -> PrettyManager {
        PrettyManager {
            cache: HashMap::<u64, Rc<Style>>::new(),
            pre_formated: pre_formated,
            data_offset_lookup: data_offset_lookup,
        }
    }

    pub fn generate_pretties(&mut self, data: &Data) -> Vec<Pretty> {
        let mut pretties = Vec::new();
        for (index, item) in data.contents.iter().enumerate() {
            let style: Rc<Style> = if index == self.data_offset_lookup {
                let hash = PrettyManager::calculate_hash(item);
                match self.cache.get(&hash) {
                    Some(style) => Rc::clone(style),
                    None => {
                        let style = Rc::new(PrettyManager::generate_style());
                        self.cache.insert(hash, Rc::clone(&style));
                        style
                    }
                }
            } else {
                let custom_key = format!("{}{}", index, item);
                let hash = PrettyManager::calculate_hash(&custom_key);
                match self.pre_formated.get(&custom_key) {
                    Some(pre_formated) => match self.cache.get(&hash) {
                        Some(style) => Rc::clone(style),
                        None => {
                            let mut dynamic_style = DynamicStyleBuilder::new();
                            dynamic_style
                                .add_background(format!("48;5;{}", pre_formated.background));
                            dynamic_style
                                .add_foreground(format!("38;5;{}", pre_formated.foreground));
                            let style = Rc::new(Style::new(dynamic_style));
                            self.cache.insert(hash, Rc::clone(&style));
                            style
                        }
                    },
                    None => {
                        let mut dynamic_style = DynamicStyleBuilder::new();
                        dynamic_style.add_background(format!("{}", 0));
                        dynamic_style.add_foreground(format!("{}", 0));
                        let style = Rc::new(Style::new(dynamic_style));
                        self.cache.insert(hash, Rc::clone(&style));
                        style
                    }
                }
            };
            let pretty = Pretty::new(style, item.to_string());
            pretties.push(pretty)
        }
        pretties
    }

    fn calculate_hash(data: &String) -> u64 {
        let mut hasher = DefaultHasher::new();
        data.hash(&mut hasher);
        hasher.finish()
    }

    fn generate_style() -> Style {
        let mut dynamic_style = DynamicStyleBuilder::new();
        //dynamic_style.add_background(PrettyManager::generate_background());
        dynamic_style.add_foreground(PrettyManager::generate_foreground());
        Style::new(dynamic_style)
    }

    fn generate_random() -> u8 {
        let mut rng = rand::thread_rng();
        rng.gen()
    }

    fn generate_foreground() -> String {
        format!("38;5;{}", PrettyManager::generate_random())
    }

    fn generate_background() -> String {
        format!("48;5;{}", PrettyManager::generate_random())
    }

    fn generate_modifier() -> String {
        let num = PrettyManager::generate_random() % 9;
        format!("{}", if num == 0 { 1 } else { num })
    }
}
