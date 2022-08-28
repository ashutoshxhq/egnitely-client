use serde_json::Value;
use std::error;
use std::fmt;

pub struct Context {
    template_name: String,
    template_version: String,
    data: Value,
    headers: Value,
}

impl Context {
    pub fn new(
        template_name: String,
        template_version: String,
        data: Value,
        headers: Value,
    ) -> Self {
        Self {
            template_name,
            template_version,
            data,
            headers,
        }
    }

    pub fn template_name(&mut self) -> String {
        self.template_name.clone()
    }
    pub fn template_version(&mut self) -> String {
        self.template_version.clone()
    }
    pub fn data(&mut self) -> Value {
        self.data.clone()
    }
    pub fn headers(&mut self) -> Value {
        self.headers.clone()
    }
}

pub type Error = Box<dyn std::error::Error + Send + Sync + 'static>;

#[derive(Debug)]
pub struct HandlerError {
    error_message: String,
    error_code: String,
    status_code: u32,
}

impl HandlerError {
    pub fn new(error_code: String, error_message: String, status_code: u32) -> Box<Self> {
        Box::new(Self {
            error_code,
            error_message,
            status_code,
        })
    }
}

impl fmt::Display for HandlerError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "({} ({}): {})",
            self.error_code, self.status_code, self.error_message
        )
    }
}

impl error::Error for HandlerError {}
