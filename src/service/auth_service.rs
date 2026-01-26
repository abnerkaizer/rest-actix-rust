use bcrypt::verify;
use bcrypt::{DEFAULT_COST, hash};
use diesel::PgConnection;
use uuid::Uuid;
use crate::{
    auth::jwt::generate_token, 
    model::user::NewUser, 
    repository::user_repository::UserRepository,
};

pub struct AuthService;

impl AuthService {
    pub fn login(
        conn: &mut PgConnection,
        email: String,
        password: String,
        secret: &str,
    ) -> Result<String, &'static str> {
        let user = UserRepository::find_by_email(conn, &email)
            .map_err(|_| "Usuário não encontrado")?;
        
        let valid = verify(password, &user.password_hash())
            .map_err(|_| "Erro ao verificar senha")?;
        
        if !valid {
            return Err("Credenciais inválidas");
        }
        generate_token(&user.id().to_string(), secret)
            .map_err(|_| "Erro ao gerar token")
    }

    pub fn register(
        conn: &mut PgConnection,
        email: String,
        password: String,
    ) -> Result<(), &'static str> {
        if password.len() < 6 {
            return Err("Senha muito curta");
        }
        
        if UserRepository::find_by_email(conn, &email).is_ok() {
            return Err("Usuário já existe");
        }
        
        let password_hash = hash(password, DEFAULT_COST)
            .map_err(|_| "Erro ao gerar hash da senha")?;
        
        let new_user = NewUser {
            id: Uuid::new_v4(),
            email,
            password_hash,
        };
        
        UserRepository::insert(conn, new_user)
            .map_err(|_| "Erro ao criar usuário")?;
        
        Ok(())
    }
}