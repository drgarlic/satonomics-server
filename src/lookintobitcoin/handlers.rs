use axum::{extract::State, response::Response};

use crate::{cached_fetch, generic_to_reponse, AppState, Params, Source};

use super::*;

pub async fn funding_rates_handler(state: State<AppState>) -> Response {
    base_handler(state, Params {
        path: "funding_rates".to_string(),
        body: Some("{\"output\":\"..chart.figure...exchange.options...resolution.disabled...resolution.value..\",\"outputs\":[{\"id\":\"chart\",\"property\":\"figure\"},{\"id\":\"exchange\",\"property\":\"options\"},{\"id\":\"resolution\",\"property\":\"disabled\"},{\"id\":\"resolution\",\"property\":\"value\"}],\"inputs\":[{\"id\":\"url\",\"property\":\"pathname\",\"value\":\"/charts/bitcoin-funding-rates/\"},{\"id\":\"currency\",\"property\":\"value\",\"value\":\"funding_rate_usd\"},{\"id\":\"exchange\",\"property\":\"value\",\"value\":\"average\"},{\"id\":\"resolution\",\"property\":\"value\",\"value\":\"1h\"}],\"changedPropIds\":[\"url.pathname\"]}".to_string()),
        index: Some(1),
    }).await
}

async fn base_handler(State(state): State<AppState>, params: Params) -> Response {
    generic_to_reponse(
        cached_fetch(state.cache, params.to_key(), || {
            fetch_and_parse(&state.client, &params)
        })
        .await,
        Some(Source {
            name: "look into bitcoin".to_owned(),
            url: "https://www.lookintobitcoin.com/".to_owned(),
            color: "#10A62B".to_owned(),
        }),
        None,
        None,
    )
}
