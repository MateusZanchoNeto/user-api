use crate::core::domain::user::User;
use crate::core::repositories::user_repository::UserRepository;
use crate::core::services::user_service::UserService;

// pub struct UserServiceImpl<R: UserRepository> {
//     repository: R,
// }
//
// impl<R: UserRepository> UserService<R> for UserServiceImpl<R> {
//     fn new(repository: R) -> Self {
//         Self { repository }
//     }
//
//     fn create_user(&self, id: u32, name: String, email: String) -> Result<(), String> {
//         let user = User::new(id, name, email);
//
//         if !user.validate_email() {
//             return Err("Invalid email address".to_string());
//         }
//
//         self.repository.save_user(&user)
//     }
//
//     fn find_user_by_id(&self, id: u32) -> Option<User> {
//         self.repository.get_user_by_id(id)
//     }
//
//     fn remove_user(&self, id: u32) -> Result<(), String> {
//         self.repository.delete_user(id)
//     }
//
//     fn list_all_users(&self) -> Vec<User> {
//         self.repository.list_users()
//     }
//
//     fn get_last_user_id(&self) -> u32 {
//         self.repository.get_last_user_id()
//     }
// }

pub struct UserServiceImpl {
    repository: Box<dyn UserRepository>,
}

impl UserServiceImpl {
    pub fn new(repository: Box<dyn UserRepository>) -> Self {
        UserServiceImpl { repository }
    }
}

impl UserService for UserServiceImpl {
    fn create_user(&self, id: u32, name: String, email: String) -> Result<(), String> {
        let user = User::new(id, name, email);

        if !user.validate_email() {
            return Err("Invalid email address".to_string());
        }

        self.repository.save_user(&user)
    }

    fn find_user_by_id(&self, id: u32) -> Option<User> {
        self.repository.get_user_by_id(id)
    }

    fn remove_user(&self, id: u32) -> Result<(), String> {
        self.repository.delete_user(id)
    }

    fn list_all_users(&self) -> Vec<User> {
        self.repository.list_users()
    }

    fn get_last_user_id(&self) -> u32 {
        self.repository.get_last_user_id()
    }
}

// TODO: Implement tests for UserServiceImpl
