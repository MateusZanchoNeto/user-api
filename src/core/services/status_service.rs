use crate::core::domain::status::Status;

pub trait StatusService {
    fn get_status(&self, database_name: &str) -> Result<Status, String>;
}
