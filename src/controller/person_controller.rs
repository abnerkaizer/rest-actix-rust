use crate::{
    dto::person_dto::{PaginatedResponse, PaginationQuery},
    error::error_response::ErrorResponse,
};
use actix_web::{HttpRequest, HttpResponse, Scope, delete, get, patch, post, put, web};
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

use crate::dto::hateoas::{Link, Links};

fn page_count(total: i64, size: i64) -> i64 {
    if size <= 0 {
        return 1;
    }
    ((total + size - 1) / size).max(1)
}

fn person_collection_links(req: &HttpRequest, page: i64, size: i64, total: i64) -> Links {
    let mut links = Links::new();

    let base = req
        .url_for("person_find_all", std::iter::empty::<&str>())
        .map(|u| u.to_string())
        .unwrap_or_else(|_| "/person".to_string());

    let last = page_count(total, size);

    links.insert(
        "self".into(),
        Link::get(format!("{base}?page={page}&size={size}")),
    );
    links.insert(
        "first".into(),
        Link::get(format!("{base}?page=1&size={size}")),
    );
    links.insert(
        "last".into(),
        Link::get(format!("{base}?page={last}&size={size}")),
    );

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

fn person_links(req: &HttpRequest, id: Uuid, claims: Option<&Claims>) -> Links {
    let mut links = Links::new();
    let id_s = id.to_string();

    let self_href = req
        .url_for("person_get_by_id", [id_s.as_str()])
        .map(|u| u.to_string())
        .unwrap_or_else(|_| format!("/person/{id}"));
    links.insert("self".into(), Link::get(self_href));

    let collection_href = req
        .url_for("person_find_all", std::iter::empty::<&str>())
        .map(|u| u.to_string())
        .unwrap_or_else(|_| "/person".to_string());
    links.insert("collection".into(), Link::get(collection_href));

    let create_href = req
        .url_for("person_create", std::iter::empty::<&str>())
        .map(|u| u.to_string())
        .unwrap_or_else(|_| "/person".to_string());
    links.insert("create".into(), Link::post(create_href));

    if claims.map_or(false, |c| c.is_admin()) {
        let del_href = req
            .url_for("person_delete", [id_s.as_str()])
            .map(|u| u.to_string())
            .unwrap_or_else(|_| format!("/person/{id}"));
        links.insert("delete".into(), Link::delete(del_href));

        let put_href = req
            .url_for("person_update", [id_s.as_str()])
            .map(|u| u.to_string())
            .unwrap_or_else(|_| format!("/person/{id}"));
        links.insert("update".into(), Link::put(put_href));

        let patch_name_href = req
            .url_for("person_patch_name", [id_s.as_str()])
            .map(|u| u.to_string())
            .unwrap_or_else(|_| format!("/person/name/{id}"));
        links.insert("update_name".into(), Link::patch(patch_name_href));

        let patch_cpf_href = req
            .url_for("person_patch_cpf", [id_s.as_str()])
            .map(|u| u.to_string())
            .unwrap_or_else(|_| format!("/person/cpf/{id}"));
        links.insert("update_cpf".into(), Link::patch(patch_cpf_href));
    }

    links
}

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

#[post("", name = "person_create")]
async fn create_person(
    req: HttpRequest,
    claims: Option<Claims>,
    state: web::Data<AppState>,
    body: web::Json<PersonRequest>,
) -> HttpResponse {
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
                links: person_links(&req, *person.id(), claims.as_ref()),
            };
            HttpResponse::Created().json(response)
        }
        Ok(Err(_)) => HttpResponse::InternalServerError().finish(),
        Err(_) => HttpResponse::InternalServerError().finish(),
    }
}

#[get("", name = "person_find_all")]
async fn find_all_people(
    req: HttpRequest,
    state: web::Data<AppState>,
    query: web::Query<PaginationQuery>,
    claims: Option<Claims>,
) -> HttpResponse {
    let pool = state.pool().clone();
    let service = state.person_service().clone();

    let page = query.page.max(1);
    let size = query.size.max(1).min(100);

    let result = web::block(move || service.find_page(&pool, page, size)).await;

    match result {
        Ok(Ok((total, people))) => {
            let links = person_collection_links(&req, page, size, total);

            let items: Vec<PersonResponse> = people
                .into_iter()
                .map(|person| {
                    let id = *person.id();
                    PersonResponse {
                        id,
                        name: person.name().to_string(),
                        cpf: person.cpf().to_string(),
                        links: person_links(&req, id, claims.as_ref()),
                    }
                })
                .collect();

            HttpResponse::Ok().json(PaginatedResponse {
                page,
                size,
                total,
                items,
                links,
            })
        }
        _ => HttpResponse::InternalServerError().finish(),
    }
}

#[get("/{id}", name = "person_get_by_id")]
async fn get_person_by_id(
    req: HttpRequest,
    state: web::Data<AppState>,
    path: web::Path<Uuid>,
    claims: Option<Claims>,
) -> HttpResponse {
    let id = path.into_inner();

    let pool = state.pool().clone();
    let service = state.person_service().clone();

    let result = web::block(move || service.find_by_id(&pool, id)).await;

    match result {
        Ok(Ok(person)) => HttpResponse::Ok().json(PersonResponse {
            id,
            name: person.name().to_string(),
            cpf: person.cpf().to_string(),
            links: person_links(&req, id, claims.as_ref()),
        }),
        Ok(Err(diesel::result::Error::NotFound)) => HttpResponse::NotFound().finish(),
        _ => HttpResponse::InternalServerError().finish(),
    }
}

#[delete("/{id}", name = "person_delete")]
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

#[patch("/name/{id}", name = "person_patch_name")]
async fn patch_person_name(
    req: HttpRequest,
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
            links: person_links(&req, *person.id(), Some(&claims)),
        }),
        Ok(Err(diesel::result::Error::NotFound)) => HttpResponse::NotFound().finish(),
        _ => HttpResponse::InternalServerError().finish(),
    }
}

#[patch("/cpf/{id}", name = "person_patch_cpf")]
async fn patch_person_cpf(
    req: HttpRequest,
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
            links: person_links(&req, id, Some(&claims)),
        }),
        Ok(Err(diesel::result::Error::NotFound)) => HttpResponse::NotFound().finish(),
        _ => HttpResponse::InternalServerError().finish(),
    }
}

#[put("/{id}", name = "person_update")]
async fn update_person(
    req: HttpRequest,
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
    let cpf = cpf_util::format(&body.cpf);

    if !cpf_util::is_valid(&cpf) {
        return HttpResponse::BadRequest().json(ErrorResponse {
            error: "Invalid CPF".into(),
        });
    }

    let result = web::block(move || service.update_person(&pool, id, name, cpf)).await;

    match result {
        Ok(Ok(person)) => HttpResponse::Ok().json(PersonResponse {
            id: *person.id(),
            name: person.name().to_string(),
            cpf: person.cpf().to_string(),
            links: person_links(&req, id, Some(&claims)),
        }),
        Ok(Err(diesel::result::Error::NotFound)) => HttpResponse::NotFound().finish(),
        _ => HttpResponse::InternalServerError().finish(),
    }
}
