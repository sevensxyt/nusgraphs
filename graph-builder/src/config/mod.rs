use std::fmt;

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
    pub value: String,
}

#[derive(Serialize, Deserialize, Debug)]
pub enum FilterRule {
    #[serde(rename = "include")]
    Include,
    #[serde(rename = "exclude")]
    Exclude,
}

impl fmt::Display for FilterRule {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "{}",
            match self {
                FilterRule::Include => "include",
                FilterRule::Exclude => "exclude",
            }
        )
    }
}

impl Config {
    pub fn new() -> Result<Self, StorageError> {
        if let Some(config) = storage::read_object::<Config>() {
            println!("Loaded config from storage");
            Ok(config)
        } else {
            println!("No config found, defaulting to empty config");
            let config = Self::default();
            storage::write_object(&config)?;
            Ok(config)
        }
    }

    pub fn details(&self) -> String {
        let mut details = String::new();
        let limit = if let Some(limit) = self.limit {
            limit.to_string()
        } else {
            "None".to_string()
        };

        details.push_str(&format!("Limit: {limit}\n"));
        details.push_str(&format!(
            "Filters: {}",
            self.filters
                .iter()
                .map(|filter| format!("{}:{}:{}", filter.target, filter.rule, filter.value))
                .collect::<Vec<_>>()
                .join(", ")
        ));
        details
    }

    pub fn is_default(&self) -> bool {
        self.limit.is_none() && self.filters.is_empty()
    }
}
