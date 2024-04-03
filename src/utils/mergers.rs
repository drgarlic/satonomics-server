use serde_json::{Map, Value};

pub fn merge_dates_and_values(dates: &[Value], values: &[Value]) -> Map<String, Value> {
    dates
        .iter()
        .enumerate()
        .fold(Map::new(), |mut dataset, (index, date)| {
            let date = date.as_str().unwrap().to_string();
            let value = values.get(index).unwrap().clone();

            dataset.insert(date, value);

            dataset
        })
}
