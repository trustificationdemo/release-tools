use octocrab::Octocrab;
use serde::Deserialize;

#[derive(Deserialize, Debug)]
struct EnvVars {
    pub github_token: String,
}

pub fn get_client() -> crate::error::Result<Octocrab> {
    let github_token = envy::from_env::<EnvVars>()?.github_token;

    let octocrab = Octocrab::builder().personal_token(github_token).build()?;

    Ok(octocrab)
}
