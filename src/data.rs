pub trait DataBuilder {
    fn build(self) -> Data;
}

pub struct Data(pub Vec<String>);

impl Data {
    pub fn new<T: DataBuilder>(builder: T) -> Data {
        builder.build()
    }
}

impl ToString for Data {
    fn to_string(&self) -> String {
        self.0.join(" ")
    }
}
