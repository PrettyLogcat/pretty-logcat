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
    pub group_offset: u8,
}

#[derive(Serialize, Deserialize)]
pub struct PreFormated {
    pub background: u16,
    pub foreground: u16,
}

#[derive(Serialize, Deserialize)]
pub struct Configuration {
    pub regex: Regex,
    pub colors: Colors,
    pub pre_formated: HashMap<String, PreFormated>,
}
