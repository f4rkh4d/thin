//! long-sentence detector. >35 words = info, >50 = warning.

use super::{line_col, rotate, Category, Finding, Rule, Severity};
use crate::tokenize::{sentences, word_count, Paragraph};

pub struct LongSentenceInfo;
pub struct LongSentenceWarn;

const MSGS: &[&str] = &[
    "this sentence needs a full stop or three.",
    "long sentence. break it.",
    "one thought per sentence.",
];

impl Rule for LongSentenceInfo {
    fn id(&self) -> &'static str {
        "thin.length.sentence-35"
    }
    fn name(&self) -> &'static str {
        "long sentence (35+ words)"
    }
    fn category(&self) -> Category {
        Category::Length
    }
    fn default_severity(&self) -> Severity {
        Severity::Info
    }
    fn description(&self) -> &'static str {
        "a sentence over 35 words. hard to follow."
    }
    fn examples(&self) -> (&'static str, &'static str) {
        (
            "one long sentence that keeps going and going without pause covering multiple ideas in a single breath that the reader can barely follow because there is no break.",
            "shorter. sentences. each one an idea.",
        )
    }
    fn scan(&self, src: &str, paragraphs: &[Paragraph<'_>]) -> Vec<Finding> {
        let mut out = Vec::new();
        for p in paragraphs {
            for s in sentences(p.text, p.start) {
                let wc = word_count(s.text);
                if (35..=50).contains(&wc) {
                    let (line, col) = line_col(src, s.start);
                    let snippet = s.text.chars().take(60).collect::<String>();
                    out.push(Finding {
                        rule_id: self.id(),
                        category: self.category(),
                        severity: self.default_severity(),
                        message: rotate(MSGS, &snippet).to_string(),
                        start: s.start,
                        end: s.end,
                        line,
                        col,
                        snippet,
                        fix: None,
                    });
                }
            }
        }
        out
    }
}

impl Rule for LongSentenceWarn {
    fn id(&self) -> &'static str {
        "thin.length.sentence-50"
    }
    fn name(&self) -> &'static str {
        "very long sentence (50+ words)"
    }
    fn category(&self) -> Category {
        Category::Length
    }
    fn default_severity(&self) -> Severity {
        Severity::Warning
    }
    fn description(&self) -> &'static str {
        "a sentence over 50 words. almost certainly should be split."
    }
    fn examples(&self) -> (&'static str, &'static str) {
        ("(50+ word sentence)", "split into two or three.")
    }
    fn scan(&self, src: &str, paragraphs: &[Paragraph<'_>]) -> Vec<Finding> {
        let mut out = Vec::new();
        for p in paragraphs {
            for s in sentences(p.text, p.start) {
                let wc = word_count(s.text);
                if wc > 50 {
                    let (line, col) = line_col(src, s.start);
                    let snippet = s.text.chars().take(60).collect::<String>();
                    out.push(Finding {
                        rule_id: self.id(),
                        category: self.category(),
                        severity: self.default_severity(),
                        message: rotate(MSGS, &snippet).to_string(),
                        start: s.start,
                        end: s.end,
                        line,
                        col,
                        snippet,
                        fix: None,
                    });
                }
            }
        }
        out
    }
}

pub fn push_all(v: &mut Vec<Box<dyn Rule>>) {
    v.push(Box::new(LongSentenceInfo));
    v.push(Box::new(LongSentenceWarn));
}
