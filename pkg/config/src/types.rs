use serde::{Deserialize, Serialize};

// Configuration is a representation of the repositories we will manage
// + their Labels
// + their Milestons
#[derive(Deserialize)]
pub struct Configuration {
    pub repos: Vec<Repo>,
    pub labels: Vec<Label>,
    pub milestones: Vec<Milestone>,
}

// Repo represents the "coordinates" to a repository
#[derive(Deserialize)]
pub struct Repo {
    pub org: String,
    pub repo: String,
}

// Label holds declarative data about the label.
#[derive(Deserialize, Serialize, Clone)]
pub struct Label {
    // Name is the current name of the label
    pub name: String,

    // Color is rrggbb or color
    pub color: String,

    // Description is brief text explaining its meaning, who can apply it
    pub description: Option<String>,
}

// Milestone holds declarative data about the milestone.
#[derive(Clone, Serialize, Deserialize)]
pub struct Milestone {
    pub title: String,
    pub description: Option<String>,
    pub state: Option<String>,
    pub due: Option<String>,
    pub replaces: Option<String>,
}

impl std::fmt::Display for Milestone {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "title: {:?}, description: {:?}, state: {:?}, due: {:?}, replaces: {:?}",
            self.title, self.description, self.state, self.due, self.replaces
        )
    }
}
