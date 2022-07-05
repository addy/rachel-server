use super::super::models::charge::Charge;
use super::super::services::charge_service::{ChargeServiceImpl, DynChargeService};
use axum::{extract::Extension, response::IntoResponse, routing::post, Json, Router};
use std::sync::Arc;

pub struct ChargeController {
    service: DynChargeService,
}

impl ChargeController {
    pub fn new() -> Self {
        Self {
            service: Arc::new(ChargeServiceImpl::new()) as DynChargeService,
        }
    }

    pub fn router(&self) -> Router {
        async fn create_charge(
            Extension(service): Extension<DynChargeService>,
            Json(params): Json<Charge>,
        ) -> impl IntoResponse {
            let response = service.create_charge(Json(params)).await;
            Json(response)
        }

        Router::new()
            .route("/charge", post(create_charge))
            .layer(Extension(self.service.clone()))
    }
}
