pub trait DataBuilder<'a> {
    fn build(self) -> Data<'a>;
}

pub struct Data<'a>(pub Vec<&'a str>);

impl<'a> Data<'a> {
    pub fn new<T: DataBuilder<'a>>(builder: T) -> Data<'a> {
        builder.build()
    }
}

impl<'a> ToString for Data<'a> {
    fn to_string(&self) -> String {
        self.0.join(" ")
    }
}
