use thin::rules::registry;
use thin::tokenize::paragraphs;

fn run_rule(id: &str, src: &str) -> usize {
    let reg = registry();
    let r = reg.iter().find(|r| r.id() == id).expect("rule not found");
    let ps = paragraphs(src);
    r.scan(src, &ps).len()
}

#[test]
fn flags_delve() {
    assert_eq!(run_rule("thin.ai-phrase.delve", "let's delve into sql."), 1);
}

#[test]
fn flags_important_to_note() {
    assert_eq!(
        run_rule(
            "thin.ai-phrase.important-to-note",
            "it's important to note that x."
        ),
        1
    );
}

#[test]
fn flags_in_conclusion() {
    assert_eq!(
        run_rule("thin.ai-phrase.in-conclusion", "in conclusion, we win."),
        1
    );
}

#[test]
fn flags_testament_to() {
    assert_eq!(
        run_rule("thin.ai-phrase.testament-to", "a testament to design."),
        1
    );
}

#[test]
fn flags_navigate_complexities() {
    assert_eq!(
        run_rule(
            "thin.ai-phrase.navigate-complexities",
            "navigate the complexities of oauth."
        ),
        1
    );
}

#[test]
fn flags_ever_evolving_landscape() {
    assert_eq!(
        run_rule(
            "thin.ai-phrase.ever-evolving-landscape",
            "in the ever-evolving landscape of ml."
        ),
        1
    );
}
