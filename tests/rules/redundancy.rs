use thin::rules::registry;
use thin::tokenize::paragraphs;

fn run_rule(id: &str, src: &str) -> usize {
    let reg = registry();
    let r = reg.iter().find(|r| r.id() == id).expect("rule not found");
    let ps = paragraphs(src);
    r.scan(src, &ps).len()
}

#[test]
fn flags_end_result() {
    assert_eq!(run_rule("thin.redundancy.end-result", "the end result."), 1);
}

#[test]
fn flags_in_order_to() {
    assert_eq!(
        run_rule("thin.redundancy.in-order-to", "in order to run."),
        1
    );
}

#[test]
fn flags_each_and_every() {
    assert_eq!(
        run_rule("thin.redundancy.each-and-every", "each and every user."),
        1
    );
}

#[test]
fn fix_is_present() {
    let reg = thin::rules::registry();
    let r = reg
        .iter()
        .find(|r| r.id() == "thin.redundancy.end-result")
        .unwrap();
    let src = "the end result.";
    let ps = paragraphs(src);
    let f = &r.scan(src, &ps)[0];
    assert_eq!(f.fix.as_deref(), Some("result"));
}
