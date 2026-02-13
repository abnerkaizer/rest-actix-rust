use crate::{
    dto::user_dto::{PageQuery, PaginatedResponse},
    model::role::Role,
};
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

use crate::dto::hateoas::{Link, Links};
use actix_web::HttpRequest;

fn page_count(total: i64, size: i64) -> i64 {
    if size <= 0 {
        return 1;
    }
    let pages = (total + size - 1) / size;
    pages.max(1)
}

fn collection_page_links(
    req: &HttpRequest,
    route_name: &str,
    page: i64,
    size: i64,
    total: i64,
) -> Links {
    let mut links = Links::new();

    let base = req
        .url_for(route_name, std::iter::empty::<&str>())
        .map(|u| u.to_string())
        .unwrap_or_else(|_| "".to_string());

    let last = page_count(total, size);

    let self_href = format!("{base}?page={page}&size={size}");
    let first_href = format!("{base}?page=1&size={size}");
    let last_href = format!("{base}?page={last}&size={size}");

    links.insert("self".into(), Link::get(self_href));
    links.insert("first".into(), Link::get(first_href));
    links.insert("last".into(), Link::get(last_href));

    if page > 1 {
        links.insert(
            "prev".into(),
            Link::get(format!("{base}?page={}&size={}", page - 1, size)),
        );
    }
    if page < last {
        links.insert(
            "next".into(),
            Link::get(format!("{base}?page={}&size={}", page + 1, size)),
        );
    }

    links
}

fn user_links(req: &HttpRequest, id: Uuid, claims: &Claims) -> Links {
    let mut links = Links::new();
    let id_s = id.to_string();

    let self_href = req
        .url_for("user_get_by_id", [id_s.as_str()])
        .map(|u| u.to_string())
        .unwrap_or_else(|_| format!("/user/{id}"));

    links.insert("self".into(), Link::get(self_href));

    if claims.is_admin() {
        let collection_href = req
            .url_for("user_find_all", std::iter::empty::<&str>())
            .map(|u| u.to_string())
            .unwrap_or_else(|_| "/user".to_string());
        links.insert("collection".into(), Link::get(collection_href));
    }

    let del_href = req
        .url_for("user_delete", [id_s.as_str()])
        .map(|u| u.to_string())
        .unwrap_or_else(|_| format!("/user/{id}"));
    links.insert("delete".into(), Link::delete(del_href));

    let put_href = req
        .url_for("user_update", [id_s.as_str()])
        .map(|u| u.to_string())
        .unwrap_or_else(|_| format!("/user/{id}"));
    links.insert("update".into(), Link::put(put_href));

    let patch_email_href = req
        .url_for("user_patch_email", [id_s.as_str()])
        .map(|u| u.to_string())
        .unwrap_or_else(|_| format!("/user/email/{id}"));
    links.insert("update_email".into(), Link::patch(patch_email_href));

    let patch_pass_href = req
        .url_for("user_patch_password", [id_s.as_str()])
        .map(|u| u.to_string())
        .unwrap_or_else(|_| format!("/user/password/{id}"));
    links.insert("update_password".into(), Link::patch(patch_pass_href));

    if claims.is_admin() {
        let patch_role_href = req
            .url_for("user_patch_role", [id_s.as_str()])
            .map(|u| u.to_string())
            .unwrap_or_else(|_| format!("/user/role/{id}"));
        links.insert("update_role".into(), Link::patch(patch_role_href));
    }

    links
}

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
#[get("", name = "user_find_all")]
async fn find_all_users(
    req: HttpRequest,
    state: web::Data<AppState>,
    claims: Claims,
    query: web::Query<PageQuery>,
) -> HttpResponse {
    if let Err(response) = require_admin(&claims) {
        return response;
    }

    let pool = state.pool().clone();
    let service = state.user_service().clone();

    let page = query.page.max(1);
    let size = query.size.max(1).min(100);

    let result = web::block(move || service.find_page(&pool, page, size)).await;

    match result {
        Ok(Ok((total, users))) => {
            let items: Vec<UserResponse> = users
                .into_iter()
                .map(|user| {
                    let id = *user.id();
                    UserResponse {
                        id,
                        email: user.email().to_string(),
                        role: user.role(),
                        links: user_links(&req, id, &claims),
                    }
                })
                .collect();

            HttpResponse::Ok().json(PaginatedResponse {
                page,
                size,
                total,
                items,
                links: collection_page_links(&req, "user_find_all", page, size, total),
            })
        }
        _ => HttpResponse::InternalServerError().finish(),
    }
}

