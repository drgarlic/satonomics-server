use chrono::Duration;
use serde_json::{json, Map, Value};

use crate::{string_to_naive_date, BITCOIN_BIRTHDAY};

pub fn parse_dataset_from_json(json: &Value) -> Value {
    let bitcoin_birthday = string_to_naive_date(BITCOIN_BIRTHDAY);

    json!(json
        .as_object()
        .unwrap()
        .get("dataset_data")
        .unwrap()
        .as_object()
        .unwrap()
        .get("data")
        .unwrap()
        .as_array()
        .unwrap()
        .iter()
        .fold(Map::new(), |mut dataset, value| {
            let value = value.as_array().unwrap();

            let date = string_to_naive_date(value.first().unwrap().as_str().unwrap());

            if date - bitcoin_birthday >= Duration::zero() {
                let value = value.get(1).unwrap().clone();

                dataset.insert(date.to_string(), value);
            }

            dataset
        }))
}
