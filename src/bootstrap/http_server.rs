use actix_cors::Cors;
use actix_web::middleware::Logger;
use actix_web::{App, HttpServer, http::header, web};

use crate::{
    AppState, auth::middleware::AuthMiddleware, config::AppConfig, controller::auth_controller,
    controller::person_controller::routes as person_routes,
    controller::user_controller::routes as user_routes,
};

pub async fn start_http_server(
    config: AppConfig,
    app_state: web::Data<AppState>,
) -> std::io::Result<()> {
    let host = config.host();
    let port = config.port();

    HttpServer::new(move || {
        App::new()
            .wrap(
                Cors::default()
                    .allowed_origin("http://localhost:8081")
                    .allowed_methods(vec!["GET", "POST", "PUT", "PATCH", "DELETE"])
                    .allowed_headers(vec![header::AUTHORIZATION, header::ACCEPT])
                    .allowed_header(header::CONTENT_TYPE)
                    .supports_credentials()
                    .max_age(3600),
            )
            .wrap(Logger::default())
            .app_data(app_state.clone())
            .service(
                web::scope("/api")
                    .service(auth_controller::routes())
                    .service(person_routes().wrap(AuthMiddleware))
                    .service(user_routes().wrap(AuthMiddleware)),
            )
    })
    .bind((host, port))?
    .run()
    .await
}
