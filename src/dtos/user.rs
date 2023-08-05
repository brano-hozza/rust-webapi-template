use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub struct SignupUserDTO {
    pub email: String,
    pub username: String,
    pub password: String,
}

#[derive(Debug, Deserialize)]
pub struct UpdateUserDTO {
    pub email: Option<String>,
    pub username: Option<String>,
    pub password: Option<String>,
}