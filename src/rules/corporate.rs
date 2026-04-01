//! corporate-speak. meeting words in prose.

use super::{find_phrase, line_col, rotate, Category, Finding, Rule, Severity};
use crate::tokenize::Paragraph;

struct Corp {
    id: &'static str,
    phrase: &'static str,
    msgs: &'static [&'static str],
    severity: Severity,
    bad: &'static str,
    good: &'static str,
}

impl Rule for Corp {
    fn id(&self) -> &'static str {
        self.id
    }
    fn name(&self) -> &'static str {
        self.phrase
    }
    fn category(&self) -> Category {
        Category::Corporate
    }
    fn default_severity(&self) -> Severity {
        self.severity
    }
    fn description(&self) -> &'static str {
        "corporate-speak. belongs in slack, not docs."
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
                category: Category::Corporate,
                severity: self.severity,
                message: rotate(self.msgs, &snippet).to_string(),
                start: s,
                end: e,
                line,
                col,
                snippet,
                fix: None,
            });
        }
        out
    }
}

pub fn push_all(v: &mut Vec<Box<dyn Rule>>) {
    v.push(Box::new(Corp {
        id: "thin.corporate.synergy",
        phrase: "synergy",
        msgs: &[
            "synergy. belongs in a deck, not docs.",
            "synergy. it's 2026.",
        ],
        severity: Severity::Error,
        bad: "team synergy is key.",
        good: "the team works well together.",
    }));
    v.push(Box::new(Corp {
        id: "thin.corporate.circle-back",
        phrase: "circle back",
        msgs: &[
            "circle back = 'i'll reply later'.",
            "circle back. we're not cars.",
        ],
        severity: Severity::Warning,
        bad: "let's circle back.",
        good: "i'll reply tomorrow.",
    }));
    v.push(Box::new(Corp {
        id: "thin.corporate.touch-base",
        phrase: "touch base",
        msgs: &["touch base. sports metaphor; cut.", "touch base = talk."],
        severity: Severity::Warning,
        bad: "let's touch base soon.",
        good: "let's talk soon.",
    }));
    v.push(Box::new(Corp {
        id: "thin.corporate.move-the-needle",
        phrase: "move the needle",
        msgs: &[
            "move the needle. meeting-speak.",
            "move the needle. no gauges here.",
        ],
        severity: Severity::Warning,
        bad: "this moves the needle.",
        good: "this matters to users.",
    }));
    v.push(Box::new(Corp {
        id: "thin.corporate.low-hanging-fruit",
        phrase: "low-hanging fruit",
        msgs: &[
            "low-hanging fruit. tired metaphor.",
            "low-hanging fruit. pick the task.",
        ],
        severity: Severity::Warning,
        bad: "grab the low-hanging fruit.",
        good: "fix the easy bugs first.",
    }));
    v.push(Box::new(Corp {
        id: "thin.corporate.paradigm-shift",
        phrase: "paradigm shift",
        msgs: &[
            "paradigm shift. kuhn hated this use.",
            "paradigm shift. overstated.",
        ],
        severity: Severity::Warning,
        bad: "a paradigm shift in x.",
        good: "a different way to do x.",
    }));
    v.push(Box::new(Corp {
        id: "thin.corporate.mission-critical",
        phrase: "mission-critical",
        msgs: &[
            "mission-critical. we're not nasa.",
            "mission-critical. just say 'important'.",
        ],
        severity: Severity::Warning,
        bad: "mission-critical system.",
        good: "important system.",
    }));
    v.push(Box::new(Corp {
        id: "thin.corporate.best-practices",
        phrase: "best practices",
        msgs: &[
            "best practices = 'stuff i do'.",
            "best practices. name them.",
        ],
        severity: Severity::Info,
        bad: "follow best practices.",
        good: "use structured logs, pin versions.",
    }));
    v.push(Box::new(Corp {
        id: "thin.corporate.core-competency",
        phrase: "core competency",
        msgs: &[
            "core competency. consultant word.",
            "core competency = 'what we do'.",
        ],
        severity: Severity::Warning,
        bad: "our core competency is data.",
        good: "we work on data.",
    }));
}
