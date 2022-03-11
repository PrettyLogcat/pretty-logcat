use std::collections::{hash_map::DefaultHasher, HashMap};
use std::hash::{Hash, Hasher};
use std::rc::Rc;

use rand::seq::IteratorRandom;

use crate::configuration::{Colors, ConditionalFormated, Configuration, RandomicFormated, Theme};
use crate::data::Data;
use crate::pretty::Pretty;
use crate::style::{DynamicStyleBuilder, RefStyleBuilder, Style};

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

impl RefStyleBuilder for Theme {
    fn to_style(&self) -> Style {
        let mut dynamic_style = DynamicStyleBuilder::new();

        if let Some(ref background) = self.background {
            dynamic_style.add_background(format!("{}", background));
        }

        if let Some(ref foreground) = self.foreground {
            dynamic_style.add_foreground(format!("{}", foreground))
        }

        if let Some(ref modifiers) = self.modifiers {
            for modifier in modifiers {
                dynamic_style.add_modifier(format!("{}", modifier))
            }
        }

        Style::new(dynamic_style)
    }
}

impl RefStyleBuilder for Colors {
    fn to_style(&self) -> Style {
        let mut rng = rand::thread_rng();

        let mut dynamic_style_builder = DynamicStyleBuilder::new();

        if let Some(ref backgrounds) = self.backgrounds {
            dynamic_style_builder
                .add_background(format!("{}", backgrounds.iter().choose(&mut rng).unwrap()))
        }

        if let Some(ref foregrounds) = self.foregrounds {
            dynamic_style_builder
                .add_foreground(format!("{}", foregrounds.iter().choose(&mut rng).unwrap()))
        }

        if let Some(ref modifiers) = self.modifiers {
            dynamic_style_builder
                .add_modifier(format!("{}", modifiers.iter().choose(&mut rng).unwrap()))
        }

        Style::new(dynamic_style_builder)
    }
}

impl RefStyleBuilder for Option<&Theme> {
    fn to_style(&self) -> Style {
        self.unwrap().to_style()
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
    pub fn new(configuration: Configuration) -> PrettyManager {
        PrettyManager {
            cache: HashMap::new(),
            colors: configuration.colors,
            themes: configuration.themes,
            randomic_formated: configuration.randomic_formated,
            conditional_formated: configuration.conditional_formated,
            fixed_formated: configuration.fixed_formated,
        }
    }

    pub fn generate_pretties<'a>(&mut self, data: Data<'a>) -> Vec<Pretty<'a>> {
        let mut pretties = Vec::new();
        for (index, item) in data.iter().enumerate() {
            let index_str = &format!("{}", index);
            let (rc_style, option_hash) = match self.randomic_formated.indexes.get(&index) {
                Some(_) => {
                    let hash = item.to_hash();
                    match self.cache.get(&hash) {
                        Some(style) => (Rc::clone(style), None),
                        None => (Rc::new(self.colors.to_style()), Some(hash)),
                    }
                }
                None => match self.randomic_formated.indexes_to_repeat.get(index_str) {
                    Some(from_where) => {
                        let content = data[*from_where];
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
                                    let style = self.themes.get(theme_str).to_style();
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
                                                let style = self.themes.get(theme_str).to_style();
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
            if let Some(hash) = option_hash {
                self.cache.insert(hash, Rc::clone(&rc_style));
            }
            pretties.push(Pretty::new(rc_style, item));
        }
        pretties
    }
}
