use crate::auth::jwt::generate_token;
use actix_web::{HttpResponse, post};
use actix_web::{Scope, web};

pub fn routes() -> Scope {
    web::scope("/auth").service(login)
}

#[post("/login")]
pub async fn login() -> HttpResponse {
    let token = generate_token("user-id-123").unwrap();

    HttpResponse::Ok().json(serde_json::json!({
        "token": token
    }))
}
