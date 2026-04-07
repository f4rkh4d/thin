//! preset profiles + toml config loader.

use crate::rules::Severity;
use serde::Deserialize;
use std::collections::HashMap;
use std::path::Path;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Preset {
    Frkhd,
    Balanced,
    Relaxed,
    Corporate,
}

impl Preset {
    pub fn parse(s: &str) -> Option<Preset> {
        match s {
            "frkhd" => Some(Preset::Frkhd),
            "balanced" => Some(Preset::Balanced),
            "relaxed" => Some(Preset::Relaxed),
            "corporate" => Some(Preset::Corporate),
            _ => None,
        }
    }
    pub fn as_str(&self) -> &'static str {
        match self {
            Preset::Frkhd => "frkhd",
            Preset::Balanced => "balanced",
            Preset::Relaxed => "relaxed",
            Preset::Corporate => "corporate",
        }
    }
}

/// per-rule override: either a severity, or "off".
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum RuleOverride {
    Off,
    Severity(Severity),
}

#[derive(Debug, Clone, Default)]
pub struct Config {
    pub preset: Option<Preset>,
    pub rule_overrides: HashMap<String, RuleOverride>,
    pub ignore_paths: Vec<String>,
    pub ignore_patterns: Vec<String>,
}

#[derive(Debug, Deserialize)]
struct TomlRoot {
    profile: Option<ProfileSec>,
    rules: Option<HashMap<String, toml::Value>>,
    ignore: Option<IgnoreSec>,
}

#[derive(Debug, Deserialize)]
struct ProfileSec {
    name: Option<String>,
}

#[derive(Debug, Deserialize)]
struct IgnoreSec {
    paths: Option<Vec<String>>,
    patterns: Option<Vec<String>>,
}

impl Config {
    pub fn from_file(path: &Path) -> std::io::Result<Config> {
        let text = std::fs::read_to_string(path)?;
        let root: TomlRoot = toml::from_str(&text)
            .map_err(|e| std::io::Error::new(std::io::ErrorKind::InvalidData, e.to_string()))?;
        let preset = root
            .profile
            .and_then(|p| p.name)
            .and_then(|n| Preset::parse(&n));
        let mut rule_overrides = HashMap::new();
        if let Some(map) = root.rules {
            for (k, v) in map {
                match &v {
                    toml::Value::String(s) if s == "off" => {
                        rule_overrides.insert(k, RuleOverride::Off);
                    }
                    toml::Value::Table(t) => {
                        if let Some(toml::Value::String(sv)) = t.get("severity") {
                            if let Some(sev) = parse_severity(sv) {
                                rule_overrides.insert(k, RuleOverride::Severity(sev));
                            }
                        }
                    }
                    _ => {}
                }
            }
        }
        let (ignore_paths, ignore_patterns) = root
            .ignore
            .map(|i| (i.paths.unwrap_or_default(), i.patterns.unwrap_or_default()))
            .unwrap_or_default();
        Ok(Config {
            preset,
            rule_overrides,
            ignore_paths,
            ignore_patterns,
        })
    }
}

fn parse_severity(s: &str) -> Option<Severity> {
    match s {
        "error" => Some(Severity::Error),
        "warning" => Some(Severity::Warning),
        "info" => Some(Severity::Info),
        _ => None,
    }
}

/// decide the effective severity for a rule given a preset + config.
/// returns None to suppress the rule entirely.
pub fn effective_severity(
    rule_id: &str,
    rule_category: crate::rules::Category,
    default: Severity,
    preset: Preset,
    overrides: &HashMap<String, RuleOverride>,
) -> Option<Severity> {
    // inline override wins.
    if let Some(o) = overrides.get(rule_id) {
        return match o {
            RuleOverride::Off => None,
            RuleOverride::Severity(s) => Some(*s),
        };
    }
    use crate::rules::Category;
    match preset {
        Preset::Frkhd => Some(Severity::Error),
        Preset::Balanced => Some(default),
        Preset::Relaxed => match rule_category {
            Category::AiPhrase | Category::Parallel | Category::Punctuation => Some(default),
            _ => None,
        },
        Preset::Corporate => match rule_category {
            Category::AiPhrase | Category::Punctuation => Some(default),
            _ => None,
        },
    }
}
