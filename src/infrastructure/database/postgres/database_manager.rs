use diesel::pg::PgConnection;
use diesel::r2d2::ConnectionManager;
use diesel_migrations::{embed_migrations, EmbeddedMigrations, MigrationHarness};
use r2d2::Pool;

pub const MIGRATIONS: EmbeddedMigrations = embed_migrations!("./migrations");

pub type ConnectionPool = Pool<ConnectionManager<PgConnection>>;

pub struct DatabaseManager {
    pool: ConnectionPool,
}

impl DatabaseManager {
    pub fn new(url: &str) -> Self {
        let pool = Self::init_pool(url);
        DatabaseManager { pool }
    }

    pub fn get_pool(&self) -> ConnectionPool {
        self.pool.clone()
    }

    fn init_pool(url: &str) -> ConnectionPool {
        let pool = Self::create_pool(url);
        Self::run_migrations(&pool);
        pool
    }

    fn create_pool(database_url: &str) -> ConnectionPool {
        let manager = ConnectionManager::new(database_url);
        Pool::builder()
            .max_size(12)
            .build(manager)
            .expect("Failed to create pool")
    }

    fn run_migrations(pool: &ConnectionPool) {
        let mut connection = pool.get().expect("Failed to get connection");
        connection.run_pending_migrations(MIGRATIONS).unwrap();
    }
}
