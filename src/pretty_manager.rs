use rand::Rng;
use std::collections::{hash_map::DefaultHasher, HashMap};
use std::hash::{Hash, Hasher};
use std::rc::Rc;

use crate::configuration::{Colors, ConditionalFormated, RandomicFormated, Theme};
use crate::data::Data;
use crate::pretty::Pretty;
use crate::style::{DynamicStyleBuilder, Style};

trait Hashable {
    fn to_hash(&self) -> u64;
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
                    dynamic_style.add_modifier(format!("{}", *value))
                }
            }
            None => (),
        }
        Style::new(dynamic_style)
    }
}

impl Stylable for () {
    fn to_style(&self) -> Style {
        Style::new(DynamicStyleBuilder::new())
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
            cache: HashMap::<u64, Rc<Style>>::new(),
            colors: colors,
            themes: themes,
            randomic_formated: randomic_formated,
            conditional_formated: conditional_formated,
            fixed_formated: fixed_formated,
        }
    }

    pub fn generate_pretties(&mut self, data: &Data) -> Vec<Pretty> {
        let mut pretties = Vec::new();
        for (index, item) in data.0.iter().enumerate() {
            let index_str = &format!("{}", index);
            let style: Rc<Style> = match self.randomic_formated.indexes.get(&index) {
                Some(_) => {
                    let hash = item.to_hash();
                    match self.cache.get(&hash) {
                        Some(style) => Rc::clone(style),
                        None => {
                            let style = Rc::new(().to_style());
                            self.cache.insert(hash, Rc::clone(&style));
                            style
                        }
                    }
                }
                None => match self.randomic_formated.indexes_to_repeat.get(index_str) {
                    Some(from_where) => {
                        let content = &data.0[*from_where];
                        let hash = content.to_hash();
                        let rc_style = self.cache.get(&hash).unwrap();
                        Rc::clone(rc_style)
                    }
                    None => match self.fixed_formated.get(index_str) {
                        Some(theme_str) => {
                            let hash = theme_str.to_hash();
                            match self.cache.get(&hash) {
                                Some(rc_style) => Rc::clone(rc_style),
                                None => {
                                    let theme = self.themes.get(theme_str).unwrap();
                                    let rc_style = Rc::new(theme.to_style());
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
                                        let hash = theme_str.to_hash();
                                        match self.cache.get(&hash) {
                                            Some(rc_style) => Rc::clone(rc_style),
                                            None => {
                                                let theme = self.themes.get(theme_str).unwrap();
                                                let rc_style = Rc::new(theme.to_style());
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
}
