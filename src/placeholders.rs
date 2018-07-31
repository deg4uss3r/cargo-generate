use chrono::offset::Local;
use chrono::Datelike;
use chrono::ParseError;

pub fn get_date() -> Result<String, ParseError> {
    let dt = Local::now();

    let datetime = format!("{}{:02}{:02}", dt.year(), dt.month(), dt.day());

    Ok(datetime)
}