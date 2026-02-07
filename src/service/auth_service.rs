use crate::model::role::Role;
use crate::{
    auth::jwt::generate_token, model::user::NewUser, repository::user_repository::UserRepository,
};
use bcrypt::verify;
use bcrypt::{DEFAULT_COST, hash};
use diesel::PgConnection;
use uuid::Uuid;

pub struct AuthService;

impl AuthService {
    pub fn login(
        conn: &mut PgConnection,
        email: String,
        password: String,
        secret: &str,
    ) -> Result<String, &'static str> {
        let user = UserRepository::find_by_email(conn, &email).map_err(|_| "User not found")?;

        let valid =
            verify(password, &user.password_hash()).map_err(|_| "Password verification error")?;

        if !valid {
            return Err("Invalid credentials");
        }

        generate_token(*user.id(), user.role().to_string(), secret, 24)
            .map_err(|_| "Token genation error")
    }

    pub fn register(
        conn: &mut PgConnection,
        email: String,
        role: Role,
        password: String,
    ) -> Result<Uuid, &'static str> {
        if password.len() < 6 {
            return Err("Password is too short");
        }

        if UserRepository::find_by_email(conn, &email).is_ok() {
            return Err("User already exists");
        }

        let password_hash =
            hash(password, DEFAULT_COST).map_err(|_| "Error generating password hash")?;

        let new_user = NewUser {
            id: Uuid::new_v4(),
            email,
            role,
            password_hash,
        };

        let id = new_user.id;

        UserRepository::insert(conn, new_user).map_err(|_| "User creation error")?;

        Ok(id)
    }
}
