use crate::core::domain::user::User;

// pub trait UserService<R: UserRepository> {
//     fn new(repository: R) -> Self;
//     fn create_user(&self, id: u32, name: String, email: String) -> Result<(), String>;
//     fn find_user_by_id(&self, id: u32) -> Option<User>;
//     fn remove_user(&self, id: u32) -> Result<(), String>;
//     fn list_all_users(&self) -> Vec<User>;
//     fn get_last_user_id(&self) -> u32;
// }

pub trait UserService {
    fn create_user(&self, id: u32, name: String, email: String) -> Result<(), String>;
    fn find_user_by_id(&self, id: u32) -> Option<User>;
    fn remove_user(&self, id: u32) -> Result<(), String>;
    fn list_all_users(&self) -> Vec<User>;
    fn get_last_user_id(&self) -> u32;
}
