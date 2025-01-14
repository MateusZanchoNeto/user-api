use crate::application::dto::read_user_input_dto::ReadUserInputDto;
use crate::application::dto::read_user_output_dto::ReadUserOutputDto;
use crate::core::services::user_service::UserService;

pub struct GetUserUseCase {
    service: Box<dyn UserService>,
}

impl GetUserUseCase {
    pub fn new(service: Box<dyn UserService>) -> Self {
        GetUserUseCase { service }
    }

    pub fn execute(&self, dto: ReadUserInputDto) -> Option<ReadUserOutputDto> {
        let id = dto.id;
        self.service
            .find_user_by_id(id)
            .map(|user| ReadUserOutputDto {
                id: user.id,
                name: user.name.clone(),
                email: user.email.clone(),
            })
    }
}
