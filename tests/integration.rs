use std::fs;
use thin::rules::{registry, Severity};
use thin::tokenize::paragraphs;

fn scan_file(path: &str) -> Vec<thin::rules::Finding> {
    let src = fs::read_to_string(path).expect("read fixture");
    let ps = paragraphs(&src);
    let reg = registry();
    let mut all = Vec::new();
    for r in &reg {
        all.extend(r.scan(&src, &ps));
    }
    all
}

#[test]
fn ai_readme_lights_up() {
    let findings = scan_file("tests/fixtures/ai-generated-readme.md");
    assert!(
        findings.len() >= 15,
        "expected many findings in ai fixture, got {}",
        findings.len()
    );
    let err_count = findings
        .iter()
        .filter(|f| f.severity == Severity::Error)
        .count();
    assert!(err_count >= 5, "expected errors, got {}", err_count);
}

#[test]
fn clean_readme_is_quiet() {
    let findings = scan_file("tests/fixtures/clean-readme.md");
    assert!(
        findings.len() <= 2,
        "clean readme should be quiet, got {} findings: {:?}",
        findings.len(),
        findings.iter().map(|f| f.rule_id).collect::<Vec<_>>()
    );
}

#[test]
fn corporate_fixture_catches_phrases() {
    let findings = scan_file("tests/fixtures/corporate.md");
    let ids: Vec<&str> = findings.iter().map(|f| f.rule_id).collect();
    assert!(ids.contains(&"thin.corporate.synergy"));
    assert!(ids.contains(&"thin.corporate.mission-critical"));
}

#[test]
fn rule_count_at_least_thirty() {
    assert!(registry().len() >= 30, "need 30+ rules");
}

#[test]
fn all_rules_have_unique_ids() {
    let reg = registry();
    let ids: std::collections::HashSet<&str> = reg.iter().map(|r| r.id()).collect();
    assert_eq!(ids.len(), reg.len(), "duplicate rule ids");
}

#[test]
fn all_rules_have_nonempty_messages_after_scan() {
    // feed each rule a string containing its own name — some may not match,
    // but we verify shape by calling on the ai-fixture.
    let findings = scan_file("tests/fixtures/ai-generated-readme.md");
    for f in findings {
        assert!(!f.message.is_empty(), "empty message for {}", f.rule_id);
        assert!(f.message.len() < 80, "message too long: {}", f.message);
    }
}

#[test]
fn em_dash_message_appears() {
    // use raw em-dashes to trigger cluster.
    let src = "one — two — three — four.";
    let ps = paragraphs(src);
    let reg = registry();
    let em = reg
        .iter()
        .find(|r| r.id() == "thin.em-dash.cluster")
        .unwrap();
    let out = em.scan(src, &ps);
    assert_eq!(out.len(), 1);
    let m = &out[0].message;
    assert!(["commas", "lane", "breathe"].iter().any(|s| m.contains(s)));
}
