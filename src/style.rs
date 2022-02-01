pub struct Style {
    pub background: Option<String>,
    pub foreground: Option<String>,
    pub modifiers: Option<Vec<String>>,
}

pub trait StyleBuilder {
    fn build(self) -> Style;
}

pub struct DynamicStyleBuilder {
    background: Option<String>,
    foreground: Option<String>,
    modifiers: Option<Vec<String>>,
}

impl DynamicStyleBuilder {
    pub fn new() -> DynamicStyleBuilder {
        DynamicStyleBuilder {
            background: None,
            foreground: None,
            modifiers: None,
        }
    }

    pub fn add_background(&mut self, background: String) {
        self.background = Some(background);
    }

    pub fn add_foreground(&mut self, foreground: String) {
        self.foreground = Some(foreground);
    }

    pub fn add_modifier(&mut self, modifier: String) {
        match self.modifiers {
            Some(ref mut vector) => vector.push(modifier),
            None => self.modifiers = Some(vec![modifier]),
        }
    }
}

impl StyleBuilder for DynamicStyleBuilder {
    fn build(self) -> Style {
        Style {
            background: self.background,
            foreground: self.foreground,
            modifiers: self.modifiers,
        }
    }
}

impl Style {
    pub fn new<T: StyleBuilder>(builder: T) -> Style {
        builder.build()
    }
}
