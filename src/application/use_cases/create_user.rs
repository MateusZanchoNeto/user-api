use crate::application::dto::create_user_input_dto::CreateUserInputDto;
use crate::application::dto::create_user_output_dto::CreateUserOutputDto;
use crate::core::services::user_service::UserService;

pub struct CreateUserUseCase {
    service: Box<dyn UserService>,
}

impl CreateUserUseCase {
    pub fn new(service: Box<dyn UserService>) -> Self {
        CreateUserUseCase { service }
    }

    pub fn execute(&self, dto: CreateUserInputDto) -> Result<CreateUserOutputDto, String> {
        let last_id = self.service.get_last_user().map_or(0, |user| user.id + 1);
        self.service
            .create_user(last_id, dto.name, dto.email)
            .map(|user| CreateUserOutputDto {
                id: user.id,
                name: user.name,
                email: user.email,
            })
    }
}
