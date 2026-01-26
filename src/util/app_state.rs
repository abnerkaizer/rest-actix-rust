use crate::{
    service::db::DbPool, 
    service::person_service::PersonService, 
    service::user_service::UserService,
};

pub struct AppState {
    pool: DbPool,
    person_service: PersonService,
    user_service: UserService,
    secret: String,
}

impl AppState {
    pub fn new(
        pool: DbPool, 
        person_service: PersonService, 
        user_service: UserService,
        secret: String,
    ) -> Self {
        Self {
            pool,
            person_service,
            user_service,
            secret,
        }
    }
    
    pub fn pool(&self) -> DbPool {
        self.pool.clone()
    }
    
    pub fn person_service(&self) -> PersonService {
        self.person_service.clone()
    }
    
    pub fn user_service(&self) -> UserService {
        self.user_service.clone()
    }
    
    pub fn secret(&self) -> &str {
        &self.secret
    }
}