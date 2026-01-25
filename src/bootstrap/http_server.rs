use actix_web::middleware::Logger;
use actix_web::{App, HttpServer, web};

use crate::{AppState, config::AppConfig, person_routes};

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
            .service(web::scope("/api").service(person_routes()))
    })
    .bind((host, port))?
    .run()
    .await
}
