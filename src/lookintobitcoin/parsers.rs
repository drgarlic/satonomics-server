use serde_json::{json, Value};

use crate::merge_dates_and_values;

pub fn parse_dataset_from_json(json: &Value, index: Option<usize>) -> Value {
    let data = json
        .as_object()
        .unwrap()
        .get("response")
        .unwrap()
        .as_object()
        .unwrap()
        .get("chart")
        .unwrap()
        .as_object()
        .unwrap()
        .get("figure")
        .unwrap()
        .as_object()
        .unwrap()
        .get("data")
        .unwrap()
        .as_array()
        .unwrap()
        .get(index.unwrap_or(0))
        .unwrap()
        .as_object()
        .unwrap()
        .to_owned();

    let values = data.get("y").unwrap().as_array().unwrap();

    let len = values.len();

    let dates = &data.get("x").unwrap().as_array().unwrap()[..len];

    let dataset = merge_dates_and_values(dates, values);

    json!(dataset)
}
