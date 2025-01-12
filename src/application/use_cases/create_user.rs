use crate::application::dto::create_user_dto::CreateUserDto;
use crate::core::services::user_service::UserService;

pub struct CreateUserUseCase {
    service: Box<dyn UserService>,
}

impl CreateUserUseCase {
    pub fn new(service: Box<dyn UserService>) -> Self {
        CreateUserUseCase { service }
    }

    pub fn execute(&self, dto: CreateUserDto) -> Result<(), String> {
        let last_id = self.service.get_last_user_id();
        self.service.create_user(last_id + 1, dto.name, dto.email)
    }
}
