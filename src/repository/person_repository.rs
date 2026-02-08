use diesel::dsl::count_star;
use diesel::prelude::*;
use uuid::Uuid;

use crate::{
    model::person::{NewPerson, Person, UpdateCpf, UpdateName, UpdatePerson},
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

    pub fn find_all(conn: &mut PgConnection) -> Result<Vec<Person>, diesel::result::Error> {
        persons.load::<Person>(conn)
    }

    pub fn find_page(
        conn: &mut PgConnection,
        page: i64,
        per_page: i64,
    ) -> QueryResult<(i64, Vec<Person>)> {
        let page = page.max(1);
        let per_page = per_page.max(1);

        let total: i64 = persons.select(count_star()).first(conn)?;

        let offset = (page - 1) * per_page;

        let items = persons
            .order(id.asc())
            .limit(per_page)
            .offset(offset)
            .load::<Person>(conn)?;

        Ok((total, items))
    }

    pub fn find_by_id(conn: &mut PgConnection, person_id: Uuid) -> QueryResult<Person> {
        persons.find(person_id).first::<Person>(conn)
    }

    pub fn delete_person(conn: &mut PgConnection, person_id: Uuid) -> QueryResult<Person> {
        diesel::delete(persons.find(person_id)).get_result(conn)
    }

    pub fn update_person(
        conn: &mut PgConnection,
        person_id: Uuid,
        new_name: String,
        new_cpf: String,
    ) -> QueryResult<Person> {
        let changes = UpdatePerson::new(new_name, new_cpf);

        diesel::update(persons.find(person_id))
            .set(&changes)
            .get_result::<Person>(conn)
    }

    pub fn update_name(
        conn: &mut PgConnection,
        person_id: Uuid,
        new_name: String,
    ) -> QueryResult<Person> {
        let changes = UpdateName::new(new_name);

        diesel::update(persons.find(person_id))
            .set(&changes)
            .get_result::<Person>(conn)
    }

    pub fn update_cpf(
        conn: &mut PgConnection,
        person_id: Uuid,
        new_cpf: String,
    ) -> QueryResult<Person> {
        let changes = UpdateCpf::new(new_cpf);

        diesel::update(persons.find(person_id))
            .set(&changes)
            .get_result::<Person>(conn)
    }
}
