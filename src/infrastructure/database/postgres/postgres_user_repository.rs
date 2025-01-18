use crate::core::domain::user::User;
use crate::core::repositories::user_repository::UserRepository;
use crate::infrastructure::database::postgres::database_manager::ConnectionPool;
use crate::schema::users::dsl::*;
use diesel::prelude::*;
use diesel::result::Error;
use diesel::{Insertable, PgConnection, QueryDsl, Queryable, RunQueryDsl};
use serde::{Deserialize, Serialize};

#[derive(Insertable, Deserialize)]
#[diesel(table_name = crate::schema::users)]
struct NewUser {
    pub id: i32,
    pub name: String,
    pub email: String,
}

#[derive(Debug, Queryable, Serialize, Deserialize)]
struct UserEntity {
    pub id: i32,
    pub username: String,
    pub email: String,
}

fn create_user(conn: &mut PgConnection, new_user: NewUser) -> Result<UserEntity, Error> {
    diesel::insert_into(users)
        .values(&new_user)
        .get_result(conn)
}

fn get_users(conn: &mut PgConnection) -> Result<Vec<UserEntity>, Error> {
    users.load::<UserEntity>(conn)
}

fn get_user_by_id(conn: &mut PgConnection, user_id: i32) -> Result<UserEntity, Error> {
    users.filter(id.eq(user_id)).first(conn)
}

fn delete_user(conn: &mut PgConnection, user_id: i32) -> Result<usize, Error> {
    diesel::delete(users.filter(id.eq(user_id))).execute(conn)
}

fn get_last_user(conn: &mut PgConnection) -> Option<UserEntity> {
    users.order(id.desc()).first(conn).ok()
}

fn update_user_email(
    conn: &mut PgConnection,
    user_id: i32,
    new_email: String,
) -> Result<usize, Error> {
    diesel::update(users.filter(id.eq(user_id)))
        .set(email.eq(new_email))
        .execute(conn)
}

pub struct PostgresUserRepository {
    pool: ConnectionPool,
}

impl PostgresUserRepository {
    pub fn new(pool: ConnectionPool) -> Self {
        PostgresUserRepository { pool }
    }
}

impl UserRepository for PostgresUserRepository {
    fn save_user(&self, user: &User) -> Result<User, String> {
        let new_user = NewUser {
            id: user.id,
            name: user.name.clone(),
            email: user.email.clone(),
        };

        let mut connection = self
            .pool
            .get()
            .map_err(|e| format!("Error getting connection: {}", e))?;

        match create_user(&mut connection, new_user) {
            Ok(user) => Ok(User::new(user.id, user.username, user.email)),
            Err(e) => Err(format!("Error saving user: {}", e)),
        }
    }

    fn get_user_by_id(&self, user_id: i32) -> Option<User> {
        let mut connection = self
            .pool
            .get()
            .map_err(|e| format!("Error getting connection: {}", e))
            .ok()?;

        match get_user_by_id(&mut connection, user_id) {
            Ok(user) => Some(User::new(user.id, user.username, user.email)),
            Err(_) => None,
        }
    }

    fn delete_user(&self, user_id: i32) -> Result<User, String> {
        let mut connection = self
            .pool
            .get()
            .map_err(|e| format!("Error getting connection: {}", e))?;

        let user = match get_user_by_id(&mut connection, user_id) {
            Ok(user) => user,
            Err(_) => return Err("User not found".to_string()),
        };

        match delete_user(&mut connection, user_id) {
            Ok(_) => Ok(User::new(user.id, user.username, user.email)),
            Err(e) => Err(format!("Error deleting user: {}", e)),
        }
    }

    fn list_users(&self) -> Vec<User> {
        let mut connection = self
            .pool
            .get()
            .map_err(|e| format!("Error getting connection: {}", e))
            .unwrap();

        get_users(&mut connection)
            .unwrap()
            .into_iter()
            .map(|user| User::new(user.id, user.username, user.email))
            .collect()
    }

    fn get_last_user(&self) -> Option<User> {
        let mut connection = self
            .pool
            .get()
            .map_err(|e| format!("Error getting connection: {}", e))
            .unwrap();

        get_last_user(&mut connection).map(|user| User::new(user.id, user.username, user.email))
    }

    fn update_user(&self, user: &User) -> Result<User, String> {
        let mut connection = self
            .pool
            .get()
            .map_err(|e| format!("Error getting connection: {}", e))?;

        match update_user_email(&mut connection, user.id, user.email.clone()) {
            Ok(_) => Ok(User::new(user.id, user.name.clone(), user.email.clone())),
            Err(e) => Err(format!("Error updating user: {}", e)),
        }
    }

    #[cfg(test)]
    fn drop_database(&self) -> Result<(), String> {
        let mut connection = self
            .pool
            .get()
            .map_err(|e| format!("Error getting connection: {}", e))?;

        let query = r#"TRUNCATE TABLE users CASCADE"#;
        match diesel::sql_query(query).execute(&mut connection) {
            Ok(_) => Ok(()),
            Err(e) => Err(format!("Error dropping table: {}", e)),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::config::{env::load_enviroment, settings::Settings};
    use crate::infrastructure::database::postgres::database_manager::DatabaseManager;

    fn create_pool() -> ConnectionPool {
        load_enviroment();
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

    #[test]
    fn test_save_user() {
        let repository = PostgresUserRepository::new(create_pool());
        repository.drop_database().unwrap();
        let user = User::new(1, "John".to_string(), "john@email.com".to_string());
        assert_eq!(repository.save_user(&user), Ok(user));
    }

    #[test]
    fn test_get_user_by_id() {
        let repository = PostgresUserRepository::new(create_pool());
        repository.drop_database().unwrap();
        let user = User::new(1, "John".to_string(), "john@email.com".to_string());
        repository.save_user(&user).unwrap();
        assert_eq!(repository.get_user_by_id(1), Some(user));
    }

    #[test]
    fn test_delete_user() {
        let repository = PostgresUserRepository::new(create_pool());
        repository.drop_database().unwrap();
        let user = User::new(1, "John".to_string(), "john@email.com".to_string());
        repository.save_user(&user).unwrap();
        assert_eq!(repository.delete_user(1), Ok(user));
    }

    #[test]
    fn test_list_users() {
        let repository = PostgresUserRepository::new(create_pool());
        repository.drop_database().unwrap();
        let user1 = User::new(1, "John".to_string(), "john@email.com".to_string());
        let user2 = User::new(2, "Jane".to_string(), "jane@email.com".to_string());
        repository.save_user(&user1).unwrap();
        repository.save_user(&user2).unwrap();
        assert_eq!(repository.list_users(), vec![user1, user2]);
    }

    #[test]
    fn test_get_last_user_id() {
        let repository = PostgresUserRepository::new(create_pool());
        repository.drop_database().unwrap();
        let user1 = User::new(1, "John".to_string(), "john@email.com".to_string());
        let user2 = User::new(2, "Jane".to_string(), "jane@email.com".to_string());
        repository.save_user(&user1).unwrap();
        repository.save_user(&user2).unwrap();
        assert_eq!(repository.get_last_user().unwrap(), user2);
    }

    #[test]
    fn test_update_user() {
        let repository = PostgresUserRepository::new(create_pool());
        repository.drop_database().unwrap();
        let user = User::new(1, "John".to_string(), "john@email.com".to_string());
        repository.save_user(&user).unwrap();
        let updated_user = User::new(1, "John Doe".to_string(), "john-doe@email.com".to_string());
        assert_eq!(repository.update_user(&updated_user), Ok(updated_user));
    }
}
