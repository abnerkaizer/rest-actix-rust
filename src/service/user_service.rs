use bcrypt::{DEFAULT_COST, hash};
use diesel::prelude::*;
use uuid::Uuid;

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

    pub fn find_by_id(&self, pool: &DbPool, id: Uuid) -> Result<User, diesel::result::Error> {
        let mut conn = pool.get().expect("Failed to get DB connection");
        UserRepository::find_by_id(&mut conn, id)
    }

    pub fn find_by_email(&self, conn: &mut PgConnection, user_email: &str) -> QueryResult<User> {
        UserRepository::find_by_email(conn, user_email)
    }

    pub fn create_user(&self, conn: &mut PgConnection, new_user: NewUser) -> QueryResult<()> {
        UserRepository::insert(conn, new_user).map(|_| ())
    }

    pub fn delete_user(&self, pool: &DbPool, id: Uuid) -> Result<User, diesel::result::Error> {
        let mut conn = pool.get().expect("Failed to get DB connection");
        UserRepository::delete_user(&mut conn, id)
    }

    pub fn find_all(&self, pool: &DbPool) -> Result<Vec<User>, diesel::result::Error> {
        let mut conn = pool.get().expect("Failed to get DB connection");
        UserRepository::find_all(&mut conn)
    }

    pub fn update_user(
        &self,
        pool: &DbPool,
        user_id: Uuid,
        new_email: String,
        new_role: String,
        new_password: String,
    ) -> QueryResult<User> {
        let mut conn = pool.get().expect("Failed to get DB connection");

        let new_password_hash = hash(new_password, DEFAULT_COST)
            .map_err(|_e| diesel::result::Error::RollbackTransaction)?;
        UserRepository::update_user(&mut conn, user_id, new_email, new_role, new_password_hash)
    }

    pub fn update_email(
        &self,
        pool: &DbPool,
        id: Uuid,
        email: String,
    ) -> Result<User, diesel::result::Error> {
        let mut conn = pool.get().expect("Failed to get DB connection");
        UserRepository::update_email(&mut conn, id, email)
    }

    pub fn update_role(
        &self,
        pool: &DbPool,
        id: Uuid,
        role: String,
    ) -> Result<User, diesel::result::Error> {
        let mut conn = pool.get().expect("Failed to get DB connection");
        UserRepository::update_role(&mut conn, id, role)
    }

    pub fn update_password(
        &self,
        pool: &DbPool,
        user_id: Uuid,
        new_password: String,
    ) -> Result<User, UserServiceError> {
        let password_hash =
            hash(new_password, DEFAULT_COST).map_err(|_| UserServiceError::HashError)?;

        let mut conn = pool.get().map_err(|e| {
            UserServiceError::DatabaseError(diesel::result::Error::DatabaseError(
                diesel::result::DatabaseErrorKind::UnableToSendCommand,
                Box::new(e.to_string()),
            ))
        })?;

        UserRepository::update_password(&mut conn, user_id, password_hash).map_err(|e| e.into())
    }
}
