use serde_json::{json, Map, Value};

use crate::timestamp_to_naive_date;

pub fn parse_dataset_from_json(json: &Value) -> color_eyre::Result<Value> {
    Ok(json!(json
        .as_object()
        .unwrap()
        .get("data")
        .unwrap()
        .as_array()
        .unwrap()
        .iter()
        .fold(Map::new(), |mut dataset, value| {
            let value = value.as_object().unwrap();

            let date =
                timestamp_to_naive_date((value.get("t").unwrap().as_u64().unwrap() / 1000) as u32)
                    .to_string();

            let value = value.get("v").unwrap().clone().as_f64().unwrap() * 1_000_000_000.0;

            dataset.insert(date, json!(value));

            dataset
        })))
}
