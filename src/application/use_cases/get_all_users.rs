use crate::application::dto::user_response_dto::UserResponseDto;
use crate::core::services::user_service::UserService;

pub struct GetAllUsersUseCase {
    service: Box<dyn UserService>,
}

impl GetAllUsersUseCase {
    pub fn new(service: Box<dyn UserService>) -> Self {
        GetAllUsersUseCase { service }
    }

    pub fn execute(&self) -> Vec<UserResponseDto> {
        self.service
            .list_all_users()
            .iter()
            .map(|user| UserResponseDto::new(user.id, user.name.clone(), user.email.clone()))
            .collect()
    }
}
