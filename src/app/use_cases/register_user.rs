use crate::{
    domain::{repo::user_repo::UserRepo, services::user_service::UserService},
    present::handlers::user_handler::UserForCreate,
};

pub struct RegisterUserUseCase<T: UserRepo> {
    user_service: UserService<T>,
}

impl<T: UserRepo> RegisterUserUseCase<T> {
    pub fn new(user_repo: T) -> Self {
        let user_service = UserService::new(user_repo);
        RegisterUserUseCase { user_service }
    }
    pub async fn execute(&self, new_user: UserForCreate) -> Result<(), diesel::result::Error> {
        self.user_service.register_user(new_user).await
    }
}
