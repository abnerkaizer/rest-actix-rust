use crate::{AppState, auth::jwt::validate_token};
use actix_web::{
    Error, HttpMessage, HttpResponse,
    body::BoxBody,
    dev::{Service, ServiceRequest, ServiceResponse, Transform, forward_ready},
    web,
};
use futures_util::future::{LocalBoxFuture, Ready, ok};

pub struct AuthMiddleware;

impl<S> Transform<S, ServiceRequest> for AuthMiddleware
where
    S: Service<ServiceRequest, Response = ServiceResponse<BoxBody>, Error = Error> + 'static,
{
    type Response = ServiceResponse<BoxBody>;
    type Error = Error;
    type InitError = ();
    type Transform = AuthMiddlewareService<S>;
    type Future = Ready<Result<Self::Transform, Self::InitError>>;

    fn new_transform(&self, service: S) -> Self::Future {
        ok(AuthMiddlewareService { service })
    }
}

pub struct AuthMiddlewareService<S> {
    service: S,
}

impl<S> Service<ServiceRequest> for AuthMiddlewareService<S>
where
    S: Service<ServiceRequest, Response = ServiceResponse<BoxBody>, Error = Error> + 'static,
{
    type Response = ServiceResponse<BoxBody>;
    type Error = Error;
    type Future = LocalBoxFuture<'static, Result<Self::Response, Self::Error>>;

    forward_ready!(service);

    fn call(&self, req: ServiceRequest) -> Self::Future {
        let secret = req
            .app_data::<web::Data<AppState>>()
            .map(|state| state.secret());

        let auth_header = req
            .headers()
            .get("Authorization")
            .and_then(|h| h.to_str().ok());

        let claims = if let Some(secret) = secret {
            auth_header
                .and_then(|h| h.strip_prefix("Bearer "))
                .and_then(|token| validate_token(token, &secret).ok())
        } else {
            None
        };

        if claims.is_none() {
            let res = HttpResponse::Unauthorized().json(serde_json::json!({
                "error": "Authentication required"
            }));
            return Box::pin(async move { Ok(req.into_response(res)) });
        }

        req.extensions_mut().insert(claims.unwrap());

        let fut = self.service.call(req);
        Box::pin(async move { fut.await })
    }
}
