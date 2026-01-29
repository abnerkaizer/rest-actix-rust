use actix_web::{HttpResponse, Scope, delete, get, patch, put, web};
use uuid::Uuid;

use crate::{
    dto::user_dto::{
        UpdateEmailRequest, UpdatePasswordRequest, UpdateRoleRequest, UpdateUserRequest,
        UserResponse,
    },
    error::user_service_error::UserServiceError,
    util::app_state::AppState,
};

pub fn routes() -> Scope {
    web::scope("/user")
        .service(find_all_users)
        .service(get_user_by_id)
        .service(delete_user)
        .service(update_user)
        .service(patch_user_email)
        .service(patch_user_password)
        .service(patch_user_role)
}

#[get("")]
async fn find_all_users(state: web::Data<AppState>) -> HttpResponse {
    let pool = state.pool().clone();
    let service = state.user_service().clone();

    let result = web::block(move || service.find_all(&pool)).await;

    match result {
        Ok(Ok(users)) => {
            let response: Vec<UserResponse> = users
                .into_iter()
                .map(|user| UserResponse {
                    id: *user.id(),
                    email: user.email().to_string(),
                    role: user.role().to_string(),
                })
                .collect();

            HttpResponse::Ok().json(response)
        }
        Ok(Err(diesel::result::Error::NotFound)) => HttpResponse::NotFound().finish(),
        _ => HttpResponse::InternalServerError().finish(),
    }
}

#[get("/{id}")]
async fn get_user_by_id(state: web::Data<AppState>, path: web::Path<Uuid>) -> HttpResponse {
    let pool = state.pool().clone();
    let service = state.user_service().clone();
    let id = path.into_inner();

    let result = web::block(move || service.find_by_id(&pool, id)).await;

    match result {
        Ok(Ok(user)) => HttpResponse::Ok().json(UserResponse {
            id,
            email: user.email().to_string(),
            role: user.role().to_string(),
        }),
        Ok(Err(diesel::result::Error::NotFound)) => HttpResponse::NotFound().finish(),
        _ => HttpResponse::InternalServerError().finish(),
    }
}

#[delete("/{id}")]
async fn delete_user(state: web::Data<AppState>, path: web::Path<Uuid>) -> HttpResponse {
    let pool = state.pool().clone();
    let service = state.user_service().clone();
    let id = path.into_inner();

    let result = web::block(move || service.delete_user(&pool, id)).await;

    match result {
        Ok(Ok(_user)) => HttpResponse::NoContent().finish(),
        Ok(Err(diesel::result::Error::NotFound)) => HttpResponse::NotFound().finish(),
        _ => HttpResponse::InternalServerError().finish(),
    }
}

#[put("/{id}")]
async fn update_user(
    state: web::Data<AppState>,
    path: web::Path<Uuid>,
    body: web::Json<UpdateUserRequest>,
) -> HttpResponse {
    let pool = state.pool().clone();
    let service = state.user_service().clone();
    let id = path.into_inner();
    let email = body.email.clone();
    let role = body.role.clone();
    let password = body.password.clone();

    let result = web::block(move || service.update_user(&pool, id, email, role, password)).await;

    match result {
        Ok(Ok(user)) => HttpResponse::Ok().json(UserResponse {
            id: *user.id(),
            email: user.email().to_string(),
            role: user.role().to_string(),
        }),
        Ok(Err(diesel::result::Error::NotFound)) => HttpResponse::NotFound().finish(),
        _ => HttpResponse::InternalServerError().finish(),
    }
}

#[patch("/email/{id}")]
async fn patch_user_email(
    state: web::Data<AppState>,
    path: web::Path<Uuid>,
    body: web::Json<UpdateEmailRequest>,
) -> HttpResponse {
    let pool = state.pool().clone();
    let service = state.user_service().clone();
    let id = path.into_inner();
    let email = body.email.clone();

    let result = web::block(move || service.update_email(&pool, id, email)).await;

    match result {
        Ok(Ok(user)) => HttpResponse::Ok().json(UserResponse {
            id: *user.id(),
            email: user.email().to_string(),
            role: user.role().to_string(),
        }),
        Ok(Err(diesel::result::Error::NotFound)) => HttpResponse::NotFound().finish(),
        _ => HttpResponse::InternalServerError().finish(),
    }
}

#[patch("/role/{id}")]
async fn patch_user_role(
    state: web::Data<AppState>,
    path: web::Path<Uuid>,
    body: web::Json<UpdateRoleRequest>,
) -> HttpResponse {
    let pool = state.pool().clone();
    let service = state.user_service().clone();
    let id = path.into_inner();
    let role = body.role.clone();

    let result = web::block(move || service.update_role(&pool, id, role)).await;

    match result {
        Ok(Ok(user)) => HttpResponse::Ok().json(UserResponse {
            id: *user.id(),
            email: user.email().to_string(),
            role: user.role().to_string(),
        }),
        Ok(Err(diesel::result::Error::NotFound)) => HttpResponse::NotFound().finish(),
        _ => HttpResponse::InternalServerError().finish(),
    }
}

#[patch("/password/{id}")]
async fn patch_user_password(
    state: web::Data<AppState>,
    path: web::Path<Uuid>,
    body: web::Json<UpdatePasswordRequest>,
) -> HttpResponse {
    let pool = state.pool().clone();
    let service = state.user_service().clone();
    let id = path.into_inner();
    let password = body.password.clone();

    let result = web::block(move || service.update_password(&pool, id, password)).await;

    match result {
        Ok(Ok(user)) => HttpResponse::Ok().json(UserResponse {
            id: *user.id(),
            email: user.email().to_string(),
            role: user.role().to_string(),
        }),
        Ok(Err(UserServiceError::NotFound)) => HttpResponse::NotFound().finish(),
        Ok(Err(UserServiceError::HashError)) => HttpResponse::InternalServerError().finish(),
        Ok(Err(_)) => HttpResponse::InternalServerError().finish(),
        Err(_) => HttpResponse::InternalServerError().finish(),
    }
}
