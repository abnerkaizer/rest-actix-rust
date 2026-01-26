use std::env;

#[derive(Clone)]
pub struct AppConfig {
    host: String,
    port: u16,
    database_url: String,
    secret: String,
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

        Self {
            host,
            port,
            database_url,
            secret,
        }
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
}
