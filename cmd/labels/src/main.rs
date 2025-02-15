use std::{
    collections::HashMap,
    process::{ExitCode, Termination},
};

use action::{client, commands::ActionCommand};
use clap::Parser;
use config::types::Label;
use serde::Serialize;
use serde_json::{json, Value};

mod error;

#[derive(Clone, Serialize)]
enum Why {
    // Wanted
    Missing(Label),
    // Wanted, Current
    Changed(Label, Label),
}

#[derive(Clone, Serialize)]
struct Update {
    org: String,
    repo: String,
    why: Why,
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
    let per_page = 100;

    let mut updates: Vec<Update> = vec![];
    for repo in &configuration.repos {
        let mut current_labels: Vec<octocrab::models::Label> = vec![];

        let mut page: u32 = 1;
        loop {
            let resp = client
                .issues(&repo.org, &repo.repo)
                .list_labels_for_repo()
                .page(page)
                .per_page(per_page)
                .send()
                .await?;

            let items = resp.items;
            current_labels.extend(items);

            match resp.next {
                Some(_url) => page += 1,
                None => break,
            }
        }

        let current_labes_map: HashMap<String, octocrab::models::Label> = current_labels
            .into_iter()
            .map(|l| (l.name.clone(), l.clone()))
            .collect();

        // Compare labels
        for label in &configuration.labels {
            match current_labes_map.get(&label.name) {
                None => {
                    updates.push(Update {
                        org: repo.org.clone(),
                        repo: repo.repo.clone(),
                        why: Why::Missing(label.clone()),
                    });
                }
                Some(existing_label) => {
                    let empty_string = "".to_string();
                    if existing_label.color.to_lowercase() != label.color.to_lowercase()
                        || existing_label.description.as_ref().unwrap_or(&empty_string)
                            != label.description.as_deref().unwrap_or(&empty_string)
                    {
                        updates.push(Update {
                            org: repo.org.clone(),
                            repo: repo.repo.clone(),
                            why: Why::Changed(
                                label.clone(),
                                Label {
                                    name: existing_label.name.clone(),
                                    color: existing_label.color.clone(),
                                    description: existing_label.description.clone(),
                                },
                            ),
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
    println!("{}", data);

    if !cli.confirm {
        ActionCommand::Notice("Running without confirm, no mutations will be made".to_string())
            .send_command();
        return Ok(());
    }

    for update in &updates {
        match &update.why {
            Why::Missing(wanted) => {
                let resp = client
                    .issues(&update.org, &update.repo)
                    .create_label(
                        &wanted.name,
                        &wanted.color,
                        wanted.description.clone().unwrap_or("".to_string()),
                    )
                    .await?;
                println!("Label created: {:?}", resp);
            }
            Why::Changed(wanted, current) => {
                let resp: Value = client
                    .patch(
                        &format!(
                            "/repos/{}/{}/labels/{}",
                            &update.org, &update.repo, &current.name
                        ),
                        Some(&json!({
                            "name": wanted.name,
                            "color": wanted.color,
                            "description": wanted.description,
                        })),
                    )
                    .await?;
                println!("Label updated: {:?}", resp);
            }
        };
    }

    ActionCommand::Notice("Yay".to_string()).send_command();

    Ok(())
}
