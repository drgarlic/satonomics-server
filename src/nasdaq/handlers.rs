use axum::{extract::State, response::Response, Json};

use crate::{cached_fetch, json_to_reponse, AppState, Params};

use super::*;

pub async fn gold_price_handler(state: State<AppState>) -> Response {
    base_handler(state, "11304240").await
}

async fn base_handler(State(state): State<AppState>, id: &str) -> Response {
    let params = Params::new(format!("/{id}/data").as_str(), None);

    json_to_reponse(
        Json(
            cached_fetch(state.cache, params.path.to_string(), || {
                fetch_and_parse(&state.client, &params)
            })
            .await,
        ),
        None,
    )
}
