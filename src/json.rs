//! json format for ci.

use crate::rules::Finding;
use serde::Serialize;

#[derive(Serialize)]
pub struct FileReport<'a> {
    pub path: &'a str,
    pub findings: &'a [Finding],
    pub counts: Counts,
}

#[derive(Serialize)]
pub struct Counts {
    pub total: usize,
    pub errors: usize,
    pub warnings: usize,
    pub info: usize,
}

pub fn counts(findings: &[Finding]) -> Counts {
    use crate::rules::Severity;
    let errors = findings
        .iter()
        .filter(|f| f.severity == Severity::Error)
        .count();
    let warnings = findings
        .iter()
        .filter(|f| f.severity == Severity::Warning)
        .count();
    let info = findings
        .iter()
        .filter(|f| f.severity == Severity::Info)
        .count();
    Counts {
        total: findings.len(),
        errors,
        warnings,
        info,
    }
}
