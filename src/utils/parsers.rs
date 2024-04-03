use regex::Regex;
use serde_json::{json, Value};

use crate::merge_dates_and_values;

pub fn parse_html(html: &str, regex_x: &str, regex_y: &str, index: Option<usize>) -> Value {
    let regex_x = Regex::new(regex_x).unwrap();
    let regex_y = Regex::new(regex_y).unwrap();

    let parse = |regex: Regex| -> Option<Value> {
        regex
            .captures_iter(html)
            .map(|capture| capture.extract())
            .nth(index.unwrap_or(0))
            .map(|(_, [values])| serde_json::from_str(values).unwrap())
    };

    let dates = parse(regex_x).unwrap();
    let dates = dates.as_array().unwrap();

    let values = parse(regex_y)
        .unwrap()
        .as_array()
        .unwrap()
        .iter()
        .map(|v| {
            if let Some(str) = v.as_str() {
                json!(str.parse::<f64>().unwrap_or(0.0))
            } else {
                v.clone()
            }
        })
        .collect::<Vec<_>>();

    let dataset = merge_dates_and_values(dates, &values);

    json!(dataset)
}
