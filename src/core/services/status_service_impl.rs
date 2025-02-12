use crate::core::domain::status::Status;
use crate::core::repositories::status_repository::StatusRepository;
use crate::core::services::status_service::StatusService;

pub struct StatusServiceImpl {
    repository: Box<dyn StatusRepository>,
}

impl StatusService for StatusServiceImpl {
    fn get_status(&self, database_name: &str) -> Result<Status, String> {
        self.repository.get_status(database_name)
    }
}

impl StatusServiceImpl {
    pub fn new(repository: Box<dyn StatusRepository>) -> Self {
        StatusServiceImpl { repository }
    }
}
