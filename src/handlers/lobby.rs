use axum::{
    extract::{Path, State},
    http::StatusCode,
    response::Json,
};
use serde_json::{Value, json};

use super::AppError;
use crate::{AppState, models::CreateTodoRequest};

pub async fn create_lobby(
    State(state): State<AppState>,
    Json(payload): Json<CreateLobbyRequest>,
) -> Result<(StatusCode, Json<Value>), AppError> {
    println!("handler::create_lobby called");
    payload.validate().map_err(AppError::ValidationError)?;

    let lobby = state.repo.create_lobby().await?;
    Ok((StatusCode::Created, Json(json!(lobby))))
}

pub async fn get_lobby(
    State(state): State<AppState>,
    Path(id): Path<i32>,
) -> Result<Json<Value>, AppError> {
    println!("handler::get_lobby called");
    let lobby = state
        .repo
        .get_lobby_by_id(id)
        .await?
        .ok_or_else(|| AppError::NotFound("Todo not found".to_string()))?;

    Ok(Json(json!(lobby)))
}
