use serde::Deserialize;

use crate::error::Result;

#[derive(Deserialize, Debug)]
pub struct GitHubVariables {
    pub ci: bool,
    pub github_actions: bool,
    pub github_event_name: String,
    pub github_event_path: String,
}

impl GitHubVariables {
    pub fn from_env() -> Result<Self> {
        let result = envy::from_env::<GitHubVariables>()?;
        Ok(result)
    }
}
