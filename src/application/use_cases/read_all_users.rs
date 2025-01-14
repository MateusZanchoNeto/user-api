use crate::application::dto::read_user_output_dto::ReadUserOutputDto;
use crate::core::services::user_service::UserService;

pub struct GetAllUsersUseCase {
    service: Box<dyn UserService>,
}

impl GetAllUsersUseCase {
    pub fn new(service: Box<dyn UserService>) -> Self {
        GetAllUsersUseCase { service }
    }

    pub fn execute(&self) -> Vec<ReadUserOutputDto> {
        self.service
            .list_all_users()
            .iter()
            .map(|user| ReadUserOutputDto {
                id: user.id,
                name: user.name.clone(),
                email: user.email.clone(),
            })
            .collect()
    }
}
