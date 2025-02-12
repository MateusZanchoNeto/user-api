use crate::core::domain::status::Status;
use crate::core::domain::status::Database;
use std::sync::{Arc, Mutex};
use crate::core::repositories::status_repository::StatusRepository;

pub struct MemoryStatusRepository {
    status: Arc<Mutex<Status>>,
}

impl MemoryStatusRepository {
    pub fn new() -> Self {
        MemoryStatusRepository {
            status: Arc::new(Mutex::new(Status {
                database: Database {
                    max_connections: 100,
                    active_connections: 10,
                    version: "1.0.0".to_string(),
                },
            })),
        }
    }
}

impl StatusRepository for MemoryStatusRepository {
    fn get_status(&self, _database_name: &str) -> Result<Status, String> {
        Ok(self.status.lock().unwrap().clone())
    }
}

impl Clone for Status {
    fn clone(&self) -> Self {
        Status {
            database: Database {
                max_connections: self.database.max_connections,
                active_connections: self.database.active_connections,
                version: self.database.version.clone(),
            },
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::config::settings::Settings;
    use super::*;

    #[test]
    fn test_get_status() {
        let settings = Settings::new();
        let repository = MemoryStatusRepository::new();
        let status = repository.get_status(&settings.database_config.database).unwrap();
        assert_eq!(status.database.max_connections, 100);
        assert_eq!(status.database.active_connections, 10);
        assert_eq!(status.database.version, "1.0.0");
    }
}
