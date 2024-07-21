use core::result::Result;
use std::sync::Arc;

use crate::present::handlers::user_handler::UserForCreate;
use crate::schema;
use crate::schema::users::dsl::*;
use crate::{
    domain::{entities::user::User, repo::user_repo::UserRepo},
    infras::db::connection::{establish_connnection, DBPool},
};
use async_trait::async_trait;
use diesel::prelude::*;
pub struct PgUserRepo {
    pool: DBPool,
}

impl PgUserRepo {
    pub fn new() -> Self {
        dotenv::dotenv().expect("failed to find .env file");
        let db_url = std::env::var("DATABASE_URL").expect("DATABASE_URL must be set");
        PgUserRepo {
            pool: establish_connnection(&db_url),
        }
    }
}

#[async_trait]
impl UserRepo for Arc<PgUserRepo> {
    async fn find_by_email(&self, input_email: String) -> Option<User> {
        users
            .filter(email.eq(input_email))
            .first::<User>(&mut self.pool.get().unwrap())
            .optional()
            .expect("error loading user")
    }
    async fn save(&self, new_user: UserForCreate) -> Result<(), diesel::result::Error> {
        diesel::insert_into(schema::users::table)
            .values(new_user)
            .execute(&mut self.pool.get().unwrap())
            .unwrap();

        Ok(())
    }
}
