use crate::application::dto::update_user_input_dto::UpdateUserInputDto;
use crate::application::dto::update_user_output_dto::UpdateUserOutputDto;
use crate::core::services::user_service::UserService;

pub struct UpdateUserUseCase {
    service: Box<dyn UserService>,
}

impl UpdateUserUseCase {
    pub fn new(service: Box<dyn UserService>) -> Self {
        UpdateUserUseCase { service }
    }

    pub fn execute(&self, dto: UpdateUserInputDto) -> Result<UpdateUserOutputDto, String> {
        self.service
            .update_user(dto.id, dto.name, dto.email)
            .map(|user| UpdateUserOutputDto {
                id: user.id,
                name: user.name,
                email: user.email,
            })
    }
}
