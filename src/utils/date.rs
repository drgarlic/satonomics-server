use chrono::{NaiveDate, TimeZone, Utc};

pub fn timestamp_to_naive_date(timestamp: u32) -> NaiveDate {
    Utc.timestamp_opt(i64::from(timestamp), 0)
        .unwrap()
        .date_naive()
}

// pub fn string_to_naive_date(date_str: &str) -> NaiveDate {
//     NaiveDate::parse_from_str(date_str, "%Y-%m-%d").unwrap()
// }
