use thin::rules::parallel::{ItsNotIts, NotItsIs, NotJustBut};
use thin::rules::Rule;
use thin::tokenize::paragraphs;

#[test]
fn flags_not_its_pattern() {
    let src = "this is not a database. it's a memory.";
    let ps = paragraphs(src);
    let r = NotItsIs;
    assert_eq!(r.scan(src, &ps).len(), 1);
}

#[test]
fn skips_lone_not() {
    let src = "this is not a database.";
    let ps = paragraphs(src);
    let r = NotItsIs;
    assert!(r.scan(src, &ps).is_empty());
}

#[test]
fn flags_not_just_but() {
    let src = "not just fast, but fast enough.";
    let ps = paragraphs(src);
    let r = NotJustBut;
    assert_eq!(r.scan(src, &ps).len(), 1);
}

#[test]
fn flags_its_not_its() {
    let src = "it's not a toy, it's a tool.";
    let ps = paragraphs(src);
    let r = ItsNotIts;
    assert_eq!(r.scan(src, &ps).len(), 1);
}

#[test]
fn message_rotation_is_deterministic() {
    let src = "this is not a hammer. it's a saw.";
    let ps = paragraphs(src);
    let r = NotItsIs;
    let a = r.scan(src, &ps)[0].message.clone();
    let b = r.scan(src, &ps)[0].message.clone();
    assert_eq!(a, b);
}
