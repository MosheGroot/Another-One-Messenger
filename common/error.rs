#[derive(Debug)]
pub enum AppError {
    NotFound,
    Unauthorized,
    InternalServerError,
}

impl std::fmt::Display for AppError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:?}", self)
    }
}
