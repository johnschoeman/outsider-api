use axum::{
    extract::{Path, State},
    http::StatusCode,
    response::Json,
};
use serde_json::{Value, json};

use crate::{AppState, models::CreateLobbyRequest};
use super::AppError;

pub async fn create_lobby(
    State(_state): State<AppState>,
    Json(payload): Json<CreateLobbyRequest>,
) -> Result<(StatusCode, Json<Value>), AppError> {
    println!("handler::create_lobby called");
    payload.validate().map_err(AppError::ValidateError)?;

    // Temporary mock implementation
    let lobby = serde_json::json!({
        "id": 1,
        "name": "Mock Lobby"
    });
    Ok((StatusCode::CREATED, Json(lobby)))
}

pub async fn get_lobby(
    State(_state): State<AppState>,
    Path(id): Path<i32>,
) -> Result<Json<Value>, AppError> {
    println!("handler::get_lobby called");
    // Temporary mock implementation
    let lobby = serde_json::json!({
        "id": id,
        "name": format!("Lobby {}", id)
    });

    Ok(Json(json!(lobby)))
}
