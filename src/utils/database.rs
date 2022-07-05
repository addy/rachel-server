use rusqlite::{Connection, Result};

pub struct Database {
    pub conn: Connection,
}

impl Database {
    pub fn new() -> Result<Self> {
        let database_url = std::env::var("DATABASE_URL").expect("DATABASE_URL must be set");
        let db_path = std::path::Path::new(&database_url);
        let conn = Connection::open(db_path)?;

        // Migrations
        conn.execute(
            "CREATE TABLE IF NOT EXISTS art (
                id TEXT PRIMARY KEY,
                title TEXT,
                medium TEXT,
                size TEXT,
                src TEXT,
                year INTEGER,
                position INTEGER,
                price INTEGER,
                sold INTEGER,
                art_type TEXT
            )",
            [],
        )?;

        Ok(Self { conn })
    }
}
