use thin::rules::registry;
use thin::tokenize::paragraphs;

fn run_rule(id: &str, src: &str) -> usize {
    let reg = registry();
    let r = reg.iter().find(|r| r.id() == id).expect("rule not found");
    let ps = paragraphs(src);
    r.scan(src, &ps).len()
}

#[test]
fn flags_leverages() {
    assert_eq!(run_rule("thin.filler.leverages", "leverages rust."), 1);
}

#[test]
fn flags_utilizes() {
    assert_eq!(run_rule("thin.filler.utilizes", "utilizes a cache."), 1);
}

#[test]
fn leverages_has_fix() {
    let reg = thin::rules::registry();
    let r = reg
        .iter()
        .find(|r| r.id() == "thin.filler.leverages")
        .unwrap();
    let src = "leverages rust.";
    let ps = paragraphs(src);
    assert_eq!(r.scan(src, &ps)[0].fix.as_deref(), Some("uses"));
}
