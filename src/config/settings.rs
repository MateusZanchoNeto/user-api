use crate::config::database_config::DatabaseConfig;
use crate::infrastructure::database::postgres::database_manager::ConnectionPool;
use std::env;

#[derive(Clone)]
pub struct Settings {
    pub environment: String,
    pub port: u16,
    pub database_config: DatabaseConfig,
    pub connection_pool: Option<ConnectionPool>,
}

impl Settings {
    pub fn new() -> Self {
        Self {
            environment: env::var("ENVIRONMENT").unwrap_or_else(|_| "development".to_string()),
            port: env::var("PORT")
                .unwrap_or_else(|_| "8080".to_string())
                .parse::<u16>()
                .unwrap(),
            database_config: DatabaseConfig::new(),
            connection_pool: None,
        }
    }
}
