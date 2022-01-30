use std::fmt::{Display, Formatter, Result};
use std::rc::Rc;

use crate::style::Style;

pub struct Pretty {
    pub text: Option<String>,
    pub style: Rc<Style>,
    pub pretty: Option<Box<Pretty>>,
}

impl Pretty {
    pub fn new(style: Rc<Style>) -> Pretty {
        Pretty {
            text: None,
            style: style,
            pretty: None,
        }
    }

    pub fn add_pretty(&mut self, pretty: Pretty) {
        self.pretty = Some(Box::new(pretty))
    }

    pub fn add_text(&mut self, text: String) {
        self.text = Some(text)
    }
}

impl Display for Pretty {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        let mut style_string = String::new();
        match self.style.background {
            Some(ref value) => {
                style_string.push_str(&value[..]);
                style_string.push_str(";");
            }
            None => (),
        }
        match self.style.foreground {
            Some(ref value) => {
                style_string.push_str(&value[..]);
                style_string.push_str(";");
            }
            None => (),
        }
        match self.style.modifiers {
            Some(ref modifiers) => {
                for modifier in modifiers {
                    style_string.push_str(&modifier[..]);
                    style_string.push_str(";");
                }
            }
            None => (),
        }
        style_string.pop();
        let to_write = &format!(
            "\x1b[{style}m{data}\x1b[0m",
            style = style_string,
            data = match self.pretty {
                Some(ref pretty) => pretty.to_string(),
                None => match self.text {
                    Some(ref text) => text.to_string(),
                    None => "".to_string(),
                },
            },
        )[..];
        f.write_str(to_write)
    }
}
