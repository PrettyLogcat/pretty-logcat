use std::collections::{hash_map::DefaultHasher, HashMap};
use std::hash::{Hash, Hasher};
use std::rc::Rc;

use rand::seq::IteratorRandom;

use crate::configuration::{Colors, ConditionalFormated, RandomicFormated, Theme};
use crate::data::Data;
use crate::pretty::Pretty;
use crate::style::{DynamicStyleBuilder, Style};

trait Hashable {
    fn to_hash(&self) -> u64;
}

impl Hashable for &str {
    fn to_hash(&self) -> u64 {
        let mut hasher = DefaultHasher::new();
        self.hash(&mut hasher);
        hasher.finish()
    }
}

impl Hashable for String {
    fn to_hash(&self) -> u64 {
        let mut hasher = DefaultHasher::new();
        self.hash(&mut hasher);
        hasher.finish()
    }
}

trait Stylable {
    fn to_style(&self) -> Style;
}

impl Stylable for Theme {
    fn to_style(&self) -> Style {
        let mut dynamic_style = DynamicStyleBuilder::new();
        match self.background {
            Some(value) => dynamic_style.add_background(format!("{}", value)),
            None => (),
        }
        match self.foreground {
            Some(value) => dynamic_style.add_foreground(format!("{}", value)),
            None => (),
        }
        match self.modifiers {
            Some(ref modifiers) => {
                for value in modifiers {
                    dynamic_style.add_modifier(format!("{}", value))
                }
            }
            None => (),
        }
        Style::new(dynamic_style)
    }
}

impl Stylable for Colors {
    fn to_style(&self) -> Style {
        let mut rng = rand::thread_rng();

        let mut dynamic_style_builder = DynamicStyleBuilder::new();

        match self.backgrounds {
            Some(ref backgrounds) => dynamic_style_builder
                .add_background(format!("{}", backgrounds.iter().choose(&mut rng).unwrap())),
            None => (),
        }

        match self.foregrounds {
            Some(ref foregrounds) => dynamic_style_builder
                .add_foreground(format!("{}", foregrounds.iter().choose(&mut rng).unwrap())),
            None => (),
        }

        match self.modifiers {
            Some(ref modifiers) => dynamic_style_builder
                .add_modifier(format!("{}", modifiers.iter().choose(&mut rng).unwrap())),
            None => (),
        }

        Style::new(dynamic_style_builder)
    }
}

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
            cache: HashMap::new(),
            colors: colors,
            themes: themes,
            randomic_formated: randomic_formated,
            conditional_formated: conditional_formated,
            fixed_formated: fixed_formated,
        }
    }

    pub fn generate_pretties<'a>(&mut self, data: Data<'a>) -> Vec<Pretty<'a>> {
        let mut pretties = Vec::new();
        for (index, item) in data.0.iter().enumerate() {
            let index_str = &format!("{}", index);
            let rc_style_option_hash: (Rc<Style>, Option<u64>) = match self
                .randomic_formated
                .indexes
                .get(&index)
            {
                Some(_) => {
                    let hash = item.to_hash();
                    match self.cache.get(&hash) {
                        Some(style) => (Rc::clone(style), None),
                        None => (Rc::new(self.colors.to_style()), Some(hash)),
                    }
                }
                None => match self.randomic_formated.indexes_to_repeat.get(index_str) {
                    Some(from_where) => {
                        let content = &data.0[*from_where];
                        let hash = content.to_hash();
                        let rc_style = self.cache.get(&hash).unwrap();
                        (Rc::clone(rc_style), None)
                    }
                    None => match self.fixed_formated.get(index_str) {
                        Some(theme_str) => {
                            let hash = theme_str.to_hash();
                            match self.cache.get(&hash) {
                                Some(rc_style) => (Rc::clone(rc_style), None),
                                None => {
                                    let style = self.themes.get(theme_str).unwrap().to_style();
                                    (Rc::new(style), Some(hash))
                                }
                            }
                        }
                        None => match self.conditional_formated.get(index_str) {
                            Some(conditional_formated) => {
                                let from_where = conditional_formated.from_where;
                                let content = data.0[from_where];
                                match conditional_formated.themes.get(content) {
                                    Some(theme_str) => {
                                        let hash = theme_str.to_hash();
                                        match self.cache.get(&hash) {
                                            Some(rc_style) => (Rc::clone(rc_style), None),
                                            None => {
                                                let style =
                                                    self.themes.get(theme_str).unwrap().to_style();
                                                (Rc::new(style), Some(hash))
                                            }
                                        }
                                    }
                                    None => (Rc::new(Style::new(())), None),
                                }
                            }
                            None => (Rc::new(Style::new(())), None),
                        },
                    },
                },
            };
            match rc_style_option_hash.1 {
                Some(hash) => {
                    self.cache.insert(hash, Rc::clone(&rc_style_option_hash.0));
                }
                None => (),
            };
            pretties.push(Pretty::new(rc_style_option_hash.0, item));
        }
        pretties
    }
}
