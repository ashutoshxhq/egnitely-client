use aws_config::SdkConfig;
use serde_json::Value;
use std::error;
use std::fmt;

pub struct RequestContext {
    template_name: String,
    template_version: String,
    aws_sdk_config: Option<SdkConfig>,
    data: Value,
    headers: Value,
}

impl RequestContext {
    pub fn new(template_name: String, template_version: String, aws_sdk_config: Option<SdkConfig>, data: Value, headers: Value) -> Self {
        Self {
            template_name,
            template_version,
            aws_sdk_config: aws_sdk_config,
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
    pub fn aws_sdk_config(&mut self) -> Option<SdkConfig> {
        self.aws_sdk_config.clone()
    }
    pub fn data(&mut self) -> Value {
        self.data.clone()
    }
    pub fn headers(&mut self) -> Value {
        self.headers.clone()
    }
}

pub type Result<T> = std::result::Result<T, Box<dyn error::Error>>;

#[derive(Debug)]
pub struct HandlerError {
    message: String,
    code: String,
}

impl HandlerError {
    pub fn new(code:String, message:String)-> Self {
        Self { message, code }
    }
}

impl fmt::Display for HandlerError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "({}: {})", self.code, self.message)
    }
}

impl error::Error for HandlerError {}