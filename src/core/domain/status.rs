pub struct Status {
    pub database: Database,
}

pub struct Database {
    pub max_connections: i32,
    pub active_connections: i32,
    pub version: String,
}
