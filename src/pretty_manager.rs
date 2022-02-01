use rand::Rng;
use std::collections::{hash_map::DefaultHasher, HashMap};
use std::hash::{Hash, Hasher};
use std::rc::Rc;

use crate::configuration::{Colors, ConditionalFormated, RandomicFormated, Theme};
use crate::data::Data;
use crate::pretty::Pretty;
use crate::style::{DynamicStyleBuilder, Style};

pub struct PrettyManager {
    cache: HashMap<u64, Rc<Style>>,
    colors: Colors,
    themes: HashMap<String, Theme>,
    randomic_formated: RandomicFormated,
    conditional_formated: HashMap<String, ConditionalFormated>,
    fixed_formated: HashMap<String, String>,
}

impl PrettyManager {
    pub fn new(
        colors: Colors,
        themes: HashMap<String, Theme>,
        randomic_formated: RandomicFormated,
        conditional_formated: HashMap<String, ConditionalFormated>,
        fixed_formated: HashMap<String, String>,
    ) -> PrettyManager {
        PrettyManager {
            cache: HashMap::<u64, Rc<Style>>::new(),
            colors: colors,
            themes: themes,
            randomic_formated: randomic_formated,
            conditional_formated: conditional_formated,
            fixed_formated: fixed_formated,
        }
    }

    pub fn generate_pretties(&mut self, data: &Data) -> Vec<Pretty> {
        let mut pretties = Vec::<Pretty>::new();
        for (index, item) in data.0.iter().enumerate() {
            let index_str = &format!("{}", index);
            let style: Rc<Style> = match self.randomic_formated.indexes.get(index_str) {
                Some(_) => {
                    let hash = PrettyManager::calculate_hash(item);
                    match self.cache.get(&hash) {
                        Some(style) => Rc::clone(style),
                        None => {
                            let style = Rc::new(PrettyManager::generate_style());
                            self.cache.insert(hash, Rc::clone(&style));
                            style
                        }
                    }
                }
                None => match self.randomic_formated.indexes_to_repeat.get(index_str) {
                    Some(from_where) => {
                        let content = &data.0[*from_where];
                        let hash = PrettyManager::calculate_hash(content);
                        let rc_style = self.cache.get(&hash).unwrap();
                        Rc::clone(rc_style)
                    }
                    None => match self.fixed_formated.get(index_str) {
                        Some(theme_str) => {
                            let hash = PrettyManager::calculate_hash(theme_str);
                            match self.cache.get(&hash) {
                                Some(rc_style) => Rc::clone(rc_style),
                                None => {
                                    let theme: &Theme = self.themes.get(theme_str).unwrap();
                                    let mut dynamic_style = DynamicStyleBuilder::new();
                                    match theme.background {
                                        Some(value) => {
                                            dynamic_style.add_background(format!("{}", value))
                                        }
                                        None => (),
                                    }
                                    match theme.foreground {
                                        Some(value) => {
                                            dynamic_style.add_foreground(format!("{}", value))
                                        }
                                        None => (),
                                    }
                                    let rc_style = Rc::new(Style::new(dynamic_style));
                                    self.cache.insert(hash, Rc::clone(&rc_style));
                                    rc_style
                                }
                            }
                        }
                        None => match self.conditional_formated.get(index_str) {
                            Some(conditional_formated) => {
                                let from_where = conditional_formated.from_where;
                                let content = &data.0[from_where];
                                match conditional_formated.themes.get(content) {
                                    Some(theme_str) => {
                                        let hash = PrettyManager::calculate_hash(theme_str);
                                        match self.cache.get(&hash) {
                                            Some(rc_style) => Rc::clone(rc_style),
                                            None => {
                                                let theme: &Theme =
                                                    self.themes.get(theme_str).unwrap();
                                                let mut dynamic_style = DynamicStyleBuilder::new();
                                                match theme.background {
                                                    Some(value) => dynamic_style
                                                        .add_background(format!("{}", value)),
                                                    None => (),
                                                };
                                                match theme.foreground {
                                                    Some(value) => dynamic_style
                                                        .add_foreground(format!("{}", value)),
                                                    None => (),
                                                };
                                                let rc_style = Rc::new(Style::new(dynamic_style));
                                                self.cache.insert(hash, Rc::clone(&rc_style));
                                                rc_style
                                            }
                                        }
                                    }
                                    None => Rc::new(Style::new(DynamicStyleBuilder::new())),
                                }
                            }
                            None => Rc::new(Style::new(DynamicStyleBuilder::new())),
                        },
                    },
                },
            };
            pretties.push(Pretty::new(style, item.to_string()))
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
        format!("{}", PrettyManager::generate_random())
    }

    fn generate_background() -> String {
        format!("{}", PrettyManager::generate_random())
    }

    fn generate_modifier() -> String {
        let num = PrettyManager::generate_random() % 9;
        format!("{}", if num == 0 { 1 } else { num })
    }
}
