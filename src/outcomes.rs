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

    pub fn item_not_found(name: &str, searched_term: &str) -> Self {
        Self {
            details: format!("No {name} could be found with name `{searched_term}`")
        }
    }

    pub fn aliased_item_not_found(name: &str, searched_term: &str) -> Self {
        Self {
            details: format!("No {name} could be found with name or alias `{searched_term}`")
        }
    }

    pub fn item_already_exists(name: &str, conflicting_term: &str) -> Self {
        Self {
            details: format!("A {name} already exists with name `{conflicting_term}")
        }
    }

    pub fn aliased_item_already_exists(name: &str, conflicting_term: &str) -> Self {
        Self {
            details: format!("A {name} already exists with name or alias `{conflicting_term}`")
        }
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
        show_body: bool,
    },
}