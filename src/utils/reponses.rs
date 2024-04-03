use axum::{
    http::header,
    response::{IntoResponse, Json, Response},
    // Json,
};
use serde::Serialize;

use crate::{Source, CACHE_TIME};

const STALE_TIME: u64 = 604800; // 1 Week

#[derive(Serialize)]
struct WrappedDataset<T>
where
    T: Serialize,
{
    source: Source,
    dataset: T,
}

pub fn json_to_reponse<T>(dataset: T, source: Source, cache_time: Option<u64>) -> Response
where
    T: Serialize,
{
    let json = Json(WrappedDataset { source, dataset });

    let mut response = json.into_response();

    let headers = response.headers_mut();

    let cache_time = cache_time.unwrap_or(CACHE_TIME);

    headers.insert(header::ACCESS_CONTROL_ALLOW_ORIGIN, "*".parse().unwrap());
    headers.insert(header::ACCESS_CONTROL_ALLOW_HEADERS, "*".parse().unwrap());
    headers.insert(
        header::CACHE_CONTROL,
        format!(
            "public, max-age={cache_time}, stale-while-revalidate={CACHE_TIME}, stale-if-error={STALE_TIME}"
        )
        .parse()
        .unwrap(),
    );
    headers.insert(header::CONTENT_TYPE, "application/json".parse().unwrap());

    response
}
