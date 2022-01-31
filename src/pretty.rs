use std::fmt::{Display, Formatter, Result};
use std::rc::Rc;

use crate::style::Style;

pub struct Pretty {
    pub text: Option<String>,
    pub style: Rc<Style>,
}

impl Pretty {
    pub fn new(style: Rc<Style>, text: String) -> Pretty {
        Pretty {
            text: Some(text),
            style: style,
        }
    }
}

impl Display for Pretty {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        let mut style_string = String::new();
        match self.style.background {
            Some(ref value) => {
                style_string.push_str(&format!("48;5;{};", value));
            }
            None => (),
        }
        match self.style.foreground {
            Some(ref value) => {
                style_string.push_str(&format!("38;5;{};", value));
            }
            None => (),
        }
        match self.style.modifiers {
            Some(ref modifiers) => {
                for modifier in modifiers {
                    style_string.push_str(&format!("{};", modifier));
                }
            }
            None => (),
        }
        style_string.pop();
        let to_write = &format!(
            "\x1b[{style}m{data}\x1b[0m",
            style = style_string,
            data = match self.text {
                Some(ref text) => text.to_string(),
                None => "".to_string(),
            },
        );
        f.write_str(to_write)
    }
}
