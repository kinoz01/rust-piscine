use chrono::{NaiveDate, Weekday, Datelike};

pub fn middle_day(year: u32) -> Option<Weekday> {
    if NaiveDate::from_ymd_opt(year as i32, 2, 29).is_some() { // if the year is a leap year
        return None;
    }

    NaiveDate::from_yo_opt(year as i32, 183).map(|d| d.weekday())
}
