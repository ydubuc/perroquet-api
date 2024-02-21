use serde::Deserialize;
use validator::Validate;

#[derive(Debug, Deserialize, Validate)]
pub struct EditPasswordDto {
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
