use serde::{Deserialize, Serialize};
use uuid::Uuid;

use crate::model::role::Role;

#[derive(Deserialize)]
pub struct UserRequest {
    pub email: String,
    pub password: String,
}

#[derive(Serialize)]
pub struct UserResponse {
    pub id: Uuid,
    pub email: String,
    pub role: Role,
}

#[derive(Deserialize)]
pub struct UpdateUserRequest {
    pub email: String,
    pub password: String,
    pub role: Role,
}

#[derive(Deserialize)]
pub struct UpdateEmailRequest {
    pub email: String,
}

#[derive(Deserialize)]
pub struct UpdateRoleRequest {
    pub role: Role,
}

#[derive(Deserialize)]
pub struct UpdatePasswordRequest {
    pub password: String,
}

fn default_page() -> i64 {
    1
}
fn default_size() -> i64 {
    20
}

#[derive(Deserialize)]
pub struct PageQuery {
    #[serde(default = "default_page")]
    pub page: i64,
    #[serde(default = "default_size")]
    pub size: i64,
}

#[derive(Serialize)]
pub struct PaginatedResponse<T> {
    pub page: i64,
    pub size: i64,
    pub total: i64,
    pub items: Vec<T>,
}
