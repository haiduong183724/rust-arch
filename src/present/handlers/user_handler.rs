use actix_web::{
    get, post,
    web::{self, Path},
    HttpResponse, Responder,
};
use diesel::prelude::Insertable;
use log::error;
use serde::Deserialize;

use crate::{
    app::use_cases::{get_user::GetUserUseCase, register_user::RegisterUserUseCase},
    infras::repo::user_repo::PgUserRepo,
    utils::api_response::ApiResponseMessage,
};

#[derive(Debug, Clone, Deserialize, Insertable)]
#[diesel(table_name = crate::schema::users)]
pub struct UserForCreate {
    pub name: String,
    pub email: String,
    pub phone: String,
    pub address: String,
}

#[post("/users")]
pub async fn register_user_handler(
    repo: web::Data<PgUserRepo>,
    new_user: web::Json<UserForCreate>,
) -> impl Responder {
    match RegisterUserUseCase::new(repo.into_inner())
        .execute(new_user.into_inner())
        .await
    {
        Ok(_) => ApiResponseMessage::success_with_data("Successfully registered user"),
        Err(ex) => {
            error!("Failed to register user: {:?}", ex);
            ApiResponseMessage::error()
        }
    }
}

#[get("/users/{email}")]
pub async fn get_by_email(repo: web::Data<PgUserRepo>, path: Path<(String,)>) -> HttpResponse {
    match GetUserUseCase::new(repo.into_inner())
        .get_user(path.into_inner().0.clone())
        .await
    {
        Some(user) => HttpResponse::Ok().json(user),
        None => HttpResponse::NotFound().finish(),
    }
}
