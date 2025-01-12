use crate::core::domain::user::User;

pub trait UserRepository {
    fn save_user(&self, user: &User) -> Result<(), String>;
    fn get_user_by_id(&self, id: u32) -> Option<User>;
    fn delete_user(&self, id: u32) -> Result<(), String>;
    fn list_users(&self) -> Vec<User>;
    fn get_last_user_id(&self) -> u32;

    #[cfg(test)]
    fn drop_database(&self) -> Result<(), String>;
}
