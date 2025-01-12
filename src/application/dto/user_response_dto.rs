use serde::Serialize;

#[derive(Debug, Serialize)]
pub struct UserResponseDto {
    pub id: u32,
    pub name: String,
    pub email: String,
}

impl UserResponseDto {
    pub fn new(id: u32, name: String, email: String) -> Self {
        UserResponseDto { id, name, email }
    }
}
