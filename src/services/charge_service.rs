use super::super::models::charge::{Charge, ChargeResponse};
use super::super::services::art_service::{ArtServiceImpl, DynArtService};
use async_trait::async_trait;
use axum::extract::Json;
use std::sync::Arc;
use stripe::{Charge as StripeCharge, ChargeSourceParams, Client, CreateCharge, Currency, TokenId};

pub type DynChargeService = Arc<dyn ChargeService + Send + Sync>;

#[async_trait]
pub trait ChargeService {
    async fn create_charge(&self, Json(params): Json<Charge>) -> ChargeResponse;
}

pub struct ChargeServiceImpl {
    art_service: DynArtService,
    stripe_client: Client,
}

#[async_trait]
impl ChargeService for ChargeServiceImpl {
    async fn create_charge(&self, Json(params): Json<Charge>) -> ChargeResponse {
        let art_piece = self.art_service.get_art_piece(params.art_id).await;
        let mut art_piece = art_piece.unwrap();
        if art_piece.sold == 1 {
            return ChargeResponse { success: false };
        }

        let token = params.token.parse::<TokenId>().unwrap();
        let mut stripe_params = CreateCharge::new();
        stripe_params.amount = Some(art_piece.price);
        stripe_params.currency = Some(Currency::USD);
        stripe_params.source = Some(ChargeSourceParams::Token(token));
        stripe_params.description = Some("{art_piece.title} for {params.email}");
        stripe_params.receipt_email = Some(&params.email);

        let charge = StripeCharge::create(&self.stripe_client, stripe_params).await;
        if charge.is_ok() {
            art_piece.sold = 1;
            self.art_service.update_art_piece(art_piece).await;
            return ChargeResponse { success: true };
        }

        ChargeResponse {
            success: charge.is_ok(),
        }
    }
}

impl ChargeServiceImpl {
    pub fn new() -> Self {
        let stripe_secret =
            std::env::var("STRIPE_ACCESS_TOKEN").expect("STRIPE_ACCESS_TOKEN must be set");
        Self {
            art_service: Arc::new(ArtServiceImpl) as DynArtService,
            stripe_client: Client::new(stripe_secret),
        }
    }
}
