use diesel::result::Error as DieselError;

#[derive(Debug)]
pub enum UserServiceError {
    HashError,
    NotFound,
    DatabaseError(DieselError),
}

impl From<DieselError> for UserServiceError {
    fn from(err: DieselError) -> Self {
        match err {
            DieselError::NotFound => UserServiceError::NotFound,
            _ => UserServiceError::DatabaseError(err),
        }
    }
}

impl std::fmt::Display for UserServiceError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            UserServiceError::HashError => write!(f, "Erro ao gerar hash da senha"),
            UserServiceError::NotFound => write!(f, "Usuário não encontrado"),
            UserServiceError::DatabaseError(err) => write!(f, "Erro no banco de dados: {}", err),
        }
    }
}

impl std::error::Error for UserServiceError {}
