use action::context::GitHubVariables;
use pr::prefix::PRType;
use serde::Deserialize;
use std::fs;

mod error;

#[derive(Debug, Deserialize)]
struct Event {
    pub pull_request: PullRequestEvent,
}

#[derive(Debug, Deserialize)]
struct PullRequestEvent {
    title: String,
}

fn main() -> error::Result<()> {
    let gh_context = GitHubVariables::from_env()?;

    // Parse the event
    let event_file = fs::read_to_string(gh_context.github_event_path)?;

    let event: Event = serde_json::from_str(&event_file).map_err(|err| {
        crate::error::Error::UnmarshalPullRequest {
            file_path: event_file,
            err,
        }
    })?;

    // Check the title of the PR
    let pr_type = PRType::from_title(&event.pull_request.title)?;

    println!("{:?}", pr_type);
    Ok(())
}

#[cfg(test)]
mod tests {
    use crate::main;
    use tempfile::NamedTempFile;

    #[test]
    fn read_from_file() {
        std::env::set_var("CI", "true");
        std::env::set_var("GITHUB_ACTIONS", "true");
        std::env::set_var("GITHUB_EVENT_NAME", "foo");

        let event_that_generates_ok = "{\"pull_request\":{\"title\":\"WIP: :bug: Fix bug\"}}";
        let event_that_generates_error =
            "{\"pull_request\":{\"title\":\"WIP: [docs] Update documentation\"}}";

        //
        let temp_file = NamedTempFile::new().expect("Failed to create temp file");
        let path = temp_file.path(); // Get the temporary file's path
        std::fs::write(path, event_that_generates_ok).expect("Failed to write to temp file");

        std::env::set_var("GITHUB_EVENT_PATH", path);
        let result = main();
        assert!(result.is_ok());

        //
        let temp_file = NamedTempFile::new().expect("Failed to create temp file");
        let path = temp_file.path(); // Get the temporary file's path
        std::fs::write(path, event_that_generates_error).expect("Failed to write to temp file");

        std::env::set_var("GITHUB_EVENT_PATH", path);
        let result = main();
        assert!(result.is_err());
    }
}
