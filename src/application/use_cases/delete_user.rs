use crate::application::dto::delete_user_input_dto::DeleteUserInputDto;
use crate::application::dto::delete_user_output_dto::DeleteUserOutputDto;
use crate::core::services::user_service::UserService;

pub struct RemoveUserUseCase {
    service: Box<dyn UserService>,
}

impl RemoveUserUseCase {
    pub fn new(service: Box<dyn UserService>) -> Self {
        RemoveUserUseCase { service }
    }

    pub fn execute(&self, dto: DeleteUserInputDto) -> Result<DeleteUserOutputDto, String> {
        let id = dto.id;
        self.service
            .remove_user(id)
            .map(|user| DeleteUserOutputDto {
                id: user.id,
                name: user.name,
                email: user.email,
            })
    }
}
