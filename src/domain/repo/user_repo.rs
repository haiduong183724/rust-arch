use async_trait::async_trait;

use crate::{domain::entities::user::User, present::handlers::user_handler::UserForCreate};
#[async_trait]
pub trait UserRepo {
    async fn find_by_email(&self, email: String) -> Option<User>;
    async fn save(&self, user: UserForCreate) -> Result<(), diesel::result::Error>;
}
