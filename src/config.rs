use anyhow::{Context, Result};
use serde::{Deserialize, Serialize};
use std::path::Path;

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(deny_unknown_fields, default)]
pub struct Config {
    pub target_repo: String,
    pub excluded_authors: Vec<String>,
    /// Labels that cause a PR to be skipped entirely (not counted at all).
    pub excluded_labels: Vec<String>,
    /// Labels that mark a PR as "deferred" — still visible in the scoreboard
    /// (in its own bucket), but not counted as dirty and never tagged
    /// `needs-author-action`.
    pub deferred_labels: Vec<String>,
    /// The repo's default branch. PRs targeting anything else are bucketed as Stale.
    /// `None` (the default) means auto-detect from GitHub on each run.
    #[serde(default)]
    pub default_target_branch: Option<String>,
    /// PRs whose `updatedAt` is older than this fall into the Stale bucket —
    /// unless they would otherwise be Clean or Deferred.
    pub stale_threshold_days: i64,
    /// CODEOWNERS-lite: route review duty by changed file paths.
    pub review_routing: Vec<RoutingRule>,
    pub grace_period_days: i64,
    pub count_nitpicks: bool,
    pub maintainer_only: bool,
    pub maintainers: Vec<String>,
    pub auto_label: bool,
    pub weights: Weights,
    pub age_multiplier: AgeMultiplier,
    pub history_retention_days: i64,
    pub needs_author_action_label: String,
    /// Merge one author's row into another's in the scoreboard. Map keys are
    /// "alias" logins (e.g. AI-assistant accounts); values are the human "principal"
    /// who's actually accountable. The principal's row is displayed as
    /// "@principal + (@alias)" with cells like "X+(Y)". Aliases also count as
    /// the principal for self-review detection.
    #[serde(default)]
    pub author_aliases: std::collections::HashMap<String, String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct RoutingRule {
    /// Glob patterns matched against each changed file path. If any path matches,
    /// the rule fires. Patterns follow `glob::Pattern` syntax (`**/sdk/**`, etc.).
    pub paths: Vec<String>,
    /// GitHub login of the person who normally owns this area.
    pub primary: String,
    /// Used when `primary` is the PR author. Optional — if unset, the rule
    /// silently doesn't fire for self-authored PRs.
    #[serde(default)]
    pub fallback: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(deny_unknown_fields, default)]
pub struct Weights {
    pub high: f64,
    pub medium: f64,
    pub low: f64,
}

#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "lowercase")]
pub enum AgeMultiplier {
    Ln,
    Log10,
    None,
}

impl Default for Config {
    fn default() -> Self {
        Self {
            target_repo: "dashpay/platform".into(),
            excluded_authors: vec![
                "dependabot[bot]".into(),
                "renovate[bot]".into(),
                "github-actions[bot]".into(),
            ],
            excluded_labels: vec![
                "wip".into(),
                "blocked".into(),
                "do-not-merge".into(),
                "help-wanted".into(),
            ],
            deferred_labels: vec!["postponed".into()],
            default_target_branch: None,
            stale_threshold_days: 120,
            review_routing: vec![
                RoutingRule {
                    paths: vec![
                        "**/sdk/**".into(),
                        "**/wasm/**".into(),
                        "**/wasm-sdk/**".into(),
                    ],
                    primary: "shumkov".into(),
                    fallback: Some("QuantumExplorer".into()),
                },
                RoutingRule {
                    paths: vec!["**/swift-sdk/**".into(), "**/ios/**".into()],
                    primary: "llbartekll".into(),
                    fallback: Some("ZocoLini".into()),
                },
            ],
            grace_period_days: 14,
            count_nitpicks: false,
            maintainer_only: false,
            maintainers: vec![],
            auto_label: true,
            weights: Weights::default(),
            age_multiplier: AgeMultiplier::Ln,
            history_retention_days: 90,
            needs_author_action_label: "needs-author-action".into(),
            author_aliases: {
                let mut m = std::collections::HashMap::new();
                m.insert("Claudius-Maginificent".into(), "lklimek".into());
                m.insert("thepastaclaw".into(), "PastaPastaPasta".into());
                m
            },
        }
    }
}

impl Default for Weights {
    fn default() -> Self {
        Self {
            high: 5.0,
            medium: 2.0,
            low: 0.5,
        }
    }
}

impl Config {
    /// Load from `path` if it exists, otherwise return defaults.
    pub fn load_or_default(path: &Path) -> Result<Self> {
        if !path.exists() {
            tracing::info!("no config file at {}, using defaults", path.display());
            return Ok(Self::default());
        }
        let text = std::fs::read_to_string(path)
            .with_context(|| format!("reading config file {}", path.display()))?;
        let cfg: Self = serde_yaml::from_str(&text)
            .with_context(|| format!("parsing YAML at {}", path.display()))?;
        Ok(cfg)
    }

    /// Split "owner/name" into parts.
    pub fn repo_parts(&self) -> Result<(&str, &str)> {
        let (owner, name) = self.target_repo.split_once('/').with_context(|| {
            format!(
                "target_repo {:?} must be in 'owner/name' form",
                self.target_repo
            )
        })?;
        if owner.is_empty() || name.is_empty() {
            anyhow::bail!("target_repo {:?} has empty owner or name", self.target_repo);
        }
        Ok((owner, name))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn defaults_are_sensible() {
        let c = Config::default();
        assert_eq!(c.target_repo, "dashpay/platform");
        assert!(c.auto_label);
        assert!(!c.count_nitpicks);
        assert_eq!(c.weights.high, 5.0);
        assert_eq!(c.age_multiplier, AgeMultiplier::Ln);
        assert_eq!(c.history_retention_days, 90);
    }

    #[test]
    fn empty_yaml_yields_defaults() {
        let c: Config = serde_yaml::from_str("{}").unwrap();
        assert_eq!(c.target_repo, "dashpay/platform");
    }

    #[test]
    fn partial_yaml_overrides_only_listed_fields() {
        let yaml = r#"
target_repo: foo/bar
count_nitpicks: true
weights:
  high: 10.0
"#;
        let c: Config = serde_yaml::from_str(yaml).unwrap();
        assert_eq!(c.target_repo, "foo/bar");
        assert!(c.count_nitpicks);
        assert_eq!(c.weights.high, 10.0);
        // Untouched fields keep defaults.
        assert_eq!(c.weights.medium, 2.0);
        assert!(c.auto_label);
    }

    #[test]
    fn unknown_field_is_rejected() {
        let yaml = "bogus: 1\n";
        let err = serde_yaml::from_str::<Config>(yaml).unwrap_err();
        assert!(err.to_string().contains("bogus"), "got: {err}");
    }

    #[test]
    fn repo_parts_splits_owner_name() {
        let c = Config::default();
        let (o, n) = c.repo_parts().unwrap();
        assert_eq!(o, "dashpay");
        assert_eq!(n, "platform");
    }

    #[test]
    fn repo_parts_rejects_malformed() {
        let mut c = Config {
            target_repo: "no-slash".into(),
            ..Config::default()
        };
        assert!(c.repo_parts().is_err());

        c.target_repo = "/empty".into();
        assert!(c.repo_parts().is_err());
    }
}
