use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub struct SignupUserDTO {
    pub email: String,
    pub username: String,
    pub password: String,
}
