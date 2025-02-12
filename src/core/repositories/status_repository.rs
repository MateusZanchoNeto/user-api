use crate::core::domain::status::Status;

pub trait StatusRepository {
    fn get_status(&self, database_name: &str) -> Result<Status, String>;
}
