use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Deserialize)]
pub struct PersonRequest {
    pub name: String,
    pub cpf: String,
}

#[derive(Serialize)]
pub struct PersonResponse {
    pub id: Uuid,
    pub name: String,
}