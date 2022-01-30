pub trait DataBuilder {
    fn build(&self) -> Data;
}

#[derive(Debug)]
pub struct Data {
    pub date: String,
    pub time: String,
    pub pid: String,
    pub tid: String,
    pub package_priority: String,
    pub tag: String,
    pub message: String
}

impl Data {
    pub fn new<T : DataBuilder>(builder: &T) -> Data {
        builder.build()
    }
}