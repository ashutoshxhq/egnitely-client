use serde_json::Value;


pub struct Context {
    template_name: String,
    template_version: String,
    config: Value,
    request: Value,
}

impl Context {
    pub fn new(
        template_name: String,
        template_version: String,
        config: Value,
        request: Value,
    ) -> Self {
        Self {
            template_name,
            template_version,
            config,
            request,
        }
    }

    pub fn template_name(&mut self) -> String {
        self.template_name.clone()
    }
    pub fn template_version(&mut self) -> String {
        self.template_version.clone()
    }
    pub fn config(&mut self) -> Value {
        self.config.clone()
    }
    pub fn request(&mut self) -> Value {
        self.request.clone()
    }
}
