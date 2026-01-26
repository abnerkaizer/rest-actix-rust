pub enum UserServiceError {
    HashError,
    Database(diesel::result::Error),
}

impl From<diesel::result::Error> for UserServiceError {
    fn from(err: diesel::result::Error) -> Self {
        Self::Database(err)
    }
}
