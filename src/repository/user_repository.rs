use diesel::prelude::*;

use crate::{
    model::user::{NewUser, User},
    schema::users::dsl::*,
};

pub struct UserRepository;

impl UserRepository {
    pub fn find_by_email(conn: &mut PgConnection, user_email: &str) -> QueryResult<User> {
        users
            .filter(email.eq(user_email))
            .select(User::as_select())
            .first(conn)
    }

    pub fn insert(conn: &mut PgConnection, new_user: NewUser) -> QueryResult<usize> {
        diesel::insert_into(users).values(new_user).execute(conn)
    }
}
