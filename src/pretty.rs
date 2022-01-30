use std::fmt::{Formatter, Result, Display};
use std::rc::Rc;

use crate::style::Style;

pub struct Pretty {
    pub text: String,
    pub style: Rc<Style>,
}

impl Pretty {
    pub fn new(text: String, style: Rc<Style>) -> Pretty {
        Pretty {
            text: text,
            style: style,
        }
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
        match self.style.set {
            Some(ref vec) => {
                for value in vec {
                    style_string.push_str(&value[..]);
                    style_string.push_str(";");
                }
            }
            None => (),
        }
        style_string.pop();
        f.write_str(
            &format!(
                "\x1b[{style}m{text}\x1b[0m",
                style = style_string,
                text = &self.text,
            )[..]
        )
    }
}
