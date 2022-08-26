use aws_config::SdkConfig;
use serde_json::Value;

pub struct RequestContext {
    template_name: String,
    template_version: String,
    aws_sdk_config: SdkConfig,
    data: Value,
    headers: Value,
}

impl RequestContext {
    pub fn new(template_name: String, template_version: String, aws_sdk_config: SdkConfig, data: Value, headers: Value) -> Self {
        Self {
            template_name,
            template_version,
            aws_sdk_config,
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
    pub fn aws_sdk_config(&mut self) -> SdkConfig {
        self.aws_sdk_config.clone()
    }
    pub fn data(&mut self) -> Value {
        self.data.clone()
    }
    pub fn headers(&mut self) -> Value {
        self.headers.clone()
    }
}
