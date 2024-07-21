use crate::domain::{
    entities::user::User, repo::user_repo::UserRepo, services::user_service::UserService,
};

pub struct GetUserUseCase<T: UserRepo> {
    user_service: UserService<T>,
}

impl<T: UserRepo> GetUserUseCase<T> {
    pub fn new(user_repo: T) -> Self {
        let user_service = UserService::new(user_repo);
        GetUserUseCase { user_service }
    }
    pub async fn get_user(&self, email: String) -> Option<User> {
        self.user_service.get_by_email(email).await
    }
}
