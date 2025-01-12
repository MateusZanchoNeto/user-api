use crate::config::database_config::DatabaseType;
use crate::core::repositories::user_repository::UserRepository;
use crate::infrastructure::database::memory::memory_user_repository::MemoryUserRepository;
use crate::infrastructure::database::postgres::postgres_user_repository::PostgresUserRepository;

pub fn user_repository_factory(database_type: &DatabaseType) -> Box<dyn UserRepository> {
    match database_type {
        DatabaseType::Memory => Box::new(MemoryUserRepository::new()),
        DatabaseType::Postgres => Box::new(PostgresUserRepository::new().unwrap()),
    }
}
