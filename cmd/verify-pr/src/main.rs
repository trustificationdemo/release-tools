use action::context::vars_from_env;
use pr::prefix::{PRType, PRTypeError};
use serde::Deserialize;
use std::fs;

#[derive(Debug, Deserialize)]
struct Event {
    pub pull_request: PullRequestEvent,
}

#[derive(Debug, Deserialize)]
struct PullRequestEvent {
    title: String,
}

fn main() -> Result<(), PRTypeError> {
    let gh_context = vars_from_env().expect("Failed to get github env vars");

    // Parse the event
    let event_file =
        fs::read_to_string(gh_context.github_event_path).expect("Unable to read github event file");

    let event: Event =
        serde_json::from_str(&event_file).expect("unable to unmarshal PullRequest event");

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
