use serde::{Serialize, Deserialize};

#[derive(Debug, Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct Charge {
    pub art_id: String,
    pub token: String,
    pub email: String,
    pub price: i64,
}

#[derive(Serialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct ChargeResponse {
    pub success: bool,
}
