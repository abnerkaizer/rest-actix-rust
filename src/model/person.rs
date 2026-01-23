use diesel::prelude::*;
use uuid::Uuid;

use crate::schema::persons;

#[derive(Queryable, Identifiable)]
#[diesel(table_name = persons)]
pub struct Person {
    id: Uuid,
    name: String,
    cpf: String,
}

#[derive(Insertable)]
#[diesel(table_name = persons)]
pub struct NewPerson {
    id: Uuid,
    name: String,
    cpf: String,
}

impl NewPerson {
    pub fn new(name: String, cpf: String) -> Self {
        Self {
            id: Uuid::new_v4(),
            name,
            cpf,
        }
    }
}

impl Person {
    pub fn name(&self) -> &str {
        &self.name
    }

    pub fn cpf(&self) -> &str {
        &self.cpf
    }

    pub fn id(&self) -> &uuid::Uuid {
        &self.id
    }
}