use crate::{
    domain::{entities::user::User, repo::user_repo::UserRepo},
    present::handlers::user_handler::UserForCreate,
};

pub struct UserService<T: UserRepo> {
    user_repo: T,
}

impl<T: UserRepo> UserService<T> {
    pub fn new(user_repo: T) -> Self {
        UserService {
            user_repo: user_repo,
        }
    }

    pub async fn register_user(&self, user: UserForCreate) -> Result<(), diesel::result::Error> {
        self.user_repo.save(user).await
    }
    pub async fn get_by_email(&self, email: String) -> Option<User> {
        self.user_repo.find_by_email(email).await
    }
}
