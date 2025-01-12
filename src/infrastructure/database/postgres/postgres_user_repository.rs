use crate::config::database_config::DatabaseConfig;
use crate::core::domain::user::User;
use crate::core::repositories::user_repository::UserRepository;
use tokio_postgres::{Client, Error, NoTls};

pub struct PostgresUserRepository {
    client: Client,
}

impl PostgresUserRepository {
    pub fn new() -> Result<Self, Error> {
        let config = DatabaseConfig::new();
        let (client, connection) =
            tokio::runtime::Runtime::new()
                .unwrap()
                .block_on(tokio_postgres::connect(
                    &format!(
                        "host={} user={} password={} dbname={}",
                        config.host, config.user, config.password, config.database
                    ),
                    NoTls,
                ))?;

        tokio::spawn(async move {
            if let Err(e) = connection.await {
                eprintln!("connection error: {}", e);
            }
        });

        Ok(Self { client })
    }
}

impl UserRepository for PostgresUserRepository {
    fn save_user(&self, user: &User) -> Result<(), String> {
        let query = "INSERT INTO users (id, name, email) VALUES ($1, $2, $3)";
        let result = tokio::runtime::Runtime::new().unwrap().block_on(
            self.client
                .execute(query, &[&user.id, &user.name, &user.email]),
        );

        match result {
            Ok(_) => Ok(()),
            Err(e) => Err(format!("Error saving user: {}", e)),
        }
    }

    fn get_user_by_id(&self, id: u32) -> Option<User> {
        let query = "SELECT id, name, email FROM users WHERE id = $1";
        let rows = tokio::runtime::Runtime::new()
            .unwrap()
            .block_on(self.client.query(query, &[&id]))
            .ok()?;

        rows.into_iter().next().map(|row| {
            User::new(
                row.get(0), // id
                row.get(1), // name
                row.get(2), // email
            )
        })
    }

    fn delete_user(&self, id: u32) -> Result<(), String> {
        let query = "DELETE FROM users WHERE id = $1";
        let result = tokio::runtime::Runtime::new()
            .unwrap()
            .block_on(self.client.execute(query, &[&id]));

        match result {
            Ok(_) => Ok(()),
            Err(e) => Err(format!("Error deleting user: {}", e)),
        }
    }

    fn list_users(&self) -> Vec<User> {
        let query = "SELECT id, name, email FROM users";
        let rows = tokio::runtime::Runtime::new()
            .unwrap()
            .block_on(self.client.query(query, &[]))
            .unwrap_or_default();

        rows.into_iter()
            .map(|row| {
                User::new(
                    row.get(0), // id
                    row.get(1), // name
                    row.get(2), // email
                )
            })
            .collect()
    }

    fn get_last_user_id(&self) -> u32 {
        let query = "SELECT id FROM users ORDER BY id DESC LIMIT 1";
        let rows = tokio::runtime::Runtime::new()
            .unwrap()
            .block_on(self.client.query(query, &[]))
            .unwrap_or_default();

        rows.into_iter().next().map(|row| row.get(0)).unwrap_or(0)
    }

    #[cfg(test)]
    fn drop_database(&self) -> Result<(), String> {
        let query = "DELETE FROM users";
        let result = tokio::runtime::Runtime::new()
            .unwrap()
            .block_on(self.client.execute(query, &[]));

        match result {
            Ok(_) => Ok(()),
            Err(e) => Err(format!("Error dropping database: {}", e)),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_save_user() {
        let repository = PostgresUserRepository::new().unwrap();
        repository.drop_database().unwrap();
        let user = User::new(1, "John".to_string(), "john@email.com".to_string());
        assert_eq!(repository.save_user(&user), Ok(()));
    }

    #[test]
    fn test_get_user_by_id() {
        let repository = PostgresUserRepository::new().unwrap();
        repository.drop_database().unwrap();
        let user = User::new(1, "John".to_string(), "john@email.com".to_string());
        repository.save_user(&user).unwrap();
        assert_eq!(repository.get_user_by_id(1), Some(user));
    }

    #[test]
    fn test_delete_user() {
        let repository = PostgresUserRepository::new().unwrap();
        repository.drop_database().unwrap();
        let user = User::new(1, "John".to_string(), "john@email.com".to_string());
        repository.save_user(&user).unwrap();
        assert_eq!(repository.delete_user(1), Ok(()));
    }

    #[test]
    fn test_list_users() {
        let repository = PostgresUserRepository::new().unwrap();
        repository.drop_database().unwrap();
        let user1 = User::new(1, "John".to_string(), "john@email.com".to_string());
        let user2 = User::new(2, "Jane".to_string(), "jane@email.com".to_string());
        repository.save_user(&user1).unwrap();
        repository.save_user(&user2).unwrap();
        assert_eq!(repository.list_users(), vec![user1, user2]);
    }

    #[test]
    fn test_get_last_user_id() {
        let repository = PostgresUserRepository::new().unwrap();
        repository.drop_database().unwrap();
        let user1 = User::new(1, "John".to_string(), "john@email.com".to_string());
        let user2 = User::new(2, "Jane".to_string(), "jane@email.com".to_string());
        repository.save_user(&user1).unwrap();
        repository.save_user(&user2).unwrap();
        assert_eq!(repository.get_last_user_id(), 2);
    }
}
