use std::fmt::Error;

pub trait Connection {
    fn execute(&self, q: String) -> Result<String, Error>;
}

pub struct DBConnection {}

impl Connection for DBConnection {
    fn execute(&self, q: String) -> Result<String, Error> {
        return Ok(q);
    }
}
