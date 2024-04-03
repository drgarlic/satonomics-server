use serde::Serialize;
use serde_json::{json, Value};

use crate::timestamp_to_naive_date;

#[derive(Debug, Serialize)]
pub struct CandlestickData {
    pub date: String,
    pub open: f32,
    pub high: f32,
    pub low: f32,
    pub close: f32,
    pub volume: f32,
}

pub fn parse_dataset_from_json(json: &Value) -> Value {
    let candlesticks: Vec<CandlestickData> = json
        .as_object()
        .unwrap()
        .get("result")
        .unwrap()
        .as_object()
        .unwrap()
        .get("XXBTZUSD")
        .unwrap()
        .as_array()
        .unwrap()
        .iter()
        .map(|value| {
            // [timestamp, open, high, low, close, vwap, volume count]

            let data = value.as_array().unwrap();

            let get = |index: usize| data.get(index).unwrap();

            let get_and_convert =
                |index: usize| get(index).as_str().unwrap().parse::<f32>().unwrap();

            let date = timestamp_to_naive_date(get(0).as_u64().unwrap() as u32).to_string();

            let open = get_and_convert(1);
            let high = get_and_convert(2);
            let low = get_and_convert(3);
            let close = get_and_convert(4);
            let volume = get_and_convert(6);

            CandlestickData {
                date,
                open,
                high,
                low,
                close,
                volume,
            }
        })
        .collect();

    json!(candlesticks)
}
