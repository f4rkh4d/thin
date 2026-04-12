use thin::rules::length::{LongSentenceInfo, LongSentenceWarn};
use thin::rules::Rule;
use thin::tokenize::paragraphs;

#[test]
fn info_flags_36_word_sentence() {
    let words: Vec<&str> = std::iter::repeat("word").take(40).collect();
    let src = format!("{}.", words.join(" "));
    let ps = paragraphs(&src);
    assert_eq!(LongSentenceInfo.scan(&src, &ps).len(), 1);
}

#[test]
fn warn_flags_55_word_sentence() {
    let words: Vec<&str> = std::iter::repeat("word").take(55).collect();
    let src = format!("{}.", words.join(" "));
    let ps = paragraphs(&src);
    assert_eq!(LongSentenceWarn.scan(&src, &ps).len(), 1);
    assert_eq!(LongSentenceInfo.scan(&src, &ps).len(), 0);
}

#[test]
fn skips_short_sentence() {
    let src = "one two three.";
    let ps = paragraphs(src);
    assert_eq!(LongSentenceInfo.scan(src, &ps).len(), 0);
    assert_eq!(LongSentenceWarn.scan(src, &ps).len(), 0);
}
