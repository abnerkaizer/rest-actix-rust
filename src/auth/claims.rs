use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Debug, Serialize, Deserialize)]
pub struct Claims {
    pub sub: Uuid,    // user_id
    pub role: String, // user role
    pub exp: usize,   // expiration time
}

impl Claims {
    pub fn new(user_id: Uuid, role: String, exp: usize) -> Self {
        Self {
            sub: user_id,
            role,
            exp,
        }
    }

    pub fn user_id(&self) -> &Uuid {
        &self.sub
    }

    pub fn role(&self) -> &str {
        &self.role
    }

    pub fn is_admin(&self) -> bool {
        self.role == "admin"
    }
}
