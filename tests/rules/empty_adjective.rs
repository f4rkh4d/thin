use thin::rules::registry;
use thin::tokenize::paragraphs;

fn run_rule(id: &str, src: &str) -> usize {
    let reg = registry();
    let r = reg.iter().find(|r| r.id() == id).expect("rule not found");
    let ps = paragraphs(src);
    r.scan(src, &ps).len()
}

#[test]
fn flags_seamless() {
    assert_eq!(
        run_rule("thin.empty-adjective.seamless", "a seamless flow."),
        1
    );
}

#[test]
fn flags_blazingly_fast() {
    assert_eq!(
        run_rule(
            "thin.empty-adjective.blazingly-fast",
            "blazingly fast parser."
        ),
        1
    );
}

#[test]
fn flags_revolutionary_case_insensitive() {
    assert_eq!(
        run_rule(
            "thin.empty-adjective.revolutionary",
            "a Revolutionary approach."
        ),
        1
    );
}

#[test]
fn respects_word_boundary() {
    // "seamlessly" should not match "seamless"
    assert_eq!(
        run_rule("thin.empty-adjective.seamless", "seamlessly done."),
        0
    );
}

#[test]
fn flags_game_changing() {
    assert_eq!(
        run_rule(
            "thin.empty-adjective.game-changing",
            "a game-changing idea."
        ),
        1
    );
}

#[test]
fn flags_cutting_edge() {
    assert_eq!(
        run_rule("thin.empty-adjective.cutting-edge", "cutting-edge stack."),
        1
    );
}
