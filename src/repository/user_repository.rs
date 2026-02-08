use diesel::dsl::count_star;
use diesel::prelude::*;
use uuid::Uuid;

use crate::{
    model::{
        role::Role,
        user::{NewUser, UpdateEmail, UpdatePassword, UpdateRole, UpdateUser, User},
    },
    schema::users::dsl::*,
};

pub struct UserRepository;

impl UserRepository {
    pub fn find_by_id(conn: &mut PgConnection, user_id: Uuid) -> QueryResult<User> {
        users.find(user_id).first::<User>(conn)
    }

    pub fn find_by_email(conn: &mut PgConnection, user_email: &str) -> QueryResult<User> {
        users
            .filter(email.eq(user_email))
            .select(User::as_select())
            .first(conn)
    }

    pub fn insert(conn: &mut PgConnection, new_user: NewUser) -> QueryResult<usize> {
        diesel::insert_into(users).values(new_user).execute(conn)
    }

    pub fn delete_user(conn: &mut PgConnection, user_id: Uuid) -> QueryResult<User> {
        diesel::delete(users.find(user_id)).get_result(conn)
    }

    pub fn find_all(conn: &mut PgConnection) -> Result<Vec<User>, diesel::result::Error> {
        users.load::<User>(conn)
    }

    pub fn find_page(
        conn: &mut PgConnection,
        page: i64,
        size: i64,
    ) -> QueryResult<(i64, Vec<User>)> {
        let page = page.max(1);
        let size = size.max(1);
        let offset = (page - 1) * size;

        let total: i64 = users.select(count_star()).first(conn)?;

        let items = users
            .order(id.asc())
            .limit(size)
            .offset(offset)
            .load::<User>(conn)?;

        Ok((total, items))
    }

    pub fn update_user(
        conn: &mut PgConnection,
        user_id: Uuid,
        new_email: String,
        new_role: Role,
        new_password_hash: String,
    ) -> QueryResult<User> {
        let changes = UpdateUser::new(new_email, new_role, new_password_hash);

        diesel::update(users.find(user_id))
            .set(&changes)
            .get_result::<User>(conn)
    }

    pub fn update_email(
        conn: &mut PgConnection,
        user_id: Uuid,
        new_email: String,
    ) -> QueryResult<User> {
        let changes = UpdateEmail::new(new_email);

        diesel::update(users.find(user_id))
            .set(&changes)
            .get_result::<User>(conn)
    }

    pub fn update_role(
        conn: &mut PgConnection,
        user_id: Uuid,
        new_role: Role,
    ) -> QueryResult<User> {
        let changes = UpdateRole::new(new_role);

        diesel::update(users.find(user_id))
            .set(&changes)
            .get_result::<User>(conn)
    }

    pub fn update_password(
        conn: &mut PgConnection,
        user_id: Uuid,
        new_password_hash: String,
    ) -> QueryResult<User> {
        let changes = UpdatePassword::new(new_password_hash);

        diesel::update(users.find(user_id))
            .set(&changes)
            .get_result::<User>(conn)
    }
}
