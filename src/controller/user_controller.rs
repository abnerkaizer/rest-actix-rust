use actix_web::{HttpResponse, Scope, delete, get, patch, put, web};
use uuid::Uuid;

use crate::{
    auth::claims::Claims,
    auth::claims_extractor::{require_admin, require_self_or_admin},
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

/// Lista todos os usuários - apenas admin
#[get("")]
async fn find_all_users(state: web::Data<AppState>, claims: Claims) -> HttpResponse {
    // Apenas admin pode listar todos os usuários
    if let Err(response) = require_admin(&claims) {
        return response;
    }

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

/// Busca usuário por ID - próprio usuário ou admin
#[get("/{id}")]
async fn get_user_by_id(
    state: web::Data<AppState>,
    path: web::Path<Uuid>,
    claims: Claims,
) -> HttpResponse {
    let id = path.into_inner();

    // Usuário pode ver seus próprios dados ou admin pode ver qualquer um
    if let Err(response) = require_self_or_admin(&claims, &id) {
        return response;
    }

    let pool = state.pool().clone();
    let service = state.user_service().clone();

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

/// Deleta usuário - apenas admin e o próprio usuário
#[delete("/{id}")]
async fn delete_user(
    state: web::Data<AppState>,
    path: web::Path<Uuid>,
    claims: Claims,
) -> HttpResponse {
    let id = path.into_inner();

    if let Err(response) = require_self_or_admin(&claims, &id) {
        return response;
    }

    let pool = state.pool().clone();
    let service = state.user_service().clone();

    let result = web::block(move || service.delete_user(&pool, id)).await;

    match result {
        Ok(Ok(_user)) => HttpResponse::NoContent().finish(),
        Ok(Err(diesel::result::Error::NotFound)) => HttpResponse::NotFound().finish(),
        _ => HttpResponse::InternalServerError().finish(),
    }
}

/// Atualiza usuário completo - próprio usuário pode atualizar (exceto role) ou admin pode atualizar tudo
#[put("/{id}")]
async fn update_user(
    state: web::Data<AppState>,
    path: web::Path<Uuid>,
    body: web::Json<UpdateUserRequest>,
    claims: Claims,
) -> HttpResponse {
    let id = path.into_inner();

    // Verifica se é o próprio usuário ou admin
    if let Err(response) = require_self_or_admin(&claims, &id) {
        return response;
    }

    let pool = state.pool().clone();
    let service = state.user_service().clone();
    let email = body.email.clone();
    let password = body.password.clone();

    // Se não for admin, não permite alterar o role (mantém o atual)
    let role = if claims.is_admin() {
        body.role.clone()
    } else {
        // Se não é admin e tentou mudar o role, retorna erro
        if let Ok(Ok(current_user)) = web::block({
            let pool = pool.clone();
            let service = service.clone();
            move || service.find_by_id(&pool, id)
        })
        .await
        {
            if current_user.role() != body.role {
                return HttpResponse::Forbidden().json(serde_json::json!({
                    "error": "You cannot change your own role"
                }));
            }
            current_user.role().to_string()
        } else {
            return HttpResponse::InternalServerError().finish();
        }
    };

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

/// Atualiza email - próprio usuário ou admin
#[patch("/email/{id}")]
async fn patch_user_email(
    state: web::Data<AppState>,
    path: web::Path<Uuid>,
    body: web::Json<UpdateEmailRequest>,
    claims: Claims,
) -> HttpResponse {
    let id = path.into_inner();

    // Verifica se é o próprio usuário ou admin
    if let Err(response) = require_self_or_admin(&claims, &id) {
        return response;
    }

    let pool = state.pool().clone();
    let service = state.user_service().clone();
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

/// Atualiza role - apenas admin
#[patch("/role/{id}")]
async fn patch_user_role(
    state: web::Data<AppState>,
    path: web::Path<Uuid>,
    body: web::Json<UpdateRoleRequest>,
    claims: Claims,
) -> HttpResponse {
    // Apenas admin pode alterar roles
    if let Err(response) = require_admin(&claims) {
        return response;
    }

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

/// Atualiza senha - próprio usuário ou admin
#[patch("/password/{id}")]
async fn patch_user_password(
    state: web::Data<AppState>,
    path: web::Path<Uuid>,
    body: web::Json<UpdatePasswordRequest>,
    claims: Claims,
) -> HttpResponse {
    let id = path.into_inner();

    // Verifica se é o próprio usuário ou admin
    if let Err(response) = require_self_or_admin(&claims, &id) {
        return response;
    }

    let pool = state.pool().clone();
    let service = state.user_service().clone();
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
