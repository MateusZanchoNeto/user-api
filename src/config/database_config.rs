use std::env;

#[derive(Clone, Debug, PartialEq)]
pub enum DatabaseType {
    Memory,
    Postgres,
}

pub struct DatabaseConfig {
    pub database_type: DatabaseType,
    pub host: String,
    pub user: String,
    pub password: String,
    pub database: String,
}

impl DatabaseConfig {
    pub fn new() -> Self {
        let database_type = match env::var("DB_TYPE")
            .unwrap_or_else(|_| "memory".to_string())
            .as_str()
        {
            "memory" => DatabaseType::Memory,
            "postgres" => DatabaseType::Postgres,
            _ => DatabaseType::Memory,
        };
        let host = env::var("DB_HOST").unwrap_or_else(|_| "localhost".to_string());
        let user = env::var("DB_USER").unwrap_or_else(|_| "postgres".to_string());
        let password = env::var("DB_PASSWORD").unwrap_or_else(|_| "postgres".to_string());
        let database = env::var("DB_DATABASE").unwrap_or_else(|_| "postgres".to_string());
        DatabaseConfig {
            database_type,
            host,
            user,
            password,
            database,
        }
    }
}
