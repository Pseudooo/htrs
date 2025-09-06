use reqwest::{Method, Url};
use std::collections::HashMap;
use std::error::Error;
use std::fmt;

#[derive(Debug)]
pub struct HtrsError {
    pub details: String,
}

impl HtrsError {
    pub fn new(msg: &str) -> HtrsError {
        HtrsError { details: msg.to_string() }
    }
}

impl fmt::Display for HtrsError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.details)
    }
}

impl Error for HtrsError {
    fn description(&self) -> &str {
        &self.details
    }
}

pub enum HtrsAction {
    UpdateConfig,
    PrintDialogue(String),
    MakeRequest {
        url: Url,
        query_parameters: HashMap<String, String>,
        method: Method,
        headers: HashMap<String, String>,
    },
}