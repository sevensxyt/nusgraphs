use crate::errors::StorageError;
use crate::storage;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, Default)]
pub struct Config {
    pub limit: Option<usize>,
    pub filters: Vec<Filter>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Filter {
    pub target: String,
    pub rule: FilterRule,
}

#[derive(Serialize, Deserialize, Debug)]
pub enum FilterRule {
    Include,
    Exclude,
}

impl Config {
    pub fn new() -> Result<Self, StorageError> {
        if let Some(config) = storage::read_object::<Config>() {
            Ok(config)
        } else {
            let config = Self::default();
            storage::write_object(&config)?;
            Ok(config)
        }
    }
}
