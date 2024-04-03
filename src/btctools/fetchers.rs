use reqwest::Client;
use serde_json::Value;

use crate::{fetch_json, Params};

use super::*;

pub async fn fetch_and_parse(client: &Client, params: &Params) -> color_eyre::Result<Value> {
    let json = fetch_json(
        client,
        &format!("https://api.btctools.io/api/{}", params.path),
        &params.body,
    )
    .await?;

    let mut value = parse_dataset_from_json(&json)?;

    // Fix the dip in stable market cap
    if params.path == STABLE_MARKETCAP_KEY {
        let to_insert = value
            .as_object()
            .unwrap()
            .get("2024-02-02")
            .unwrap()
            .clone();

        value
            .as_object_mut()
            .unwrap()
            .insert("2024-02-01".to_string(), to_insert);
    }

    Ok(value)
}
