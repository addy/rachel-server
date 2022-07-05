use super::super::models::contact::Contact;
use super::super::services::contact_service::{ContactServiceImpl, DynContactService};
use axum::{extract::Extension, response::IntoResponse, routing::post, Json, Router};
use std::sync::Arc;

pub struct ContactController {
    service: DynContactService,
}

impl ContactController {
    pub fn new() -> Self {
        Self {
            service: Arc::new(ContactServiceImpl::new()) as DynContactService,
        }
    }

    pub fn router(&self) -> Router {
        async fn create_contact(
            Extension(service): Extension<DynContactService>,
            Json(params): Json<Contact>,
        ) -> impl IntoResponse {
            let response = service.create_contact(Json(params)).await;
            Json(response)
        }

        Router::new()
            .route("/contact", post(create_contact))
            .layer(Extension(self.service.clone()))
    }
}
