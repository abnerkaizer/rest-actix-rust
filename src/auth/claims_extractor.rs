use actix_web::{Error, FromRequest, HttpRequest, HttpResponse, dev::Payload};
use futures_util::future::{Ready, ready};

use crate::{AppState, auth::claims::Claims, auth::jwt::validate_token};

impl FromRequest for Claims {
    type Error = Error;
    type Future = Ready<Result<Self, Self::Error>>;

    fn from_request(req: &HttpRequest, _payload: &mut Payload) -> Self::Future {
        let auth_header = req
            .headers()
            .get("Authorization")
            .and_then(|h| h.to_str().ok());

        let token = match auth_header.and_then(|h| h.strip_prefix("Bearer ")) {
            Some(t) => t,
            None => {
                return ready(Err(actix_web::error::ErrorUnauthorized(
                    "Missing or invalid authorization header",
                )));
            }
        };

        let secret = match req.app_data::<actix_web::web::Data<AppState>>() {
            Some(state) => state.secret(),
            None => {
                return ready(Err(actix_web::error::ErrorInternalServerError(
                    "App state not found",
                )));
            }
        };

        match validate_token(token, &secret) {
            Ok(claims) => ready(Ok(claims)),
            Err(_) => ready(Err(actix_web::error::ErrorUnauthorized("Invalid token"))),
        }
    }
}

pub fn require_admin(claims: &Claims) -> Result<(), HttpResponse> {
    if claims.is_admin() {
        Ok(())
    } else {
        Err(HttpResponse::Forbidden().json(serde_json::json!({
            "error": "Admin access required"
        })))
    }
}

pub fn require_self_or_admin(claims: &Claims, user_id: &uuid::Uuid) -> Result<(), HttpResponse> {
    if claims.is_admin() || claims.user_id() == user_id {
        Ok(())
    } else {
        Err(HttpResponse::Forbidden().json(serde_json::json!({
            "error": "You can only access your own resources"
        })))
    }
}
