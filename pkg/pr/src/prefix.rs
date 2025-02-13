// Motivated by, and largely copied from,
// https://github.com/kubernetes-sigs/kubebuilder-release-tools

use core::fmt;
use regex::Regex;

const PREFIX_FEATURE: (&str, &str) = (":sparkles:", "✨");
const PREFIX_BUG_FIX: (&str, &str) = (":bug:", "🐛");
const PREFIX_DOCS: (&str, &str) = (":book:", "📖");
const PREFIX_INFRA: (&str, &str) = (":seedling:", "🌱");
const PREFIX_BREAKING: (&str, &str) = (":warning:", "⚠");
const PREFIX_NO_NOTE: (&str, &str) = (":ghost:", "👻");

// Title, Emoji
pub type PRTypeError = (String, Option<String>);

#[derive(Debug, PartialEq)]
pub enum PRType {
    Feature(String),
    BugFix(String),
    Docs(String),
    Infra(String),
    Breaking(String),
    NoNote(String),
}

impl fmt::Display for PRType {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Self::Feature(title) => write!(f, "PR type 'Feature'\n PR title '{}'", title),
            Self::BugFix(title) => write!(f, "PR type 'Bug'\n PR title '{}'", title),
            Self::Docs(title) => write!(f, "PR type 'Docs'\n PR title '{}'", title),
            Self::Infra(title) => write!(f, "PR type 'Infra'\n PR title '{}'", title),
            Self::Breaking(title) => write!(f, "PR type 'Breaking'\n PR title '{}'", title),
            Self::NoNote(title) => write!(f, "PR type 'NoNote'\n PR title '{}'", title),
        }
    }
}

impl PRType {
    pub fn from_title(value: &str) -> Result<Self, PRTypeError> {
        let wip_regex = Regex::new(r"(?i)^\W?WIP\W").unwrap();
        let tag_regex = Regex::new(r"^\[[\w.-]*]").unwrap();

        // Remove the WIP prefix if found.
        let value = wip_regex.replace_all(value, "");

        // Trim to remove spaces after WIP.
        let value = value.trim();

        // Remove a tag prefix if found.
        let value = tag_regex.replace_all(value, "");
        let value = value.trim();

        if value.is_empty() {
            return Err((value.to_string(), None));
        }

        // Trusting those that came before...
        // https://github.com/kubernetes-sigs/kubebuilder-release-tools/blob/4f3d1085b4458a49ed86918b4b55505716715b77/notes/common/prefix.go#L123-L125
        // strip the variation selector from the title, if present
        // (some systems sneak it in -- my guess is OSX)
        fn trust(title: &str) -> String {
            let result = if title.starts_with('\u{FE0F}') {
                let result: String = title.chars().skip(1).collect();
                result
            } else {
                title.to_string()
            };
            result.trim().to_string()
        }

        if let Some(title) = value.strip_prefix(PREFIX_FEATURE.0) {
            Ok(PRType::Feature(trust(title)))
        } else if let Some(title) = value.strip_prefix(PREFIX_BUG_FIX.0) {
            Ok(PRType::BugFix(trust(title)))
        } else if let Some(title) = value.strip_prefix(PREFIX_DOCS.0) {
            Ok(PRType::Docs(trust(title)))
        } else if let Some(title) = value.strip_prefix(PREFIX_INFRA.0) {
            Ok(PRType::Infra(trust(title)))
        } else if let Some(title) = value.strip_prefix(PREFIX_BREAKING.0) {
            Ok(PRType::Breaking(trust(title)))
        } else if let Some(title) = value.strip_prefix(PREFIX_NO_NOTE.0) {
            Ok(PRType::NoNote(trust(title)))
        } else if value.strip_prefix(PREFIX_FEATURE.1).is_some()
            || value.strip_prefix(PREFIX_BUG_FIX.1).is_some()
            || value.strip_prefix(PREFIX_DOCS.1).is_some()
            || value.strip_prefix(PREFIX_INFRA.1).is_some()
            || value.strip_prefix(PREFIX_BREAKING.1).is_some()
            || value.strip_prefix(PREFIX_NO_NOTE.1).is_some()
        {
            let emoji = value.chars().next().map(|c| c.to_string());
            return Err((trust(value), emoji));
        } else {
            return Err((trust(value), None));
        }
    }

    pub fn title(&self) -> String {
        match self {
            PRType::Feature(title) => title.to_string(),
            PRType::BugFix(title) => title.to_string(),
            PRType::Docs(title) => title.to_string(),
            PRType::Infra(title) => title.to_string(),
            PRType::Breaking(title) => title.to_string(),
            PRType::NoNote(title) => title.to_string(),
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::prefix::{PRType, PRTypeError};

    struct TestCase {
        pub title: &'static str,
        pub expected_result: Result<PRType, PRTypeError>,
    }

    #[test]
    fn title_cases() {
        let test_cases = vec![
            TestCase {
                title: "WIP: [docs] Update documentation",
                expected_result: Err(("Update documentation".to_string(), None)),
            },
            TestCase {
                title: "WIP: :sparkles: Add new feature",
                expected_result: Ok(PRType::Feature("Add new feature".to_string())),
            },
            TestCase {
                title: "WIP: :warning: Breaking change",
                expected_result: Ok(PRType::Breaking("Breaking change".to_string())),
            },
            TestCase {
                title: "WIP: :bug: Fix bug",
                expected_result: Ok(PRType::BugFix("Fix bug".to_string())),
            },
            TestCase {
                title: ":ghost: Don't put me in release notes",
                expected_result: Ok(PRType::NoNote("Don't put me in release notes".to_string())),
            },
            TestCase {
                title: "WIP: :seedling: Infrastructure change",
                expected_result: Ok(PRType::Infra("Infrastructure change".to_string())),
            },
            TestCase {
                title: "WIP: No prefix in title",
                expected_result: Err(("No prefix in title".to_string(), None)),
            },
            TestCase {
                title: "No prefix in title",
                expected_result: Err(("No prefix in title".to_string(), None)),
            },
            TestCase {
                title: "WIP:",
                expected_result: Err(("".to_string(), None)),
            },
            TestCase {
                title: "",
                expected_result: Err(("".to_string(), None)),
            },
            TestCase {
                title: "WIP: [tag] :sparkles: Add new feature",
                expected_result: Ok(PRType::Feature("Add new feature".to_string())),
            },
            TestCase {
                title: "👻 I should have used the alias",
                expected_result: Err((
                    "👻 I should have used the alias".to_string(),
                    Some("👻".to_string()),
                )),
            },
        ];

        for tc in test_cases {
            let pr = PRType::from_title(tc.title);
            assert_eq!(tc.expected_result, pr);
        }
    }
}
