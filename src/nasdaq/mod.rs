use axum::{routing::get, Router};

mod fetchers;
mod handlers;
mod parsers;

use fetchers::*;
use handlers::*;
use parsers::*;

use crate::AppState;

pub fn add_nasdaq_routes(router: Router<AppState>) -> Router<AppState> {
    router.route("/gold-price", get(gold_price_handler))
}
