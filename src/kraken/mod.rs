use axum::{routing::get, Router};

pub mod fetchers;
pub mod handlers;
pub mod parsers;

use fetchers::*;
use handlers::*;
use parsers::*;

use crate::AppState;

pub fn add_kraken_routes(router: Router<AppState>) -> Router<AppState> {
    router.route("/date-to-price", get(ohlcv_handler))
}
