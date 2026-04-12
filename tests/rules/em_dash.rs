use thin::rules::{em_dash::EmDashCluster, Rule};
use thin::tokenize::paragraphs;

#[test]
fn flags_three_em_dashes_in_one_paragraph() {
    let src = "this — is — a — cluster.";
    let ps = paragraphs(src);
    let r = EmDashCluster;
    let out = r.scan(src, &ps);
    assert_eq!(out.len(), 1);
    assert_eq!(out[0].rule_id, "thin.em-dash.cluster");
}

#[test]
fn skips_two_em_dashes() {
    let src = "this — is — fine.";
    let ps = paragraphs(src);
    let r = EmDashCluster;
    assert!(r.scan(src, &ps).is_empty());
}

#[test]
fn counts_per_paragraph() {
    let src = "this — is — only — two plus one.\n\nand this — one — has — three.";
    let ps = paragraphs(src);
    let r = EmDashCluster;
    let out = r.scan(src, &ps);
    assert_eq!(out.len(), 2);
}

#[test]
fn message_is_witty() {
    let src = "a — b — c — d";
    let ps = paragraphs(src);
    let r = EmDashCluster;
    let out = r.scan(src, &ps);
    let msg = &out[0].message;
    assert!(
        msg.contains("em-dash")
            || msg.contains("comma")
            || msg.contains("lane")
            || msg.contains("breathe")
    );
}
