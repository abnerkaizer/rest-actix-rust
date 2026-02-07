use crate::{
    dto::auth_dto::{LoginRequest, RegisterRequest},
    model::role::Role,
    service::auth_service::AuthService,
    util::app_state::AppState,
};
use actix_web::{HttpResponse, Scope, post, web};

pub fn routes() -> Scope {
    web::scope("/auth").service(login).service(register)
}

#[post("/login")]
pub async fn login(state: web::Data<AppState>, body: web::Json<LoginRequest>) -> HttpResponse {
    let pool = state.pool().clone();

    let email = body.email.clone();
    let password = body.password.clone();

    let result = web::block(move || {
        let mut conn = pool.get().map_err(|_| "Database connection error")?;
        let secret = state.secret();

        AuthService::login(&mut conn, email, password, &secret)
    })
    .await;

    match result {
        Ok(Ok(token)) => HttpResponse::Ok().json(serde_json::json!({ "token": token })),
        Ok(Err(msg)) => HttpResponse::Unauthorized().json(serde_json::json!({ "error": msg })),
        Err(_) => HttpResponse::InternalServerError().finish(),
    }
}

#[post("/register")]
pub async fn register(
    state: web::Data<AppState>,
    body: web::Json<RegisterRequest>,
) -> HttpResponse {
    let pool = state.pool().clone();
    let email = body.email.clone();
    let role = Role::User;

    let password = body.password.clone();

    let result = web::block(move || {
        let mut conn = pool.get().map_err(|_| "Database connection error")?;

        AuthService::register(&mut conn, email, role, password)
    })
    .await;

    match result {
        Ok(Ok(id)) => HttpResponse::Created().json(serde_json::json!({ "id": id })),
        Ok(Err(msg)) => HttpResponse::BadRequest().json(serde_json::json!({ "error": msg })),
        Err(_) => HttpResponse::InternalServerError().finish(),
    }
}
