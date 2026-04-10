//! safe auto-replacement engine.
//!
//! only apply fixes where the rule provides a deterministic replacement.
//! case-preserving: if the matched text starts with uppercase, capitalize the fix.

use crate::rules::Finding;

/// apply all non-overlapping fixes to `src`. returns (patched, count_applied).
pub fn apply_fixes(src: &str, findings: &[Finding]) -> (String, usize) {
    // pick only findings with a fix, and de-overlap by preferring earlier starts.
    let mut fixes: Vec<&Finding> = findings.iter().filter(|f| f.fix.is_some()).collect();
    fixes.sort_by_key(|f| f.start);
    let mut result = String::with_capacity(src.len());
    let mut cursor = 0;
    let mut applied = 0;
    for f in fixes {
        if f.start < cursor {
            continue; // overlapping, skip
        }
        result.push_str(&src[cursor..f.start]);
        let original = &src[f.start..f.end];
        let replacement = f.fix.clone().unwrap();
        result.push_str(&case_match(original, &replacement));
        cursor = f.end;
        applied += 1;
    }
    result.push_str(&src[cursor..]);
    (result, applied)
}

fn case_match(original: &str, replacement: &str) -> String {
    if let Some(first) = original.chars().next() {
        if first.is_uppercase() {
            let mut out = String::new();
            let mut chars = replacement.chars();
            if let Some(c) = chars.next() {
                out.extend(c.to_uppercase());
            }
            out.extend(chars);
            return out;
        }
    }
    replacement.to_string()
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::rules::{Category, Finding, Severity};

    #[test]
    fn preserves_case() {
        assert_eq!(case_match("Utilizes", "uses"), "Uses");
        assert_eq!(case_match("utilizes", "uses"), "uses");
    }

    #[test]
    fn applies_non_overlapping() {
        let src = "we utilizes in order to run.";
        let findings = vec![
            Finding {
                rule_id: "a",
                category: Category::Filler,
                severity: Severity::Error,
                message: "m".into(),
                start: 3,
                end: 11,
                line: 1,
                col: 4,
                snippet: "utilizes".into(),
                fix: Some("uses".into()),
            },
            Finding {
                rule_id: "b",
                category: Category::Redundancy,
                severity: Severity::Warning,
                message: "m".into(),
                start: 12,
                end: 23,
                line: 1,
                col: 13,
                snippet: "in order to".into(),
                fix: Some("to".into()),
            },
        ];
        let (out, n) = apply_fixes(src, &findings);
        assert_eq!(n, 2);
        assert_eq!(out, "we uses to run.");
    }
}
