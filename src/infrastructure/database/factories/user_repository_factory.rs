use crate::core::repositories::user_repository::UserRepository;
use crate::infrastructure::database::memory::memory_user_repository::MemoryUserRepository;
use crate::infrastructure::database::postgres::database_manager::ConnectionPool;
use crate::infrastructure::database::postgres::postgres_user_repository::PostgresUserRepository;

pub fn user_repository_factory(pool: &Option<ConnectionPool>) -> Box<dyn UserRepository> {
    match pool {
        None => Box::new(MemoryUserRepository::new()),
        Some(pool) => Box::new(PostgresUserRepository::new(pool.clone())),
    }
}
