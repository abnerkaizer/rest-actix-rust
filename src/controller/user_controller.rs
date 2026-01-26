use actix_web::{HttpResponse, Scope, get, web};

use crate::{dto::user_dto::UserResponse, util::app_state::AppState};

pub fn routes() -> Scope {
    web::scope("/user").service(find_all_users)
}

#[get("")]
async fn find_all_users(state: web::Data<AppState>) -> HttpResponse {
    let pool = state.pool().clone();
    let service = state.user_service().clone();

    let result = web::block(move || service.find_all(&pool)).await;

    match result {
        Ok(Ok(users)) => {
            let response: Vec<UserResponse> = users
                .into_iter()
                .map(|user| UserResponse {
                    id: *user.id(),
                    email: user.email().to_string(),
                })
                .collect();

            HttpResponse::Ok().json(response)
        }
        Ok(Err(diesel::result::Error::NotFound)) => HttpResponse::NotFound().finish(),
        _ => HttpResponse::InternalServerError().finish(),
    }
}
