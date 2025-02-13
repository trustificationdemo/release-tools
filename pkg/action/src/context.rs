use envy::Error;
use serde::Deserialize;

#[derive(Deserialize, Debug)]
pub struct GitHubVariables {
    pub ci: bool,
    pub github_actions: bool,
    pub github_event_name: String,
    pub github_event_path: String,
}

pub fn vars_from_env() -> Result<GitHubVariables, Error> {
    envy::from_env::<GitHubVariables>()
}
