use rusqlite::types::{FromSql, FromSqlError, FromSqlResult, ValueRef};
use serde::Serialize;
use std::str::FromStr;
use strum_macros::{EnumDiscriminants, EnumString, AsRefStr};

#[derive(Debug, Clone, Serialize, EnumString, EnumDiscriminants, AsRefStr)]
#[strum_discriminants(derive(Serialize))]
#[serde(rename_all = "camelCase")]
pub enum ArtType {
    #[strum(serialize = "plant")]
    Plant,
    #[strum(serialize = "portrait")]
    Portrait,
    #[strum(serialize = "vector")]
    Vector,
}

impl FromSql for ArtType {
    fn column_result(value: ValueRef<'_>) -> FromSqlResult<Self> {
        match value {
            ValueRef::Text(v) => {
                Ok(ArtType::from_str(&String::from_utf8_lossy(v).to_string()).unwrap())
            }
            _ => Err(FromSqlError::InvalidType),
        }
    }
}

#[derive(Debug, Serialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct ArtPiece {
    pub id: String,
    pub title: String,
    pub medium: String,
    pub size: String,
    pub src: String,
    pub year: i16,
    pub position: i8,
    pub price: i64,
    pub sold: i8,
    pub art_type: ArtType,
}
