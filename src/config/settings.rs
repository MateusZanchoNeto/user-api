use crate::config::database_config::{DatabaseConfig, DatabaseType};
use std::env;

#[derive(Clone)]
pub struct Settings {
    pub environment: String,
    pub port: u16,
    pub database_type: DatabaseType,
}

impl Settings {
    pub fn new() -> Self {
        println!("{}", env::var("DATABASE_URL").unwrap_or_else(|_| "development".to_string()));
        Self {
            environment: env::var("ENVIRONMENT").unwrap_or_else(|_| "development".to_string()),
            port: env::var("PORT")
                .unwrap_or_else(|_| "8080".to_string())
                .parse::<u16>()
                .unwrap(),
            database_type: DatabaseConfig::new().database_type,
        }
    }
}
