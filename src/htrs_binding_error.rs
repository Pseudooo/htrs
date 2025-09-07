use std::error::Error;
use std::fmt;

#[derive(Debug)]
pub struct HtrsBindingError {
    pub description: String,
}

impl fmt::Display for HtrsBindingError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description)
    }
}

impl Error for HtrsBindingError {
    fn description(&self) -> &str {
        self.description.as_str()
    }
}
