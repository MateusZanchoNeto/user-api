use crate::core::domain::user::User;

pub trait UserService {
    fn create_user(&self, id: i32, name: String, email: String) -> Result<(), String>;
    fn find_user_by_id(&self, id: i32) -> Option<User>;
    fn remove_user(&self, id: i32) -> Result<(), String>;
    fn list_all_users(&self) -> Vec<User>;
    fn get_last_user_id(&self) -> i32;
    fn update_user(&self, id: i32, name: String, email: String) -> Result<(), String>;
}
