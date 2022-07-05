use super::super::services::art_service::{DynArtService, ArtServiceImpl};
use axum::{
    extract::{Extension, Path},
    response::IntoResponse,
    routing::get,
    Json, Router,
};
use std::sync::Arc;

pub struct ArtController {
    service: DynArtService,
}

impl ArtController {
    pub fn new() -> Self {
        Self {
            service: Arc::new(ArtServiceImpl) as DynArtService,
        }
    }

    pub fn router(&self) -> Router {
        async fn art_list(Extension(service): Extension<DynArtService>) -> impl IntoResponse {
            let art_pieces = service.get_art_pieces().await;
            Json(art_pieces)
        }

        async fn art_show(
            Extension(service): Extension<DynArtService>,
            Path(id): Path<String>,
        ) -> impl IntoResponse {
            let art_piece = service.get_art_piece(id).await;
            Json(art_piece.unwrap())
        }

        Router::new()
            .route("/art", get(art_list))
            .route("/art/:id", get(art_show))
            .layer(Extension(self.service.clone()))
    }
}
