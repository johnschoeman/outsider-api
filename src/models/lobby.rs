use chrono::{DataTime, Utc};
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Lobby {
    pub id: i32,
    pub name: String,
}

#[derive(Debug, Deserialize)]
pub struct CreateLobbyRequest {
}

impl CreateLobbyRequest {
    pub fn validate(&self) -> Result<(), String> {
        Ok(())
    }
}
