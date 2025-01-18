use crate::core::domain::user::User;
use crate::core::repositories::user_repository::UserRepository;
use std::sync::{Arc, Mutex};

pub struct MemoryUserRepository {
    users: Arc<Mutex<Vec<User>>>,
}

impl MemoryUserRepository {
    pub fn new() -> Self {
        MemoryUserRepository {
            users: Arc::new(Mutex::new(Vec::new())),
        }
    }
}

impl UserRepository for MemoryUserRepository {
    fn save_user(&self, user: &User) -> Result<User, String> {
        let mut users = self
            .users
            .lock()
            .map_err(|_| "Failed to lock users".to_string())?;
        users.push(self.clone_user(user));
        Ok(self.clone_user(user))
    }

    fn get_user_by_id(&self, id: i32) -> Option<User> {
        let users = self
            .users
            .lock()
            .map_err(|_| "Failed to lock users".to_string())
            .ok()?;
        users
            .iter()
            .find(|user| user.id == id)
            .map(|user| self.clone_user(user))
    }

    fn delete_user(&self, id: i32) -> Result<User, String> {
        let mut users = self
            .users
            .lock()
            .map_err(|_| "Failed to lock users".to_string())?;
        if let Some(pos) = users.iter().position(|user| user.id == id) {
            let user = self.clone_user(&users[pos]);
            users.remove(pos);
            Ok(user)
        } else {
            Err("User not found".to_string())
        }
    }

    fn list_users(&self) -> Vec<User> {
        let users = self.users.lock().unwrap();
        users.iter().map(|user| self.clone_user(user)).collect()
    }

    fn get_last_user(&self) -> Option<User> {
        let users = self.users.lock().unwrap();
        users
            .last()
            .map(|user| Some(self.clone_user(user)))
            .unwrap_or(None)
    }

    fn update_user(&self, user: &User) -> Result<User, String> {
        let mut users = self
            .users
            .lock()
            .map_err(|_| "Failed to lock users".to_string())?;
        if let Some(pos) = users.iter().position(|u| u.id == user.id) {
            users[pos] = self.clone_user(user);
            Ok(self.clone_user(user))
        } else {
            Err("User not found".to_string())
        }
    }

    #[cfg(test)]
    fn drop_database(&self) -> Result<(), String> {
        let mut users = self
            .users
            .lock()
            .map_err(|_| "Failed to lock users".to_string())?;
        users.clear();
        Ok(())
    }
}

impl MemoryUserRepository {
    fn clone_user(&self, user: &User) -> User {
        User::new(user.id, user.name.clone(), user.email.clone())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_save_user() {
        let repository = MemoryUserRepository::new();
        repository.drop_database().unwrap();
        let user = User::new(1, "John".to_string(), "john@email.com".to_string());
        assert_eq!(repository.save_user(&user), Ok(user));
    }

    #[test]
    fn test_get_user_by_id() {
        let repository = MemoryUserRepository::new();
        repository.drop_database().unwrap();
        let user = User::new(1, "John".to_string(), "john@email.com".to_string());
        repository.save_user(&user).unwrap();
        assert_eq!(repository.get_user_by_id(1), Some(user));
    }

    #[test]
    fn test_delete_user() {
        let repository = MemoryUserRepository::new();
        repository.drop_database().unwrap();
        let user = User::new(1, "John".to_string(), "john@email.com".to_string());
        repository.save_user(&user).unwrap();
        assert_eq!(repository.delete_user(1), Ok(user));
    }

    #[test]
    fn test_list_users() {
        let repository = MemoryUserRepository::new();
        repository.drop_database().unwrap();
        let user1 = User::new(1, "John".to_string(), "john@email.com".to_string());
        let user2 = User::new(2, "Jane".to_string(), "jane@email.com".to_string());
        repository.save_user(&user1).unwrap();
        repository.save_user(&user2).unwrap();
        assert_eq!(repository.list_users(), vec![user1, user2]);
    }

    #[test]
    fn test_get_last_user_id() {
        let repository = MemoryUserRepository::new();
        repository.drop_database().unwrap();
        let user1 = User::new(1, "John".to_string(), "john@email.com".to_string());
        let user2 = User::new(2, "Jane".to_string(), "jane@email.com".to_string());
        repository.save_user(&user1).unwrap();
        repository.save_user(&user2).unwrap();
        assert_eq!(repository.get_last_user().unwrap(), user2);
    }

    #[test]
    fn test_update_user() {
        let repository = MemoryUserRepository::new();
        repository.drop_database().unwrap();
        let user = User::new(1, "John".to_string(), "john@email.com".to_string());
        repository.save_user(&user).unwrap();
        let updated_user = User::new(1, "John Doe".to_string(), "john-doe@email.com".to_string());
        assert_eq!(repository.update_user(&updated_user), Ok(updated_user));
    }
}
