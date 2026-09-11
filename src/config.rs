use anyhow::{Context, Result};
use serde::{Deserialize, Serialize};
use std::path::Path;

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(deny_unknown_fields, default)]
pub struct Config {
    pub excluded_authors: Vec<String>,
    /// Labels that cause a PR to be skipped entirely (not counted at all).
    pub excluded_labels: Vec<String>,
    /// Labels that mark a PR as "deferred" — still visible in the scoreboard
    /// (in its own bucket), but not counted as dirty and never tagged
    /// `needs-author-action`.
    pub deferred_labels: Vec<String>,
    /// PRs whose `updatedAt` is older than this fall into the Stale bucket —
    /// unless they would otherwise be Clean or Deferred.
    pub stale_threshold_days: i64,
    pub grace_period_days: i64,
    pub count_nitpicks: bool,
    pub maintainer_only: bool,
    pub maintainers: Vec<String>,
    pub weights: Weights,
    pub age_multiplier: AgeMultiplier,
    pub history_retention_days: i64,
    /// Merge one author's row into another's in the scoreboard. Map keys are
    /// "alias" logins (e.g. AI-assistant accounts); values are the human "principal"
    /// who's actually accountable. The principal's row is displayed as
    /// "@principal + (@alias)" with cells like "X+(Y)". Aliases also count as
    /// the principal for self-review detection.
    #[serde(default)]
    pub author_aliases: std::collections::HashMap<String, String>,
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
            stale_threshold_days: 120,
            grace_period_days: 14,
            count_nitpicks: false,
            maintainer_only: false,
            maintainers: vec![],
            weights: Weights::default(),
            age_multiplier: AgeMultiplier::Ln,
            history_retention_days: 90,
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
}

/// Split "owner/name" into parts.
pub fn repo_parts(repo: &str) -> Result<(&str, &str)> {
    let (owner, name) = repo
        .split_once('/')
        .with_context(|| format!("repository {repo:?} must be in 'owner/name' form"))?;
    if owner.is_empty() || name.is_empty() {
        anyhow::bail!("repository {repo:?} has empty owner or name");
    }
    Ok((owner, name))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn defaults_are_sensible() {
        let c = Config::default();
        assert!(!c.count_nitpicks);
        assert_eq!(c.weights.high, 5.0);
        assert_eq!(c.age_multiplier, AgeMultiplier::Ln);
        assert_eq!(c.history_retention_days, 90);
    }

    #[test]
    fn empty_yaml_yields_defaults() {
        let c: Config = serde_yaml::from_str("{}").unwrap();
        assert_eq!(c.grace_period_days, 14);
    }

    #[test]
    fn partial_yaml_overrides_only_listed_fields() {
        let yaml = r#"
count_nitpicks: true
weights:
  high: 10.0
"#;
        let c: Config = serde_yaml::from_str(yaml).unwrap();
        assert!(c.count_nitpicks);
        assert_eq!(c.weights.high, 10.0);
        // Untouched fields keep defaults.
        assert_eq!(c.weights.medium, 2.0);
        assert!(!c.maintainer_only);
    }

    #[test]
    fn unknown_field_is_rejected() {
        let yaml = "bogus: 1\n";
        let err = serde_yaml::from_str::<Config>(yaml).unwrap_err();
        assert!(err.to_string().contains("bogus"), "got: {err}");
    }

    #[test]
    fn repo_parts_splits_owner_name() {
        let (o, n) = repo_parts("dashpay/platform").unwrap();
        assert_eq!(o, "dashpay");
        assert_eq!(n, "platform");
    }

    #[test]
    fn repo_parts_rejects_malformed() {
        assert!(repo_parts("no-slash").is_err());
        assert!(repo_parts("/empty").is_err());
        assert!(repo_parts("empty/").is_err());
    }

    #[test]
    fn removed_keys_are_rejected() {
        for key in ["target_repo", "review_routing", "auto_label"] {
            let err = serde_yaml::from_str::<Config>(&format!("{key}: x\n")).unwrap_err();
            assert!(err.to_string().contains(key), "got: {err}");
        }
    }
}
