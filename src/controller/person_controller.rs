use actix_web::{get, post, web, HttpResponse, Scope};
use uuid::Uuid;

use crate::{
    AppState,
    dto::person_dto::{PersonRequest, PersonResponse},
};

pub fn routes() -> Scope {
    web::scope("/person")
        .service(create_person)
        .service(get_person_by_id)
}

#[post("")]
async fn create_person(
    state: web::Data<AppState>,
    body: web::Json<PersonRequest>,
) -> HttpResponse {
    let pool = state.pool.clone();
    let service = state.person_service.clone();

    let name = body.name.clone();
    let cpf = body.cpf.clone();

    let result = web::block(move || {
        service.create_person(&pool, name, cpf)
    })
    .await;

    match result {
        Ok(Ok(person)) => {
            let response = PersonResponse {
                id: *person.id(),
                name: person.name().to_string(),
            };
            HttpResponse::Created().json(response)
        }
        Ok(Err(_)) => HttpResponse::InternalServerError().finish(),
        Err(_) => HttpResponse::InternalServerError().finish(),
    }
}

#[get("/{id}")]
async fn get_person_by_id(
    state: web::Data<AppState>,
    path: web::Path<Uuid>,
) -> HttpResponse {
    let pool = state.pool.clone();
    let service = state.person_service.clone();
    let id = path.into_inner();

    let result = web::block(move || {
        service.find_by_id(&pool, id)
    })
    .await;

    match result {
        Ok(Ok(person)) => HttpResponse::Ok().json(PersonResponse {
            id,
            name: person.name().to_string(),
        }),
        Ok(Err(diesel::result::Error::NotFound)) => {
            HttpResponse::NotFound().finish()
        }
        _ => HttpResponse::InternalServerError().finish(),
    }
}