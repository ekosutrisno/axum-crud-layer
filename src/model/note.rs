use serde::{Deserialize, Serialize};
use sqlx::FromRow;
use uuid::Uuid;

use crate::{
    model::ModelManager,
    shcema::{CreateNoteSchema, FilterOptions, UpdateNoteSchema},
};

#[derive(Debug, FromRow, Deserialize, Serialize)]
pub struct NoteModel {
    pub id: Uuid,
    pub title: String,
    pub content: String,
    pub category: Option<String>,
    pub published: Option<bool>,
    pub created_at: Option<chrono::DateTime<chrono::Utc>>,
    pub updated_at: Option<chrono::DateTime<chrono::Utc>>,
}

pub struct NoteModelController;

impl NoteModelController {
    pub async fn find_all(
        mm: &ModelManager,
        opts: FilterOptions,
    ) -> Result<Vec<NoteModel>, sqlx::Error> {
        let limit = opts.limit.unwrap_or(10);
        let offset = (opts.page.unwrap_or(1) - 1) * limit;

        sqlx::query_as::<_, NoteModel>(r#"SELECT * FROM notes ORDER BY id LIMIT $1 OFFSET $2"#)
            .bind(limit as i32)
            .bind(offset as i32)
            .fetch_all(mm.db())
            .await
    }

    pub async fn find_by_id(mm: &ModelManager, id: Uuid) -> Result<NoteModel, sqlx::Error> {
        sqlx::query_as::<_, NoteModel>(r#"SELECT * FROM notes WHERE id = $1"#)
            .bind(id)
            .fetch_one(mm.db())
            .await
    }

    pub async fn create(
        mm: &ModelManager,
        data: CreateNoteSchema,
    ) -> Result<NoteModel, sqlx::Error> {
        sqlx::query_as::<_, NoteModel>(
            r#"INSERT INTO notes (title,content,category) VALUES ($1, $2, $3) RETURNING *"#,
        )
        .bind(data.title.to_string())
        .bind(data.content.to_string())
        .bind(data.category.to_owned().unwrap_or("".to_string()))
        .fetch_one(mm.db())
        .await
    }

    pub async fn update(
        mm: &ModelManager,
        id: Uuid,
        data: UpdateNoteSchema,
    ) -> Result<NoteModel, sqlx::Error> {
        // Fetch the existing note
        let query_result = match sqlx::query_as!(NoteModel, "SELECT * FROM notes WHERE id = $1", id)
            .fetch_one(mm.db())
            .await
        {
            Ok(note) => note,
            Err(error) => return Err(error),
        };

        let title = data
            .title
            .clone()
            .unwrap_or_else(|| query_result.title.clone());
        let content = data
            .content
            .clone()
            .unwrap_or_else(|| query_result.content.clone());
        let category = data
            .category
            .clone()
            .unwrap_or_else(|| query_result.category.unwrap());
        let published = data
            .published
            .clone()
            .unwrap_or_else(|| query_result.published.unwrap());

        let now = chrono::Utc::now();

        // Perform the update
        match sqlx::query_as::<_,NoteModel>(
            r#"UPDATE notes SET title = $1, content = $2, category = $3, published = $4, updated_at = $5 WHERE id = $6 RETURNING *"#
        )
        .bind(&title)
        .bind(&content)
        .bind(&category)
        .bind(&published)
        .bind(&now)
        .bind(&id)
        .fetch_one(mm.db())
        .await
        {
            Ok(updated_note) => Ok(updated_note),
            Err(error) => Err(error),
        }
    }

    pub async fn delete(mm: &ModelManager, id: Uuid) -> Result<u64, sqlx::Error> {
        let delete_result = sqlx::query!("DELETE FROM notes WHERE id = $1", id)
            .execute(mm.db())
            .await;

        match delete_result {
            Ok(result) => Ok(result.rows_affected()),
            Err(error) => Err(error),
        }
    }
}