use std::error;
use std::fmt;

pub type Error = Box<dyn std::error::Error + Send + Sync + 'static>;

#[derive(Debug)]
pub struct HandlerError {
    error_message: String,
    error_code: String,
}

impl HandlerError {
    pub fn new(error_code: String, error_message: String) -> Box<Self> {
        Box::new(Self {
            error_code,
            error_message,
        })
    }
}

impl fmt::Display for HandlerError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "({}: {})",
            self.error_code, self.error_message
        )
    }
}

impl error::Error for HandlerError {}
