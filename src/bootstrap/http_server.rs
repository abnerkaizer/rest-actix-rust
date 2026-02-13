use actix_web::middleware::Logger;
use actix_web::{App, HttpServer, web};

use crate::{
    AppState, auth::middleware::AuthMiddleware, config::AppConfig, controller::auth_controller,
    controller::person_controller::routes as person_routes,
    controller::user_controller::routes as user_routes,
};

pub async fn start_http_server(
    config: AppConfig,
    app_state: web::Data<AppState>,
) -> std::io::Result<()> {
    let host = config.host().to_string();
    let port = config.port();
    let cors_config = config.clone();

    HttpServer::new(move || {
        App::new()
            .wrap(cors_config.cors())
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
