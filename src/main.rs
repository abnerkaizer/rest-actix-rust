use actix_web::web;
use dotenvy::dotenv;
use env_logger::Env;

use rest_actix_rust::{
    AppConfig, AppState, PersonService, UserService, create_pool, start_http_server,
};

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    dotenv().ok();
    env_logger::init_from_env(Env::default().default_filter_or("info"));

    let config = AppConfig::from_env();

    let pool = create_pool(&config.database_url());
    let app_state = web::Data::new(AppState::new(
        pool,
        PersonService::new(),
        UserService::new(),
        config.secret().to_string(),
    ));

    start_http_server(config, app_state).await
}
