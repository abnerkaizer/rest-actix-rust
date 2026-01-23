use diesel::prelude::*;
use uuid::Uuid;

use crate::{
    model::person::{NewPerson, Person},
    schema::persons::dsl::*,
    service::db::DbPool,
};

pub struct PersonRepository;

impl PersonRepository {
    pub fn create(
        pool: &DbPool,
        name_value: String,
        cpf_value: String,
    ) -> Result<Person, diesel::result::Error> {
        let mut conn = pool.get().expect("DB connection");

        let new_person = NewPerson::new(name_value, cpf_value);

        diesel::insert_into(persons)
            .values(&new_person)
            .get_result(&mut conn)
    }

    pub fn find_by_id(
        conn: &mut PgConnection,
        person_id: Uuid,
    ) -> QueryResult<Person> {
        persons.find(person_id).first::<Person>(conn)
    }
}