use std::ops::Deref;

pub struct Data<'a>(pub Vec<&'a str>);

pub trait DataBuilder<'a> {
    fn build(self) -> Data<'a>;
}

impl<'a> Data<'a> {
    pub fn new<T: DataBuilder<'a>>(builder: T) -> Data<'a> {
        builder.build()
    }
}

impl<'a> Deref for Data<'a> {
    type Target = Vec<&'a str>;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl<'a> ToString for Data<'a> {
    fn to_string(&self) -> String {
        self.0.join(" ")
    }
}
