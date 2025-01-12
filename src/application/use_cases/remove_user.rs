use crate::core::services::user_service::UserService;

pub struct RemoveUserUseCase {
    service: Box<dyn UserService>,
}

impl RemoveUserUseCase {
    pub fn new(service: Box<dyn UserService>) -> Self {
        RemoveUserUseCase { service }
    }

    pub fn execute(&self, id: u32) -> Result<(), String> {
        self.service.remove_user(id)
    }
}