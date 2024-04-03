use axum::{
    extract::{Query, State},
    response::Response,
};
use serde::Deserialize;

use crate::{cached_fetch, json_to_reponse, AppState, Params, Source};

use super::*;

#[derive(Deserialize)]
pub struct KrakenQuery {
    since: Option<usize>,
}

pub async fn ohlcv_handler(State(state): State<AppState>, query: Query<KrakenQuery>) -> Response {
    let params = Params::new(
        &format!(
            "OHLC?pair=XBTUSD&interval=1440&since={}",
            query.since.unwrap_or(0)
        ),
        None,
    );

    json_to_reponse(
        cached_fetch(state.cache, params.to_key(), || {
            fetch_and_parse(&state.client, &params)
        })
        .await,
        Source {
            name: "Kraken".to_owned(),
            url: "https://www.kraken.com".to_owned(),
            color: "#6366f1".to_owned(),
        },
        None,
    )
}