/// Busca usuário por ID - próprio usuário ou admin
#[get("/{id}", name = "user_get_by_id")]
async fn get_user_by_id(
    req: HttpRequest,
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

    let result = web::block(move || service.find_by_id(&pool, id)).await;

    match result {
        Ok(Ok(user)) => HttpResponse::Ok().json(UserResponse {
            id,
            email: user.email().to_string(),
            role: user.role(),
            links: user_links(&req, id, &claims),
        }),
        Ok(Err(diesel::result::Error::NotFound)) => HttpResponse::NotFound().finish(),
        _ => HttpResponse::InternalServerError().finish(),
    }
}

/// Deleta usuário - apenas admin e o próprio usuário
#[delete("/{id}", name = "user_delete")]
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
#[put("/{id}", name = "user_update")]
async fn update_user(
    req: HttpRequest,
    state: web::Data<AppState>,
    path: web::Path<Uuid>,
    body: web::Json<UpdateUserRequest>,
    claims: Claims,
) -> HttpResponse {
    let id = path.into_inner();

    if let Err(response) = require_self_or_admin(&claims, &id) {
        return response;
    }

    let pool = state.pool().clone();
    let service = state.user_service().clone();
    let email = body.email.clone();
    let password = body.password.clone();

    let desired_role: Role = match body.role.as_str().parse() {
        Ok(r) => r,
        Err(_) => {
            return HttpResponse::BadRequest().json(serde_json::json!({
                "error": "Invalid role (use 'admin' or 'user')"
            }));
        }
    };

    let role: Role = if claims.is_admin() {
        desired_role
    } else {
        let current = match web::block({
            let pool = pool.clone();
            let service = service.clone();
            move || service.find_by_id(&pool, id)
        })
        .await
        {
            Ok(Ok(u)) => u,
            Ok(Err(diesel::result::Error::NotFound)) => return HttpResponse::NotFound().finish(),
            _ => return HttpResponse::InternalServerError().finish(),
        };

        if current.role() != desired_role {
            return HttpResponse::Forbidden().json(serde_json::json!({
                "error": "You cannot change your own role"
            }));
        }

        current.role()
    };

    let result = web::block(move || service.update_user(&pool, id, email, role, password)).await;

    match result {
        Ok(Ok(user)) => HttpResponse::Ok().json(UserResponse {
            id: *user.id(),
            email: user.email().to_string(),
            role: user.role(),
            links: user_links(&req, id, &claims),
        }),
        Ok(Err(diesel::result::Error::NotFound)) => HttpResponse::NotFound().finish(),
        _ => HttpResponse::InternalServerError().finish(),
    }
}

/// Atualiza email - próprio usuário ou admin
#[patch("/email/{id}", name = "user_patch_email")]
async fn patch_user_email(
    req: HttpRequest,
    state: web::Data<AppState>,
    path: web::Path<Uuid>,
    body: web::Json<UpdateEmailRequest>,
    claims: Claims,
) -> HttpResponse {
    let id = path.into_inner();

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
            role: user.role(),
            links: user_links(&req, id, &claims),
        }),
        Ok(Err(diesel::result::Error::NotFound)) => HttpResponse::NotFound().finish(),
        _ => HttpResponse::InternalServerError().finish(),
    }
}

/// Atualiza role - apenas admin
#[patch("/password/{id}", name = "user_patch_password")]
async fn patch_user_role(
    req: HttpRequest,
    state: web::Data<AppState>,
    path: web::Path<Uuid>,
    body: web::Json<UpdateRoleRequest>,
    claims: Claims,
) -> HttpResponse {
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
            role: user.role(),
            links: user_links(&req, id, &claims),
        }),
        Ok(Err(diesel::result::Error::NotFound)) => HttpResponse::NotFound().finish(),
        _ => HttpResponse::InternalServerError().finish(),
    }
}

/// Atualiza senha - próprio usuário ou admin
#[patch("/role/{id}", name = "user_patch_role")]
async fn patch_user_password(
    req: HttpRequest,
    state: web::Data<AppState>,
    path: web::Path<Uuid>,
    body: web::Json<UpdatePasswordRequest>,
    claims: Claims,
) -> HttpResponse {
    let id = path.into_inner();

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
            role: user.role(),
            links: user_links(&req, id, &claims),
        }),
        Ok(Err(UserServiceError::NotFound)) => HttpResponse::NotFound().finish(),
        Ok(Err(UserServiceError::HashError)) => HttpResponse::InternalServerError().finish(),
        Ok(Err(_)) => HttpResponse::InternalServerError().finish(),
        Err(_) => HttpResponse::InternalServerError().finish(),
    }
}
