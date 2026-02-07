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
