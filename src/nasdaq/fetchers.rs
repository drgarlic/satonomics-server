use reqwest::Client;
use serde_json::Value;

use crate::{fetch_json, Params};

use super::*;

pub async fn fetch_and_parse(client: &Client, params: &Params) -> color_eyre::Result<Value> {
    let json = fetch_json(
        client,
        &format!("https://data.nasdaq.com/api/v3/datasets/{}", params.path),
        &params.body,
    )
    .await?;

    Ok(parse_dataset_from_json(&json))
}
