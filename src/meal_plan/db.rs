use std::fmt::Error;

pub trait Connection {
    fn execute(&self, q: String) -> Result<String, Error>;
} 

pub struct dbConnection {
}

impl Connection for dbConnection {
    fn execute(&self, q: String) -> Result<String, Error> {
        return Ok(q)
    }
}
