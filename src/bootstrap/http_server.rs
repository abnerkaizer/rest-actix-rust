use actix_web::{web, App, HttpServer};
use actix_web::middleware::Logger;

use crate::{
    AppState,
    person_routes,
    config::AppConfig,
};

pub async fn start_http_server(
    config: AppConfig,
    app_state: web::Data<AppState>,
) -> std::io::Result<()> {
    let host = config.host();
    let port = config.port();

    HttpServer::new(move || {
        App::new()
            .wrap(Logger::default())
            .app_data(app_state.clone())
            .service(
                web::scope("/api")
                    .service(person_routes()),
            )
    })
    .bind((host, port))?
    .run()
    .await
}
