use axum::{routing::get, Router};

use crate::apis;

pub fn init_router() -> Router {
    Router::new().route("/", get(apis::handler))
}
