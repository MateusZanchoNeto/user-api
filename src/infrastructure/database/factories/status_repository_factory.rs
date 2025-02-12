use crate::core::repositories::status_repository::StatusRepository;
use crate::infrastructure::database::memory::memory_status_repository::MemoryStatusRepository;
use crate::infrastructure::database::postgres::database_manager::ConnectionPool;
use crate::infrastructure::database::postgres::postgres_status_repository::PostgresStatusRepository;

pub fn status_repository_factory(pool: &Option<ConnectionPool>) -> Box<dyn StatusRepository> {
    match pool {
        None => Box::new(MemoryStatusRepository::new()),
        Some(pool) => Box::new(PostgresStatusRepository::new(pool.clone())),
    }
}
