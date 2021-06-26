use crate::models::BelimbingError;


impl From<std::io::Error> for BelimbingError {
    fn from(err: std::io::Error) -> Self {
        Self::ActixIoError(err)
    }
}
