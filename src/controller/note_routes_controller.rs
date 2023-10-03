use axum::{
    extract::{Path, Query, State},
    http::StatusCode,
    response::IntoResponse,
    routing::{post, put},
    Json, Router,
};
use serde_json::{json, Value};
use uuid::Uuid;

use crate::{
    model::note::NoteModelController,
    model::ModelManager,
    shcema::{CreateNoteSchema, FilterOptions, UpdateNoteSchema},
};

pub fn note_routes(mm: ModelManager) -> Router {
    Router::new()
        .route("/api/notes", post(create_handler).get(find_all_handler))
        .route(
            "/api/notes/:id",
            put(update_handler)
                .get(find_by_id_handler)
                .delete(delete_handler),
        )
        .with_state(mm)
}

pub async fn find_all_handler(
    State(mm): State<ModelManager>,
    opts: Option<Query<FilterOptions>>,
) -> Result<impl IntoResponse, (StatusCode, Json<Value>)> {
    let Query(opts) = opts.unwrap_or_default();

    match NoteModelController::find_all(&mm, opts).await {
        Ok(notes) => {
            let json_response = json!({
                "status": "success",
                "results": notes.len(),
                "notes": notes
            });
            Ok(Json(json_response))
        }
        Err(_) => {
            let error_response = json!({
                "status": "fail",
                "message": "Something bad happened while fetching all note items",
            });
            Err((StatusCode::INTERNAL_SERVER_ERROR, Json(error_response)))
        }
    }
}

pub async fn find_by_id_handler(
    State(mm): State<ModelManager>,
    Path(id): Path<Uuid>,
) -> Result<impl IntoResponse, (StatusCode, Json<Value>)> {
    match NoteModelController::find_by_id(&mm, id).await {
        Ok(note) => {
            let note_response = json!({"status": "success","data": json!({
                "note": note
            })});
            Ok(Json(note_response))
        }
        Err(_) => {
            let error_response = json!({
                "status": "fail",
                "message": format!("Note with ID: {} not found", id)
            });
            Err((StatusCode::NOT_FOUND, Json(error_response)))
        }
    }
}

pub async fn create_handler(
    State(mm): State<ModelManager>,
    Json(data): Json<CreateNoteSchema>,
) -> Result<impl IntoResponse, (StatusCode, Json<Value>)> {
    match NoteModelController::create(&mm, data).await {
        Ok(note) => {
            let note_response = json!({"status": "success","data": json!({
                "note": note
            })});

            Ok((StatusCode::CREATED, Json(note_response)))
        }
        Err(e) => {
            if e.to_string()
                .contains("duplicate key value violates unique constraint")
            {
                let error_response = json!({
                    "status": "fail",
                    "message": "Note with that title already exists",
                });
                return Err((StatusCode::CONFLICT, Json(error_response)));
            }
            Err((
                StatusCode::INTERNAL_SERVER_ERROR,
                Json(json!({"status": "error","message": format!("{:?}", e)})),
            ))
        }
    }
}

pub async fn update_handler(
    State(mm): State<ModelManager>,
    Path(id): Path<uuid::Uuid>,
    Json(data): Json<UpdateNoteSchema>,
) -> Result<impl IntoResponse, (StatusCode, Json<Value>)> {
    match NoteModelController::update(&mm, id, data).await {
        Ok(note) => {
            let note_response = json!({"status": "success","data": json!({
                "note": note
            })});

            Ok(Json(note_response))
        }
        Err(_) => Err((
            StatusCode::NOT_FOUND,
            Json(json!({"status": "error","message": format!("Note with ID: {} not found", id)})),
        )),
    }
}

pub async fn delete_handler(
    State(mm): State<ModelManager>,
    Path(id): Path<Uuid>,
) -> Result<impl IntoResponse, (StatusCode, Json<Value>)> {
    match NoteModelController::delete(&mm, id).await {
        Ok(rows_affected) => {
            if rows_affected == 0 {
                let error_response = json!({
                    "status": "fail",
                    "message": format!("Note with ID: {} not found", id),
                });
                return Err((StatusCode::NOT_FOUND, Json(error_response)));
            }

            Ok(StatusCode::NO_CONTENT)
        }
        Err(error) => {
            let error_response = json!({
                "status": "error",
                "message": format!("Failed to delete note with ID: {}", id),
                "details": error.to_string(),
            });
            Err((StatusCode::INTERNAL_SERVER_ERROR, Json(error_response)))
        }
    }
}
