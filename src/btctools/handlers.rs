use axum::{extract::State, response::Response};

use crate::{cached_fetch, generic_to_reponse, AppState, Params, Source};

use super::*;

pub const ALT_MARKETCAP_KEY: &str = "marketcap-alt-chart";
pub const STABLE_MARKETCAP_KEY: &str = "marketcap-stable-chart";

pub async fn altcoins_marketcap_handler(state: State<AppState>) -> Response {
    base_handler(state, ALT_MARKETCAP_KEY).await
}

pub async fn stablecoins_marketcap_handler(state: State<AppState>) -> Response {
    base_handler(state, STABLE_MARKETCAP_KEY).await
}

async fn base_handler(State(state): State<AppState>, id: &str) -> Response {
    let params = Params::new(format!("{id}?period=all").as_str(), None);

    generic_to_reponse(
        cached_fetch(state.cache, params.to_key(), || {
            fetch_and_parse(&state.client, &params)
        })
        .await,
        Some(Source {
            name: "btctools".to_owned(),
            url: "https://btctools.io".to_owned(),
            color: "#2BC185".to_owned(),
        }),
        None,
        None,
    )
}
