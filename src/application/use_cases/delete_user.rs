use crate::application::dto::delete_user_input_dto::DeleteUserInputDto;
use crate::core::services::user_service::UserService;

pub struct RemoveUserUseCase {
    service: Box<dyn UserService>,
}

impl RemoveUserUseCase {
    pub fn new(service: Box<dyn UserService>) -> Self {
        RemoveUserUseCase { service }
    }

    pub fn execute(&self, dto: DeleteUserInputDto) -> Result<(), String> {
        let id = dto.id;
        self.service.remove_user(id)
    }
}