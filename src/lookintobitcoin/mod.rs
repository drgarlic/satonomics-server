use axum::{routing::get, Router};

mod fetchers;
mod handlers;
mod parsers;

use fetchers::*;
use handlers::*;
use parsers::*;

use crate::AppState;

pub fn add_lookintobitcoin_routes(router: Router<AppState>) -> Router<AppState> {
    router.route("/funding-rates", get(funding_rates_handler))
}
