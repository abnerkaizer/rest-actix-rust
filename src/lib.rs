pub mod config;
pub mod bootstrap;

pub mod controller;
pub mod dto;
pub mod model;
pub mod repository;
pub mod schema;
pub mod service;
pub mod util;


pub use config::AppConfig;
pub use bootstrap::start_http_server;
pub use controller::routes as person_routes;
pub use service::db::create_pool;
pub use service::person_service::PersonService;
pub use util::app_state::AppState;
