use super::super::models::art::ArtPiece;
use super::super::utils::database::Database;
use async_trait::async_trait;
use std::sync::Arc;

pub type DynArtService = Arc<dyn ArtService + Send + Sync>;

#[async_trait]
pub trait ArtService {
    async fn get_art_pieces(&self) -> Vec<ArtPiece>;
    async fn get_art_piece(&self, id: String) -> Option<ArtPiece>;
    async fn update_art_piece(&self, art_piece: ArtPiece) -> bool;
}

#[derive(Clone)]
pub struct ArtServiceImpl;

#[async_trait]
impl ArtService for ArtServiceImpl {
    async fn get_art_pieces(&self) -> Vec<ArtPiece> {
        let db: Database = Database::new().unwrap();

        let mut stmt = db
            .conn
            .prepare(
                "
            SELECT
                id,
                title,
                medium,
                size,
                src,
                year,
                position,
                price,
                sold,
                art_type
            FROM art",
            )
            .unwrap();

        let mut art_pieces = Vec::new();
        let art_iter = stmt
            .query_map([], |row| {
                Ok(ArtPiece {
                    id: row.get(0).unwrap(),
                    title: row.get(1).unwrap(),
                    medium: row.get(2).unwrap(),
                    size: row.get(3).unwrap(),
                    src: row.get(4).unwrap(),
                    year: row.get(5).unwrap(),
                    position: row.get(6).unwrap(),
                    price: row.get(7).unwrap(),
                    sold: row.get(8).unwrap(),
                    art_type: row.get(9).unwrap(),
                })
            })
            .unwrap();

        for art in art_iter {
            art_pieces.push(art.unwrap());
        }

        art_pieces
    }

    async fn get_art_piece(&self, id: String) -> Option<ArtPiece> {
        let db: Database = Database::new().unwrap();

        let mut stmt = db
            .conn
            .prepare(
                "
            SELECT
                id,
                title,
                medium,
                size,
                src,
                year,
                position,
                price,
                sold,
                art_type
            FROM art
            WHERE id = ?1",
            )
            .unwrap();

        let mut art_pieces = Vec::new();
        let art_iter = stmt
            .query_map([id], |row| {
                Ok(ArtPiece {
                    id: row.get(0).unwrap(),
                    title: row.get(1).unwrap(),
                    medium: row.get(2).unwrap(),
                    size: row.get(3).unwrap(),
                    src: row.get(4).unwrap(),
                    year: row.get(5).unwrap(),
                    position: row.get(6).unwrap(),
                    price: row.get(7).unwrap(),
                    sold: row.get(8).unwrap(),
                    art_type: row.get(9).unwrap(),
                })
            })
            .unwrap();

        for art in art_iter {
            art_pieces.push(art.unwrap());
        }

        if art_pieces.len() > 0 {
            return Some(art_pieces[0].clone());
        }

        None
    }

    async fn update_art_piece(&self, art_piece: ArtPiece) -> bool {
        let db: Database = Database::new().unwrap();

        let mut stmt = db
            .conn
            .prepare(
                "
            UPDATE art
            SET
                title = ?1,
                medium = ?2,
                size = ?3,
                src = ?4,
                year = ?5,
                position = ?6,
                price = ?7,
                sold = ?8,
                art_type = ?9
            WHERE id = ?10",
            )
            .unwrap();

        let result = stmt
            .execute([
                art_piece.title,
                art_piece.medium,
                art_piece.size,
                art_piece.src,
                art_piece.year.to_string(),
                art_piece.position.to_string(),
                art_piece.price.to_string(),
                art_piece.sold.to_string(),
                art_piece.art_type.as_ref().to_string(),
                art_piece.id,
            ])
            .unwrap();

        result > 0
    }
}
