use crate::error::error_response::ErrorResponse;
use actix_web::{HttpResponse, Scope, delete, get, patch, post, put, web};
use cpf_util;
use uuid::Uuid;

use crate::{
    auth::claims::Claims,
    auth::claims_extractor::require_admin,
    dto::person_dto::{
        PersonRequest, PersonResponse, UpdateCpfRequest, UpdateNameRequest, UpdatePersonRequest,
    },
    util::app_state::AppState,
};

pub fn routes() -> Scope {
    web::scope("/person")
        .service(create_person)
        .service(find_all_people)
        .service(get_person_by_id)
        .service(update_person)
        .service(delete_person)
        .service(patch_person_name)
        .service(patch_person_cpf)
}

#[post("")]
async fn create_person(state: web::Data<AppState>, body: web::Json<PersonRequest>) -> HttpResponse {
    let pool = state.pool().clone();
    let service = state.person_service().clone();

    let name = body.name.clone();
    let cpf = cpf_util::format(&body.cpf.clone());

    if !cpf_util::is_valid(&cpf) {
        return HttpResponse::BadRequest().json(ErrorResponse {
            error: "Invalid CPF".to_string(),
        });
    }

    let result = web::block(move || service.create_person(&pool, name, cpf)).await;

    match result {
        Ok(Ok(person)) => {
            let response = PersonResponse {
                id: *person.id(),
                name: person.name().to_string(),
                cpf: person.cpf().to_string(),
            };
            HttpResponse::Created().json(response)
        }
        Ok(Err(_)) => HttpResponse::InternalServerError().finish(),
        Err(_) => HttpResponse::InternalServerError().finish(),
    }
}

#[get("")]
async fn find_all_people(state: web::Data<AppState>) -> HttpResponse {
    let pool = state.pool().clone();
    let service = state.person_service().clone();

    let result = web::block(move || service.find_all(&pool)).await;

    match result {
        Ok(Ok(people)) => {
            let response: Vec<PersonResponse> = people
                .into_iter()
                .map(|person| PersonResponse {
                    id: *person.id(),
                    name: person.name().to_string(),
                    cpf: person.cpf().to_string(),
                })
                .collect();

            HttpResponse::Ok().json(response)
        }
        Ok(Err(diesel::result::Error::NotFound)) => HttpResponse::NotFound().finish(),
        _ => HttpResponse::InternalServerError().finish(),
    }
}

#[get("/{id}")]
async fn get_person_by_id(state: web::Data<AppState>, path: web::Path<Uuid>) -> HttpResponse {
    let id = path.into_inner();

    let pool = state.pool().clone();
    let service = state.person_service().clone();

    let result = web::block(move || service.find_by_id(&pool, id)).await;

    match result {
        Ok(Ok(person)) => HttpResponse::Ok().json(PersonResponse {
            id,
            name: person.name().to_string(),
            cpf: person.cpf().to_string(),
        }),
        Ok(Err(diesel::result::Error::NotFound)) => HttpResponse::NotFound().finish(),
        _ => HttpResponse::InternalServerError().finish(),
    }
}

#[delete("/{id}")]
async fn delete_person(
    state: web::Data<AppState>,
    path: web::Path<Uuid>,
    claims: Claims,
) -> HttpResponse {
    if let Err(response) = require_admin(&claims) {
        return response;
    }
    let pool = state.pool().clone();
    let service = state.person_service().clone();
    let id = path.into_inner();

    let result = web::block(move || service.delete_person(&pool, id)).await;

    match result {
        Ok(Ok(_person)) => HttpResponse::NoContent().finish(),
        Ok(Err(diesel::result::Error::NotFound)) => HttpResponse::NotFound().finish(),
        _ => HttpResponse::InternalServerError().finish(),
    }
}

#[patch("/name/{id}")]
async fn patch_person_name(
    state: web::Data<AppState>,
    path: web::Path<Uuid>,
    body: web::Json<UpdateNameRequest>,
    claims: Claims,
) -> HttpResponse {
    if let Err(response) = require_admin(&claims) {
        return response;
    }

    let pool = state.pool().clone();
    let service = state.person_service().clone();
    let id = path.into_inner();
    let name = body.name.clone();

    let result = web::block(move || service.update_name(&pool, id, name)).await;

    match result {
        Ok(Ok(person)) => HttpResponse::Ok().json(PersonResponse {
            id: *person.id(),
            name: person.name().to_string(),
            cpf: person.cpf().to_string(),
        }),
        Ok(Err(diesel::result::Error::NotFound)) => HttpResponse::NotFound().finish(),
        _ => HttpResponse::InternalServerError().finish(),
    }
}

#[patch("/cpf/{id}")]
async fn patch_person_cpf(
    state: web::Data<AppState>,
    path: web::Path<Uuid>,
    body: web::Json<UpdateCpfRequest>,
    claims: Claims,
) -> HttpResponse {
    if let Err(response) = require_admin(&claims) {
        return response;
    }
    let pool = state.pool().clone();
    let service = state.person_service().clone();
    let id = path.into_inner();
    let cpf = cpf_util::format(&body.cpf.clone());

    if !cpf_util::is_valid(&cpf) {
        return HttpResponse::BadRequest().json(ErrorResponse {
            error: "Invalid CPF".to_string(),
        });
    }

    let result = web::block(move || service.update_cpf(&pool, id, cpf)).await;

    match result {
        Ok(Ok(person)) => HttpResponse::Ok().json(PersonResponse {
            id: *person.id(),
            name: person.name().to_string(),
            cpf: person.cpf().to_string(),
        }),
        Ok(Err(diesel::result::Error::NotFound)) => HttpResponse::NotFound().finish(),
        _ => HttpResponse::InternalServerError().finish(),
    }
}

#[put("/{id}")]
async fn update_person(
    state: web::Data<AppState>,
    path: web::Path<Uuid>,
    body: web::Json<UpdatePersonRequest>,
    claims: Claims,
) -> HttpResponse {
    if let Err(response) = require_admin(&claims) {
        return response;
    }

    let pool = state.pool().clone();
    let service = state.person_service().clone();
    let id = path.into_inner();
    let name = body.name.clone();
    let cpf = body.cpf.clone();

    let result = web::block(move || service.update_person(&pool, id, name, cpf)).await;

    match result {
        Ok(Ok(person)) => HttpResponse::Ok().json(PersonResponse {
            id: *person.id(),
            name: person.name().to_string(),
            cpf: person.cpf().to_string(),
        }),
        Ok(Err(diesel::result::Error::NotFound)) => HttpResponse::NotFound().finish(),
        _ => HttpResponse::InternalServerError().finish(),
    }
}
