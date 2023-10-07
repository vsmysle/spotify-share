use axum::routing::post;
use std::net::SocketAddr;
use std::sync::Arc;

use tower_http::trace::{self, TraceLayer};

use crate::api::handlers;
use crate::state::AppState;
use crate::storage::local::LocalStorage;

pub async fn get_app(
    address: SocketAddr,
    app_state: Arc<AppState<LocalStorage>>,
) -> anyhow::Result<()> {
    let app = axum::Router::new()
        .route(
            "/tracks",
            post(handlers::tracks::add).get(handlers::tracks::list),
        )
        .route("/tracks/search", post(handlers::tracks::search))
        .route("/control/stop", post(handlers::control::stop))
        .route("/control/next", post(handlers::control::next))
        .route("/control/prev", post(handlers::control::prev))
        .route("/control/play", post(handlers::control::play))
        .layer(
            TraceLayer::new_for_http()
                .make_span_with(trace::DefaultMakeSpan::new().level(tracing::Level::INFO))
                .on_response(trace::DefaultOnResponse::new().level(tracing::Level::INFO)),
        )
        .with_state(app_state);

    axum::Server::bind(&address)
        .serve(app.into_make_service())
        .await?;
    Ok(())
}
