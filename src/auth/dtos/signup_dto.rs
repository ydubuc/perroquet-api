use serde::Deserialize;
use validator::Validate;

#[derive(Debug, Deserialize, Validate)]
pub struct SignupDto {
    #[validate(length(
        min = 3,
        max = 24,
        message = "username must be between 3 and 24 characters."
    ))]
    #[validate(regex(path = "super::USERNAME_REGEX"))]
    pub username: Option<String>,
    #[validate(email)]
    pub email: String,
    #[validate(
        length(
            min = 8,
            max = 64,
            message = "password must be between at least 8 characters."
        ),
        custom = "super::validate_password"
    )]
    pub password: String,
}
