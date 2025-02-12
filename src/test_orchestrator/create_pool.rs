use crate::config::settings::Settings;

use crate::infrastructure::database::postgres::database_manager::{
    ConnectionPool, DatabaseManager,
};

pub fn create_pool() -> ConnectionPool {
    let settings = Settings::new();
    let database_manager = DatabaseManager::new(&format!(
        "postgres://{}:{}@{}/{}",
        settings.database_config.user,
        settings.database_config.password,
        settings.database_config.host,
        settings.database_config.database
    ));
    database_manager.get_pool()
}
