use actix_web::{web, App, HttpServer};
use actix_web::middleware::Logger;
use env_logger::Env;
use dotenvy::dotenv;

use std::env;

mod controller;
mod dto;
mod model;
mod repository;
mod service;
mod schema;

use controller::person_controller;
use service::db::{DbPool, create_pool};
use service::person_service::PersonService;

pub struct AppState {
    pub pool: DbPool,
    pub person_service: PersonService,
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    dotenv().ok();

    env_logger::init_from_env(Env::default().default_filter_or("info"));

    let database_url =
        env::var("DATABASE_URL").expect("DATABASE_URL must be set");

    let pool = create_pool(&database_url);

    let app_state = web::Data::new(AppState {
        pool,
        person_service: PersonService::new(),
    });

    HttpServer::new(move || {
        App::new()
            .wrap(Logger::default())
            .app_data(app_state.clone())
            .service(
                web::scope("/api")
                    .service(person_controller::routes()),
            )
    })
    .bind(("127.0.0.1", 8080))?
    .run()
    .await
}