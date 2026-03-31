//! redundant pairs. "end result" = "result". etc.

use super::{find_phrase, line_col, rotate, Category, Finding, Rule, Severity};
use crate::tokenize::Paragraph;

struct Redundant {
    id: &'static str,
    phrase: &'static str,
    fix: &'static str,
    msgs: &'static [&'static str],
    bad: &'static str,
    good: &'static str,
}

impl Rule for Redundant {
    fn id(&self) -> &'static str {
        self.id
    }
    fn name(&self) -> &'static str {
        self.phrase
    }
    fn category(&self) -> Category {
        Category::Redundancy
    }
    fn default_severity(&self) -> Severity {
        Severity::Warning
    }
    fn description(&self) -> &'static str {
        "redundant phrase. one word does the job."
    }
    fn examples(&self) -> (&'static str, &'static str) {
        (self.bad, self.good)
    }
    fn scan(&self, src: &str, _paragraphs: &[Paragraph<'_>]) -> Vec<Finding> {
        let mut out = Vec::new();
        for (s, e) in find_phrase(src, self.phrase) {
            let (line, col) = line_col(src, s);
            let snippet = src[s..e].to_string();
            out.push(Finding {
                rule_id: self.id,
                category: Category::Redundancy,
                severity: Severity::Warning,
                message: rotate(self.msgs, &snippet).to_string(),
                start: s,
                end: e,
                line,
                col,
                snippet,
                fix: Some(self.fix.to_string()),
            });
        }
        out
    }
}

pub fn push_all(v: &mut Vec<Box<dyn Rule>>) {
    v.push(Box::new(Redundant {
        id: "thin.redundancy.end-result",
        phrase: "end result",
        fix: "result",
        msgs: &["end result = result.", "end result. the end is implied."],
        bad: "the end result was clear.",
        good: "the result was clear.",
    }));
    v.push(Box::new(Redundant {
        id: "thin.redundancy.past-history",
        phrase: "past history",
        fix: "history",
        msgs: &[
            "past history = history.",
            "past history. all history is past.",
        ],
        bad: "check past history.",
        good: "check history.",
    }));
    v.push(Box::new(Redundant {
        id: "thin.redundancy.future-plans",
        phrase: "future plans",
        fix: "plans",
        msgs: &[
            "future plans = plans.",
            "future plans. plans are already future.",
        ],
        bad: "future plans include x.",
        good: "plans include x.",
    }));
    v.push(Box::new(Redundant {
        id: "thin.redundancy.basic-fundamentals",
        phrase: "basic fundamentals",
        fix: "fundamentals",
        msgs: &[
            "fundamentals are basic by definition.",
            "basic fundamentals. pick one.",
        ],
        bad: "learn the basic fundamentals.",
        good: "learn the fundamentals.",
    }));
    v.push(Box::new(Redundant {
        id: "thin.redundancy.completely-finished",
        phrase: "completely finished",
        fix: "finished",
        msgs: &[
            "finished is already complete.",
            "completely finished. cut one.",
        ],
        bad: "it's completely finished.",
        good: "it's finished.",
    }));
    v.push(Box::new(Redundant {
        id: "thin.redundancy.each-and-every",
        phrase: "each and every",
        fix: "every",
        msgs: &["each and every = every.", "each and every. redundant pair."],
        bad: "each and every user.",
        good: "every user.",
    }));
    v.push(Box::new(Redundant {
        id: "thin.redundancy.free-gift",
        phrase: "free gift",
        fix: "gift",
        msgs: &["gifts are free by definition.", "free gift. pick one."],
        bad: "get a free gift.",
        good: "get a gift.",
    }));
    v.push(Box::new(Redundant {
        id: "thin.redundancy.mutual-cooperation",
        phrase: "mutual cooperation",
        fix: "cooperation",
        msgs: &[
            "cooperation is mutual already.",
            "mutual cooperation. trim.",
        ],
        bad: "mutual cooperation is key.",
        good: "cooperation is key.",
    }));
    v.push(Box::new(Redundant {
        id: "thin.redundancy.advance-forward",
        phrase: "advance forward",
        fix: "advance",
        msgs: &["advance is forward already.", "advance forward. pick one."],
        bad: "advance forward in line.",
        good: "advance in line.",
    }));
    v.push(Box::new(Redundant {
        id: "thin.redundancy.in-order-to",
        phrase: "in order to",
        fix: "to",
        msgs: &["in order to = to.", "in order to. three extra words."],
        bad: "in order to compile, run x.",
        good: "to compile, run x.",
    }));
}
