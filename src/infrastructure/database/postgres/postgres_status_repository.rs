use crate::core::domain::status::{Database, Status};
use crate::core::repositories::status_repository::StatusRepository;
use crate::infrastructure::database::postgres::database_manager::ConnectionPool;
use diesel::prelude::*;
use diesel::sql_query;
use diesel::sql_types::{Integer, Text};

#[derive(QueryableByName, Debug)]
struct ServerVersion {
    #[sql_type = "Text"]
    server_version: String,
}

#[derive(QueryableByName, Debug)]
struct MaxConnections {
    #[sql_type = "Integer"]
    max_connections: i32,
}

#[derive(QueryableByName, Debug)]
struct ActiveConnections {
    #[sql_type = "Integer"]
    count: i32,
}

pub struct PostgresStatusRepository {
    pool: ConnectionPool,
}

impl PostgresStatusRepository {
    pub fn new(pool: ConnectionPool) -> Self {
        PostgresStatusRepository { pool }
    }
}

impl StatusRepository for PostgresStatusRepository {
    fn get_status(&self, database_name: &str) -> Result<Status, String> {
        let mut connection = self
            .pool
            .get()
            .map_err(|e| format!("Error getting connection: {}", e))?;

        let server_version: String = sql_query("SHOW server_version;")
            .load::<ServerVersion>(&mut connection)
            .map_err(|e| format!("Error getting server_version: {}", e))?
            .first()
            .map(|v| v.server_version.clone())
            .unwrap_or_else(|| "Unknown".to_string());

        let max_connections: i32 = sql_query("SHOW max_connections;")
            .load::<MaxConnections>(&mut connection)
            .map_err(|e| format!("Error getting max_connections: {}", e))?
            .first()
            .map(|v| v.max_connections)
            .unwrap_or_else(|| 0);

        let active_connections: i32 =
            sql_query("SELECT count(*)::int FROM pg_stat_activity WHERE datname = $1;")
                .bind::<Text, _>(database_name)
                .load::<ActiveConnections>(&mut connection)
                .map_err(|e| format!("Error getting active connections: {}", e))?
                .first()
                .map(|v| v.count)
                .unwrap_or(0);

        Ok(Status {
            database: Database {
                version: server_version,
                max_connections,
                active_connections,
            },
        })
    }
}

#[cfg(test)]
mod tests {
    use crate::config::settings::Settings;
    use super::*;
    use crate::test_orchestrator::startup::startup;
    use crate::test_orchestrator::create_pool::create_pool;

    #[test]
    fn test_get_status() {
        startup();
        let settings = Settings::new();
        let repository = PostgresStatusRepository::new(create_pool());
        let status = repository
            .get_status(&settings.database_config.database)
            .unwrap();
        assert_eq!(status.database.max_connections, 100);
        assert_eq!(status.database.active_connections, 10);
        assert_eq!(status.database.version, "1.0.0");
    }
}
