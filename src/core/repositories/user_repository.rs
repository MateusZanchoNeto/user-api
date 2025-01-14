use crate::core::domain::user::User;

pub trait UserRepository {
    fn save_user(&self, user: &User) -> Result<(), String>;
    fn get_user_by_id(&self, id: i32) -> Option<User>;
    fn delete_user(&self, id: i32) -> Result<(), String>;
    fn list_users(&self) -> Vec<User>;
    fn get_last_user_id(&self) -> i32;

    fn update_user(&self, user: &User) -> Result<(), String>;

    #[cfg(test)]
    fn drop_database(&self) -> Result<(), String>;
}
