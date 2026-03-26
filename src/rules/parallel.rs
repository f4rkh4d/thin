//! parallel sophistry. "not x. it's y." pattern and variants.

use super::{line_col, rotate, Category, Finding, Rule, Severity};
use crate::tokenize::{sentences, Paragraph};

const MSGS_NOT_ITS: &[&str] = &[
    "this phrasing dominates llm training. break it.",
    "parallel sophistry. say one thing, mean it.",
    "pick one: not x, or it's y. both = generated-feeling.",
];

const MSGS_NOT_JUST: &[&str] = &[
    "'not just x, but y.' the oldest trick. cut.",
    "not-just-but is ad-copy. pick one clause.",
];

const MSGS_ITS_NOT: &[&str] = &[
    "'it's not x, it's y.' recognizable ai cadence.",
    "it's-not-it's. rewrite as one claim.",
];

pub struct NotItsIs;
pub struct NotJustBut;
pub struct ItsNotIts;

fn contains_ci(h: &str, n: &str) -> bool {
    h.to_lowercase().contains(n)
}

impl Rule for NotItsIs {
    fn id(&self) -> &'static str {
        "thin.parallel.not-its"
    }
    fn name(&self) -> &'static str {
        "not x. it's y."
    }
    fn category(&self) -> Category {
        Category::Parallel
    }
    fn default_severity(&self) -> Severity {
        Severity::Error
    }
    fn description(&self) -> &'static str {
        "the 'not x. it's y.' pattern. overrepresented in ai prose."
    }
    fn examples(&self) -> (&'static str, &'static str) {
        (
            "this isn't a database. it's a memory.",
            "a small in-process key-value store.",
        )
    }
    fn scan(&self, src: &str, paragraphs: &[Paragraph<'_>]) -> Vec<Finding> {
        let mut out = Vec::new();
        for p in paragraphs {
            let sents = sentences(p.text, p.start);
            for w in sents.windows(2) {
                let a = w[0].text.to_lowercase();
                let b = w[1].text.to_lowercase();
                let a_has_not = a.contains(" not ")
                    || a.contains("isn't")
                    || a.contains("aren't")
                    || a.contains("is not");
                let b_starts_its =
                    b.trim_start().starts_with("it's ") || b.trim_start().starts_with("its ");
                if a_has_not && b_starts_its {
                    let start = w[0].start;
                    let end = w[1].end;
                    let (line, col) = line_col(src, start);
                    let snippet = src[start..end].to_string();
                    out.push(Finding {
                        rule_id: self.id(),
                        category: self.category(),
                        severity: self.default_severity(),
                        message: rotate(MSGS_NOT_ITS, &snippet).to_string(),
                        start,
                        end,
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

impl Rule for NotJustBut {
    fn id(&self) -> &'static str {
        "thin.parallel.not-just-but"
    }
    fn name(&self) -> &'static str {
        "not just x, but y."
    }
    fn category(&self) -> Category {
        Category::Parallel
    }
    fn default_severity(&self) -> Severity {
        Severity::Warning
    }
    fn description(&self) -> &'static str {
        "the 'not just x, but y' pattern."
    }
    fn examples(&self) -> (&'static str, &'static str) {
        ("not just fast, but fast enough.", "fast enough.")
    }
    fn scan(&self, src: &str, paragraphs: &[Paragraph<'_>]) -> Vec<Finding> {
        let mut out = Vec::new();
        for p in paragraphs {
            let sents = sentences(p.text, p.start);
            for s in &sents {
                if contains_ci(s.text, "not just ") && contains_ci(s.text, "but ") {
                    // ensure "but" comes after "not just"
                    let lower = s.text.to_lowercase();
                    let nj = lower.find("not just ").unwrap();
                    let bu = lower[nj..].find("but ");
                    if bu.is_some() {
                        let start = s.start;
                        let end = s.end;
                        let (line, col) = line_col(src, start);
                        let snippet = src[start..end].to_string();
                        out.push(Finding {
                            rule_id: self.id(),
                            category: self.category(),
                            severity: self.default_severity(),
                            message: rotate(MSGS_NOT_JUST, &snippet).to_string(),
                            start,
                            end,
                            line,
                            col,
                            snippet,
                            fix: None,
                        });
                    }
                }
            }
        }
        out
    }
}

impl Rule for ItsNotIts {
    fn id(&self) -> &'static str {
        "thin.parallel.its-not-its"
    }
    fn name(&self) -> &'static str {
        "it's not x, it's y."
    }
    fn category(&self) -> Category {
        Category::Parallel
    }
    fn default_severity(&self) -> Severity {
        Severity::Error
    }
    fn description(&self) -> &'static str {
        "the 'it's not x, it's y' pattern."
    }
    fn examples(&self) -> (&'static str, &'static str) {
        ("it's not a toy, it's a tool.", "it's a tool.")
    }
    fn scan(&self, src: &str, paragraphs: &[Paragraph<'_>]) -> Vec<Finding> {
        let mut out = Vec::new();
        for p in paragraphs {
            let sents = sentences(p.text, p.start);
            for s in &sents {
                let lower = s.text.to_lowercase();
                if let Some(first) = lower.find("it's not ") {
                    let after = &lower[first + 9..];
                    if after.contains("it's ") {
                        let start = s.start;
                        let end = s.end;
                        let (line, col) = line_col(src, start);
                        let snippet = src[start..end].to_string();
                        out.push(Finding {
                            rule_id: self.id(),
                            category: self.category(),
                            severity: self.default_severity(),
                            message: rotate(MSGS_ITS_NOT, &snippet).to_string(),
                            start,
                            end,
                            line,
                            col,
                            snippet,
                            fix: None,
                        });
                    }
                }
            }
        }
        out
    }
}

pub fn push_all(v: &mut Vec<Box<dyn Rule>>) {
    v.push(Box::new(NotItsIs));
    v.push(Box::new(NotJustBut));
    v.push(Box::new(ItsNotIts));
}
