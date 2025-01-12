use crate::application::dto::user_response_dto::UserResponseDto;
use crate::core::services::user_service::UserService;

pub struct GetUserUseCase {
    service: Box<dyn UserService>,
}

impl GetUserUseCase
{
    pub fn new(service: Box<dyn UserService>) -> Self {
        GetUserUseCase { service }
    }

    pub fn execute(&self, id: u32) -> Option<UserResponseDto> {
        self.service
            .find_user_by_id(id)
            .map(|user| UserResponseDto::new(user.id, user.name, user.email))
    }
}
