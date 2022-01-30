pub struct Style {
    pub background: Option<String>,
    pub foreground: Option<String>,
    pub set: Option<Vec<String>>,
}

pub trait StyleBuilder {
    fn build(self) -> Style;
}

pub struct DynamicStyleBuilder {
    background: Option<String>,
    foreground: Option<String>,
    set: Option<Vec<String>>,
}

impl DynamicStyleBuilder {
    pub fn new() -> DynamicStyleBuilder {
        DynamicStyleBuilder {
            background: None,
            foreground: None,
            set: None,
        }
    }

    pub fn add_background(mut self, background: String) -> DynamicStyleBuilder {
        self.background = Some(background);
        self
    }

    pub fn add_foreground(mut self, foreground: String) -> DynamicStyleBuilder {
        self.foreground = Some(foreground);
        self
    }

    pub fn add_set_item(mut self, set_item: String) -> DynamicStyleBuilder {
        match self.set {
            Some(ref mut vector) => vector.push(set_item),
            None => self.set = Some(vec![set_item]),
        }
        self
    }
}
impl StyleBuilder for DynamicStyleBuilder {
    fn build(self) -> Style {
        Style {
            background: self.background,
            foreground: self.foreground,
            set: self.set,
        }
    }
}

impl Style {
    pub fn new<T: StyleBuilder>(builder: T) -> Style {
        builder.build()
    }
}