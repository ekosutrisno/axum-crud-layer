use crate::model::database::{Database, new_db_pool};

mod database;

pub mod note;

#[derive(Clone)]
pub struct ModelManager {
    db: Database,
}

impl ModelManager {
    pub async fn new() -> Result<Self, sqlx::Error> {
        let db = new_db_pool().await?;
        Ok(ModelManager { db })
    }

    pub(in crate::model) fn db(&self) -> &Database {
        &self.db
    }
}