use chrono::{DateTime, NaiveDateTime, Utc};

pub struct DataRawProject {
    name: String,
    deadline: String,
    author: String,
}

pub enum TypeDate {
    Creation,
    Deadline,
}

pub trait FormattableDate {
    fn format_date(&self) -> String;
}
impl FormattableDate for DateTime<Utc> {
    fn format_date(&self) -> String {
        self.format("%d-%m-%Y").to_string()
    }
}
impl FormattableDate for NaiveDateTime {
    fn format_date(&self) -> String {
        self.format("%d-%m-%Y").to_string()
    }
}
pub fn format_date<T: FormattableDate>(date: T) -> String {
    date.format_date()
}
