use serde_json::Value;
use std::collections::HashMap;

#[derive(Debug, Clone, Default)]
pub struct ApplicationContext {
    pub name: String,
    pub version: String,
    pub config: Value,
}

impl ApplicationContext {
    pub fn new(name: String, version: String, config: Value) -> Self {
        Self {
            name,
            version,
            config,
        }
    }
}

#[derive(Debug, Default)]
pub struct FunctionRequest {
    json: Option<Value>,
    path: Option<HashMap<String, Value>>,
    query: Option<HashMap<String, Value>>,
    form: Option<HashMap<String, Value>>,
    multipart: Option<multer::Multipart<'static>>,
}

impl FunctionRequest {
    pub fn new(
        json: Option<Value>,
        path: Option<HashMap<String, Value>>,
        query: Option<HashMap<String, Value>>,
        form: Option<HashMap<String, Value>>,
        multipart: Option<multer::Multipart<'static>>,
    ) -> Self {
        Self {
            json,
            path,
            query,
            form,
            multipart,
        }
    }
}

#[derive(Debug, Default)]
pub struct Context {
    pub application: ApplicationContext,
    pub request: FunctionRequest,
}

impl Context {
    pub fn new(application: ApplicationContext, request: FunctionRequest) -> Self {
        Self {
            application,
            request,
        }
    }

    pub fn json(&self) -> Option<Value> {
        self.request.json.clone()
    }

    pub fn path(&self) -> Option<HashMap<String, Value>> {
        self.request.path.clone()
    }

    pub fn query(&self) -> Option<HashMap<String, Value>> {
        self.request.query.clone()
    }

    pub fn form(&self) -> Option<HashMap<String, Value>> {
        self.request.form.clone()
    }

    pub fn multipart(&self) -> Option<&multer::Multipart<'static>> {
        self.request.multipart.as_ref()
    }
}
