use crate::{
    model::person::Person, repository::person_repository::PersonRepository, service::db::DbPool,
};

use uuid::Uuid;

#[derive(Clone)]
pub struct PersonService;

impl PersonService {
    pub fn new() -> Self {
        Self
    }

    pub fn create_person(
        &self,
        pool: &DbPool,
        name: String,
        cpf: String,
    ) -> Result<Person, diesel::result::Error> {
        PersonRepository::create(pool, name, cpf)
    }

    pub fn find_by_id(&self, pool: &DbPool, id: Uuid) -> Result<Person, diesel::result::Error> {
        let mut conn = pool.get().expect("Failed to get DB connection");
        PersonRepository::find_by_id(&mut conn, id)
    }

    pub fn delete(&self, pool: &DbPool, id: Uuid) -> Result<Person, diesel::result::Error> {
        let mut conn = pool.get().expect("Failed to get DB connection");
        PersonRepository::delete(&mut conn, id)
    }

    pub fn update_name(
        &self,
        pool: &DbPool,
        id: Uuid,
        name: String,
    ) -> Result<Person, diesel::result::Error> {
        let mut conn = pool.get().expect("Failed to get DB connection");
        PersonRepository::update_name(&mut conn, id, name)
    }

    pub fn update_cpf(
        &self,
        pool: &DbPool,
        id: Uuid,
        cpf: String,
    ) -> Result<Person, diesel::result::Error> {
        let mut conn = pool.get().expect("Failed to get DB connection");
        PersonRepository::update_cpf(&mut conn, id, cpf)
    }
}
