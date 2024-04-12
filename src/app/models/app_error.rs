#[derive(Debug)]
pub struct AppError {
    pub message: String,
}

impl AppError {
    pub fn new(message: &str) -> Self {
        Self {
            message: message.to_string(),
        }
    }
}
