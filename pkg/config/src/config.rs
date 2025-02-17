use std::fs;

use crate::{error::Result, types::Configuration};

impl Configuration {
    pub fn from_path(path: &str) -> Result<Self> {
        let data = fs::read_to_string(path)?;
        let data: Configuration = serde_yml::from_str(&data)?;
        Ok(data)
    }
}
