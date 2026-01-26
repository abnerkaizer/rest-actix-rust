use chrono::NaiveDateTime;
use diesel::{AsChangeset, Insertable, Queryable, Selectable};
use uuid::Uuid;

use crate::schema::users;

#[derive(Debug, Queryable, Selectable)]
#[diesel(table_name = users)]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct User {
    id: Uuid,
    email: String,
    password_hash: String,
    created_at: NaiveDateTime,
}

#[derive(Insertable)]
#[diesel(table_name = users)]
pub struct NewUser {
    pub id: Uuid,
    pub email: String,
    pub password_hash: String,
}

#[derive(AsChangeset)]
#[diesel(table_name = users)]
pub struct UpdateUser {
    email: String,
    password_hash: String,
}

#[derive(AsChangeset)]
#[diesel(table_name = users)]
pub struct UpdatePassword {
    password_hash: String,
}

#[derive(AsChangeset)]
#[diesel(table_name = users)]
pub struct UpdateEmail {
    email: String,
}

impl UpdatePassword {
    pub fn new(password_hash: String) -> Self {
        Self { password_hash }
    }
}

impl UpdateEmail {
    pub fn new(email: String) -> Self {
        Self { email }
    }
}

impl UpdateUser {
    pub fn new(email: String, password_hash: String) -> Self {
        Self {
            email,
            password_hash,
        }
    }
}

impl User {
    pub fn id(&self) -> &Uuid {
        &self.id
    }

    pub fn email(&self) -> &str {
        &self.email
    }

    pub fn password_hash(&self) -> &str {
        &self.password_hash
    }

    pub fn created_at(&self) -> &NaiveDateTime {
        &self.created_at
    }
}
