use std::error;

type Result<T> = std::result::Result<T, SabiError>;

#[derive(Debug, Clone)]
enum SabiError {
    InvalidWord,
    NetworkError
}

impl error::Error for SabiError {
    fn source(&self) -> Option<&(dyn error::Error + 'static)> {
        None
    }
}