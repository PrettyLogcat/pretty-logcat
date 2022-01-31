use std::collections::HashMap;

use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub struct Colors {
    background: Vec<u16>,
    foreground: Vec<u16>,
    modifiers: Vec<u8>,
}

#[derive(Serialize, Deserialize)]
pub struct Regex {
    pub text: String,
    pub group_count: u8,
}

#[derive(Serialize, Deserialize)]
pub struct Theme {
    pub background: Option<u16>,
    pub foreground: Option<u16>,
    pub modifiers: Vec<u8>,
}

#[derive(Serialize, Deserialize)]
pub struct ConditionalFormated {
    pub comparission_offset: usize,
    pub themes: HashMap<String, String>
}

#[derive(Serialize, Deserialize)]
pub struct Configuration {
    pub regex: Regex,
    pub colors: Colors,
    pub themes: HashMap<String, Theme>,
    pub randomic_formated: HashMap<String, String>,
    pub conditional_formated: HashMap<String, ConditionalFormated>,
    pub fixed_formated: HashMap<String, String>,
}
