use std::env;

use actix_cors::Cors;
use actix_web::http::header;

#[derive(Clone)]
pub struct AppConfig {
    host: String,
    port: u16,
    database_url: String,
    secret: String,
    cors_allowed_origins: Vec<String>,
}

impl AppConfig {
    pub fn from_env() -> Self {
        let host = env::var("APP_HOST").unwrap_or_else(|_| "127.0.0.1".into());

        let port = env::var("APP_PORT")
            .unwrap_or_else(|_| "8080".into())
            .parse()
            .expect("APP_PORT must be a number");

        let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");
        let secret = env::var("SECRET").expect("SECRET must be set");

        let cors_allowed_origins = env::var("CORS_ALLOWED_ORIGINS")
            .unwrap_or_else(|_| "http://localhost:8081".into())
            .split(',')
            .map(|s| s.trim())
            .filter(|s| !s.is_empty())
            .map(|s| s.to_string())
            .collect::<Vec<_>>();

        Self {
            host,
            port,
            database_url,
            secret,
            cors_allowed_origins,
        }
    }

    pub fn cors(&self) -> Cors {
        let cors = self
            .cors_allowed_origins
            .iter()
            .fold(Cors::default(), |cors, origin| cors.allowed_origin(origin));

        cors.allowed_methods(["GET", "POST", "PUT", "PATCH", "DELETE"])
            .allowed_headers([header::AUTHORIZATION, header::ACCEPT])
            .allowed_header(header::CONTENT_TYPE)
            .supports_credentials()
            .max_age(3600)
    }

    pub fn host(&self) -> &str {
        &self.host
    }

    pub fn port(&self) -> u16 {
        self.port
    }

    pub fn database_url(&self) -> &str {
        &self.database_url
    }

    pub fn secret(&self) -> &str {
        &self.secret
    }

    pub fn cors_allowed_origins(&self) -> &[String] {
        &self.cors_allowed_origins
    }
}
