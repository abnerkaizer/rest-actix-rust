use diesel::prelude::*;
use uuid::Uuid;

use bcrypt::{DEFAULT_COST, hash};

use crate::{
    error::user_service_error::UserServiceError,
    model::user::{NewUser, User},
    repository::user_repository::UserRepository,
    service::db::DbPool,
};

#[derive(Clone, Default)]
pub struct UserService;

impl UserService {
    pub fn new() -> Self {
        Self
    }

    pub fn find_by_email(&self, conn: &mut PgConnection, user_email: &str) -> QueryResult<User> {
        UserRepository::find_by_email(conn, user_email)
    }

    pub fn create_user(&self, conn: &mut PgConnection, new_user: NewUser) -> QueryResult<()> {
        UserRepository::insert(conn, new_user).map(|_| ())
    }

    pub fn delete_user(&self, conn: &mut PgConnection, user_id: Uuid) -> QueryResult<User> {
        UserRepository::delete_user(conn, user_id)
    }

    pub fn find_all(&self, pool: &DbPool) -> Result<Vec<User>, diesel::result::Error> {
        let mut conn = pool.get().expect("Failed to get DB connection");
        UserRepository::find_all(&mut conn)
    }

    pub fn update_user(
        &self,
        conn: &mut PgConnection,
        user_id: Uuid,
        new_email: String,
        new_password: String,
    ) -> QueryResult<User> {
        UserRepository::update_user(conn, user_id, new_email, new_password)
    }

    pub fn update_email(
        &self,
        conn: &mut PgConnection,
        user_id: Uuid,
        new_email: String,
    ) -> QueryResult<User> {
        UserRepository::update_email(conn, user_id, new_email)
    }

    pub fn update_password(
        &self,
        conn: &mut PgConnection,
        user_id: Uuid,
        new_password: String,
    ) -> Result<User, UserServiceError> {
        let hash = hash(new_password, DEFAULT_COST).map_err(|_| UserServiceError::HashError)?;

        Ok(UserRepository::update_password(conn, user_id, hash)?)
    }
}
