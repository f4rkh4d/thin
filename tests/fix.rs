use thin::fix::apply_fixes;
use thin::rules::registry;
use thin::tokenize::paragraphs;

#[test]
fn fix_replaces_utilizes_with_uses() {
    let src = "we utilizes a cache.";
    let ps = paragraphs(src);
    let reg = registry();
    let mut all = Vec::new();
    for r in &reg {
        all.extend(r.scan(src, &ps));
    }
    let (out, n) = apply_fixes(src, &all);
    assert!(n >= 1);
    assert!(out.contains("uses"));
    assert!(!out.contains("utilizes"));
}

#[test]
fn fix_handles_in_order_to() {
    let src = "run it in order to compile.";
    let ps = paragraphs(src);
    let reg = registry();
    let mut all = Vec::new();
    for r in &reg {
        all.extend(r.scan(src, &ps));
    }
    let (out, _) = apply_fixes(src, &all);
    assert_eq!(out, "run it to compile.");
}

#[test]
fn fix_preserves_capitalization() {
    let src = "Utilizes a cache.";
    let ps = paragraphs(src);
    let reg = registry();
    let mut all = Vec::new();
    for r in &reg {
        all.extend(r.scan(src, &ps));
    }
    let (out, _) = apply_fixes(src, &all);
    assert!(out.starts_with("Uses"));
}

#[test]
fn fix_skips_rules_without_replacement() {
    // em-dash cluster has no fix
    let src = "a — b — c — d.";
    let ps = paragraphs(src);
    let reg = registry();
    let mut all = Vec::new();
    for r in &reg {
        all.extend(r.scan(src, &ps));
    }
    let (out, n) = apply_fixes(src, &all);
    assert_eq!(n, 0);
    assert_eq!(out, src);
}
