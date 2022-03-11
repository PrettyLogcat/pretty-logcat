use std::fmt::{Display, Formatter, Result};
use std::rc::Rc;

use crate::style::Style;

pub struct Pretty<'a> {
    pub text: &'a str,
    pub style: Rc<Style>,
}

impl<'a> Pretty<'a> {
    pub fn new(style: Rc<Style>, text: &'a str) -> Pretty<'a> {
        Pretty { text, style }
    }
}

impl<'a> Display for Pretty<'a> {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        let mut string = String::new();

        if let Some(ref value) = self.style.background {
            string.push_str(&format!("48;5;{};", value));
        }

        if let Some(ref value) = self.style.foreground {
            string.push_str(&format!("38;5;{};", value));
        }

        if let Some(ref modifiers) = self.style.modifiers {
            for modifier in modifiers {
                string.push_str(&format!("{};", modifier))
            }
        }

        //last semicolon
        string.pop();
        let to_write = &format!(
            "\x1b[{style}m{data}\x1b[0m",
            style = string,
            data = self.text,
        );
        f.write_str(to_write)
    }
}
