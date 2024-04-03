use axum::{routing::get, Router};

mod fetchers;
mod handlers;
mod parsers;

use fetchers::*;
pub use handlers::*;
use parsers::*;

use crate::AppState;

pub fn add_btctools_routes(router: Router<AppState>) -> Router<AppState> {
    router
        .route(
            "/date-to-altcoins-marketcap",
            get(altcoins_marketcap_handler),
        )
        .route(
            "/date-to-stablecoins-marketcap",
            get(stablecoins_marketcap_handler),
        )
}
