use crate::{
    service::db::DbPool,
    service::person_service::PersonService
};

pub struct AppState {
    pool: DbPool,
    person_service: PersonService,
}

impl AppState {
    pub fn new(pool: DbPool, person_service: PersonService) -> Self {
        Self{
            pool,
            person_service,
        }
    }

    pub fn pool(&self) -> DbPool{
        self.pool.clone()
    }

    pub fn person_service(&self) -> PersonService{
        self.person_service.clone()
    }
}