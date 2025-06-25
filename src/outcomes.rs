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

pub struct HtrsOutcome {
    pub config_updated: bool,
    pub outcome_dialogue: Option<String>,
    pub action: Option<HtrsAction>,
}

impl HtrsOutcome {
    pub fn new(config_updated: bool, outcome_dialogue: Option<String>, action: Option<HtrsAction>) -> HtrsOutcome {
        HtrsOutcome { config_updated, outcome_dialogue, action }
    }
}

pub enum HtrsAction {
    MakeRequest {
        url: Url,
        headers: HashMap<String, String>,
        method: Method,
    }
}