use axum::{
    http::header,
    response::{IntoResponse, Json, Response},
};
use serde::Serialize;

use crate::{Chunk, Source, CACHE_TIME};

const STALE_IF_ERROR: u64 = 604800; // 1 Week

#[derive(Serialize)]
struct WrappedDataset<T>
where
    T: Serialize,
{
    source: Source,
    chunk: Option<Chunk>,
    dataset: T,
}

pub fn generic_to_reponse<T>(
    generic: T,
    source: Option<Source>,
    chunk: Option<Chunk>,
    cache_time: Option<u64>,
) -> Response
where
    T: Serialize,
{
    let mut response = {
        if let Some(source) = source {
            Json(WrappedDataset {
                source,
                chunk,
                dataset: generic,
            })
            .into_response()
        } else {
            Json(generic).into_response()
        }
    };

    let headers = response.headers_mut();

    let max_age = cache_time.unwrap_or(CACHE_TIME);
    let stale_while_revalidate = 2 * max_age;

    headers.insert(header::ACCESS_CONTROL_ALLOW_ORIGIN, "*".parse().unwrap());
    headers.insert(header::ACCESS_CONTROL_ALLOW_HEADERS, "*".parse().unwrap());
    headers.insert(
        header::CACHE_CONTROL,
        format!(
            "public, max-age={max_age}, stale-while-revalidate={stale_while_revalidate}, stale-if-error={STALE_IF_ERROR}")
        .parse()
        .unwrap(),
    );
    headers.insert(header::CONTENT_TYPE, "application/json".parse().unwrap());

    response
}
