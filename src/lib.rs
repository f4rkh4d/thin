//! thin — rule-based prose linter.

pub mod cli;
pub mod fix;
pub mod json;
pub mod profile;
pub mod render;
pub mod rules;
pub mod tokenize;

pub use rules::{registry, Category, Finding, Rule, Severity};
