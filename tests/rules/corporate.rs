use thin::rules::registry;
use thin::tokenize::paragraphs;

fn run_rule(id: &str, src: &str) -> usize {
    let reg = registry();
    let r = reg.iter().find(|r| r.id() == id).expect("rule not found");
    let ps = paragraphs(src);
    r.scan(src, &ps).len()
}

#[test]
fn flags_synergy() {
    assert_eq!(run_rule("thin.corporate.synergy", "team synergy."), 1);
}

#[test]
fn flags_circle_back() {
    assert_eq!(
        run_rule("thin.corporate.circle-back", "let's circle back."),
        1
    );
}

#[test]
fn flags_mission_critical() {
    assert_eq!(
        run_rule("thin.corporate.mission-critical", "mission-critical."),
        1
    );
}
