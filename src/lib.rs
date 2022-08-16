use std::error::Error;

pub struct Context {
    name: String, 
    version: String
}
pub trait EgnitelyContext {
    fn get_function_name(&mut self) -> Result<String, Box<dyn Error>>;
    fn get_version(&mut self) -> Result<String, Box<dyn Error>>;
}