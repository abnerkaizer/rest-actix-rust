use serde::{Deserialize, Serialize};
use uuid::Uuid;

use crate::dto::hateoas::Links;

#[derive(Deserialize)]
pub struct PersonRequest {
    pub name: String,
    pub cpf: String,
}

#[derive(Serialize)]
pub struct PersonResponse {
    pub id: Uuid,
    pub name: String,
    pub cpf: String,

    #[serde(rename = "_links")]
    pub links: Links,
}

#[derive(Deserialize)]
pub struct UpdatePersonRequest {
    pub name: String,
    pub cpf: String,
}

#[derive(Deserialize)]
pub struct UpdateNameRequest {
    pub name: String,
}

#[derive(Deserialize)]
pub struct UpdateCpfRequest {
    pub cpf: String,
}

fn default_page() -> i64 {
    1
}
fn default_size() -> i64 {
    20
}

#[derive(Deserialize)]
pub struct PaginationQuery {
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

    #[serde(rename = "_links")]
    pub links: Links,
}
