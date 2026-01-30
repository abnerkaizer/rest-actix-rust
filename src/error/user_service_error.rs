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
            UserServiceError::HashError => write!(f, "Error generating password hash"),
            UserServiceError::NotFound => write!(f, "User not found"),
            UserServiceError::DatabaseError(err) => write!(f, "Database error: {}", err),
        }
    }
}

impl std::error::Error for UserServiceError {}
