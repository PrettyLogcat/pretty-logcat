pub trait DataBuilder {
    fn build(&self) -> Data;
}

pub struct Data {
    pub date: String,
    pub time: String,
    pub pid: String,
    pub tid: String,
    pub package_priority: String,
    pub tag: String,
    pub message: String,
}

impl Data {
    pub fn new<T: DataBuilder>(builder: &T) -> Data {
        builder.build()
    }
}

impl ToString for Data {
    fn to_string(&self) -> String {
        format!(
            "{} {} {:0>4} {:0>4} {} {} {}",
            self.date, self.time, self.pid, self.tid, self.package_priority, self.tag, self.message
        )
    }
}
