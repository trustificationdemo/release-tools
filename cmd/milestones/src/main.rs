use std::{
    collections::HashMap,
    process::{ExitCode, Termination},
};

use action::{client, commands::ActionCommand};
use clap::Parser;
use config::types::Milestone;
use octocrab::Page;
use serde::Serialize;
use serde_json::{json, Value};

mod error;

#[derive(Clone, Serialize)]
enum Why {
    // Wanted
    Missing(Milestone),
    // Wanted, CurrentNumber, Current
    Changed(Milestone, i64, Milestone),
}

#[derive(Clone, Serialize)]
struct Update {
    org: String,
    repo: String,
    why: Why,
    issues: Vec<octocrab::models::issues::Issue>,
}

#[derive(clap::Parser, Debug)]
#[command(
    author,
    version = env ! ("CARGO_PKG_VERSION"),
    long_about = None
)]
struct Cli {
    /// Path to config.yaml
    #[arg(long, default_value = "")]
    config: String,

    /// Make mutating changes to labels via GitHub API
    #[arg(long, default_value = "false")]
    confirm: bool,
}

#[tokio::main]
async fn main() -> impl Termination {
    match exec().await {
        Ok(_) => ExitCode::SUCCESS,
        Err(error) => {
            println!("{:#?}", error);
            ExitCode::FAILURE
        }
    }
}

async fn exec() -> crate::error::Result<()> {
    let cli = Cli::parse();

    let configuration = config::types::Configuration::from_path(&cli.config)?;

    // Instantiate the client and get the current labels on the repo
    let client = client::get_client()?;

    let mut updates: Vec<Update> = vec![];
    for repo in &configuration.repos {
        let mut current_milestones: Vec<octocrab::models::Milestone> = vec![];

        // Fetch all milestones
        let mut page: Page<octocrab::models::Milestone> = client
            .get(
                format!("/repos/{}/{}/milestones", &repo.org, &repo.repo),
                None::<&()>,
            )
            .await?;

        let items = page.items;
        current_milestones.extend(items);

        while let Some(next_page) = client
            .get_page::<octocrab::models::Milestone>(&page.next)
            .await?
        {
            page = next_page;

            let items = page.items;
            current_milestones.extend(items);
        }

        let current_milestones_map: HashMap<String, octocrab::models::Milestone> =
            current_milestones
                .into_iter()
                .map(|m| (m.title.clone(), m.clone()))
                .collect();

        // Compare milestones
        for want_milestone in &configuration.milestones {
            println!("Wanted milestone: {want_milestone}");

            let mut repo_issues: Vec<octocrab::models::issues::Issue> = vec![];

            if let Some(replaces) = &want_milestone.replaces {
                match current_milestones_map.get(replaces) {
                    Some(old_milestone) => {
                        if let Some(open_issues) = old_milestone.open_issues {
                            if open_issues > 0 {
                                println!("old milestone exists: want milestone title: {:?} replaces: {:?} open issues: {:?}", want_milestone.title, want_milestone.replaces, open_issues);

                                let mut page: u32 = 1;
                                loop {
                                    let resp: Page<octocrab::models::issues::Issue> = client
                                        .issues(&repo.org, &repo.repo)
                                        .list()
                                        .page(page)
                                        .state(octocrab::params::State::Open) // Get only open issues
                                        .milestone(old_milestone.number as u64)
                                        .send()
                                        .await?;

                                    let items = resp.items;
                                    repo_issues.extend(items);

                                    match resp.next {
                                        Some(_url) => page += 1,
                                        None => break,
                                    }
                                }
                            }
                        }
                    }
                    None => todo!(),
                }
            }

            match current_milestones_map.get(&want_milestone.title) {
                None => {
                    updates.push(Update {
                        org: repo.org.clone(),
                        repo: repo.repo.clone(),
                        why: Why::Missing(want_milestone.clone()),
                        issues: repo_issues.clone(),
                    });
                }
                Some(existing_milestone) => {
                    let existing_milestone_due = existing_milestone
                        .due_on
                        .map(|date| date.format("%Y-%m-%d").to_string());

                    if existing_milestone.description != want_milestone.description
                        || existing_milestone_due != want_milestone.due
                        || existing_milestone.state != want_milestone.state
                        || !repo_issues.is_empty()
                    {
                        updates.push(Update {
                            org: repo.org.clone(),
                            repo: repo.repo.clone(),
                            why: Why::Changed(
                                want_milestone.clone(),
                                existing_milestone.number,
                                Milestone {
                                    title: existing_milestone.title.clone(),
                                    description: existing_milestone.description.clone(),
                                    state: existing_milestone.state.clone(),
                                    due: existing_milestone_due.clone(),
                                    replaces: None,
                                },
                            ),
                            issues: repo_issues,
                        });
                    };
                }
            };
        }
    }

    if updates.is_empty() {
        ActionCommand::Notice("Yay, there are no changes to be made".to_string()).send_command();
        return Ok(());
    }

    let data = serde_yml::to_string(&updates).unwrap();
    ActionCommand::Notice("Changes will be made".to_string()).send_command();
    println!("{}", data);

    if !cli.confirm {
        ActionCommand::Notice("Running without confirm, no mutations will be made".to_string())
            .send_command();
        return Ok(());
    }

    // https://docs.github.com/en/rest/issues/milestones?apiVersion=2022-11-28
    #[derive(Serialize, Debug)]
    struct MilestonePayload {
        title: String,

        #[serde(skip_serializing_if = "Option::is_none")]
        description: Option<String>,

        #[serde(skip_serializing_if = "Option::is_none")]
        state: Option<String>,

        #[serde(skip_serializing_if = "Option::is_none")]
        due_on: Option<String>,
    }

    for update in &updates {
        let milestone: octocrab::models::Milestone = match &update.why {
            Why::Missing(wanted_milestone) => {
                let payload = serde_json::to_value(MilestonePayload {
                    title: wanted_milestone.title.clone(),
                    description: wanted_milestone.description.clone(),
                    state: wanted_milestone.state.clone(),
                    due_on: wanted_milestone.due.clone(),
                })?;

                let resp: octocrab::models::Milestone = client
                    .post(
                        format!("/repos/{}/{}/milestones", &update.org, &update.repo),
                        Some(&payload),
                    )
                    .await?;
                println!("Milestone created: {:?}", resp);
                resp
            }
            Why::Changed(wanted_milestone, current_number, _current_milestone) => {
                let payload = serde_json::to_value(MilestonePayload {
                    title: wanted_milestone.title.clone(),
                    description: wanted_milestone.description.clone(),
                    state: wanted_milestone.state.clone(),
                    due_on: wanted_milestone.due.clone(),
                })?;

                let resp: octocrab::models::Milestone = client
                    .patch(
                        &format!(
                            "/repos/{}/{}/milestones/{}",
                            &update.org, &update.repo, current_number
                        ),
                        Some(&payload),
                    )
                    .await?;
                println!("Milestone updated: {:?}", resp);
                resp
            }
        };

        for issue in &update.issues {
            let _resp: Value = client
                .patch(
                    &format!(
                        "/repos/{}/{}/issues/{}",
                        &update.org, &update.repo, &issue.number
                    ),
                    Some(&json!({
                        "milestone": milestone.number,
                    })),
                )
                .await?;
            println!(
                "Issue added to milestone org: {:?} repo: {:?} issue: {:?} milestone: {:?}",
                update.org, update.repo, issue.number, milestone.number,
            );
        }
    }

    ActionCommand::Notice("Yay".to_string()).send_command();

    Ok(())
}
