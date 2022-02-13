use std::collections::{HashMap, HashSet};

use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub struct Colors {
    pub backgrounds: Option<Vec<u8>>,
    pub foregrounds: Option<Vec<u8>>,
    pub modifiers: Option<Vec<u8>>,
}

#[derive(Serialize, Deserialize)]
pub struct Theme {
    pub background: Option<u8>,
    pub foreground: Option<u8>,
    pub modifiers: Option<Vec<u8>>,
}

#[derive(Serialize, Deserialize)]
pub struct ConditionalFormated {
    pub from_where: usize,
    pub themes: HashMap<String, String>,
}

#[derive(Serialize, Deserialize)]
pub struct RandomicFormated {
    pub indexes: HashSet<usize>,
    pub indexes_to_repeat: HashMap<String, usize>,
}

#[derive(Serialize, Deserialize)]
pub struct Configuration {
    pub regex: String,
    pub colors: Colors,
    pub themes: HashMap<String, Theme>,
    pub randomic_formated: RandomicFormated,
    pub conditional_formated: HashMap<String, ConditionalFormated>,
    pub fixed_formated: HashMap<String, String>,
}
