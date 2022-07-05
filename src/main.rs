use axum::Router;
use controllers::{
    art_controller::ArtController, charge_controller::ChargeController,
    contact_controller::ContactController,
};
use dotenv::dotenv;
use std::net::SocketAddr;
use tower::ServiceBuilder;
use tower_http::trace::TraceLayer;
use tracing_subscriber::{layer::SubscriberExt, util::SubscriberInitExt};

mod controllers;
mod models;
mod services;
mod utils;

#[tokio::main]
async fn main() {
    dotenv().ok();
    tracing_subscriber::registry()
        .with(tracing_subscriber::EnvFilter::new(
            std::env::var("RUST_LOG").unwrap_or_else(|_| "server=debug,tower_http=debug".into()),
        ))
        .with(tracing_subscriber::fmt::layer())
        .init();

    // Controllers
    let art_controller = ArtController::new();
    let charge_controller = ChargeController::new();
    let contact_controller = ContactController::new();

    // Routing & Layers
    let routes = Router::new()
        .merge(art_controller.router())
        .merge(charge_controller.router())
        .merge(contact_controller.router())
        .layer(
            ServiceBuilder::new()
                .layer(TraceLayer::new_for_http())
                .into_inner(),
        );

    let app = Router::new().nest("/api/v1", routes);

    let addr = SocketAddr::from(([127, 0, 0, 1], 3000));
    tracing::debug!("listening on {}", addr);
    axum::Server::bind(&addr)
        .serve(app.into_make_service())
        .await
        .unwrap();
}
