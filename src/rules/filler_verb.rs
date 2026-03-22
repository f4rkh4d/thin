//! filler verbs — corporate verbs that inflate meaning.

use super::{find_phrase, line_col, rotate, Category, Finding, Rule, Severity};
use crate::tokenize::Paragraph;

struct FillerVerb {
    id: &'static str,
    phrase: &'static str,
    msgs: &'static [&'static str],
    severity: Severity,
    fix: Option<&'static str>,
    bad: &'static str,
    good: &'static str,
}

impl Rule for FillerVerb {
    fn id(&self) -> &'static str {
        self.id
    }
    fn name(&self) -> &'static str {
        self.phrase
    }
    fn category(&self) -> Category {
        Category::Filler
    }
    fn default_severity(&self) -> Severity {
        self.severity
    }
    fn description(&self) -> &'static str {
        "filler verb. replace with a concrete one."
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
                category: Category::Filler,
                severity: self.severity,
                message: rotate(self.msgs, &snippet).to_string(),
                start: s,
                end: e,
                line,
                col,
                snippet,
                fix: self.fix.map(|f| f.to_string()),
            });
        }
        out
    }
}

pub fn push_all(v: &mut Vec<Box<dyn Rule>>) {
    v.push(Box::new(FillerVerb {
        id: "thin.filler.leverages",
        phrase: "leverages",
        msgs: &[
            "you're not a forklift. use 'uses'.",
            "leverages. we're not in 2014 consulting.",
        ],
        severity: Severity::Error,
        fix: Some("uses"),
        bad: "leverages rust's type system.",
        good: "uses rust's type system.",
    }));
    v.push(Box::new(FillerVerb {
        id: "thin.filler.utilizes",
        phrase: "utilizes",
        msgs: &["utilizes = uses + hubris.", "utilizes. say 'uses'."],
        severity: Severity::Error,
        fix: Some("uses"),
        bad: "utilizes a cache.",
        good: "uses a cache.",
    }));
    v.push(Box::new(FillerVerb {
        id: "thin.filler.harnesses-the-power-of",
        phrase: "harnesses the power of",
        msgs: &[
            "not training horses. say the word.",
            "harnesses. drop it, you're not in a saddle.",
        ],
        severity: Severity::Error,
        fix: None,
        bad: "harnesses the power of simd.",
        good: "uses simd.",
    }));
    v.push(Box::new(FillerVerb {
        id: "thin.filler.empowers",
        phrase: "empowers",
        msgs: &[
            "empower is a ted-talk word. drop it.",
            "empowers. pick a verb.",
        ],
        severity: Severity::Warning,
        fix: None,
        bad: "empowers developers to ship.",
        good: "lets developers ship.",
    }));
    v.push(Box::new(FillerVerb {
        id: "thin.filler.enables-you-to",
        phrase: "enables you to",
        msgs: &[
            "enables you to = lets you.",
            "enables you to. just say 'lets'.",
        ],
        severity: Severity::Warning,
        fix: Some("lets you"),
        bad: "enables you to query.",
        good: "lets you query.",
    }));
    v.push(Box::new(FillerVerb {
        id: "thin.filler.facilitates",
        phrase: "facilitates",
        msgs: &[
            "facilitates is a hr word. replace.",
            "facilitates. use a real verb.",
        ],
        severity: Severity::Warning,
        fix: None,
        bad: "facilitates collaboration.",
        good: "puts a shared cursor in the doc.",
    }));
    v.push(Box::new(FillerVerb {
        id: "thin.filler.streamlines",
        phrase: "streamlines",
        msgs: &[
            "streamlines. name the step you cut.",
            "streamlines = 'did something'.",
        ],
        severity: Severity::Warning,
        fix: None,
        bad: "streamlines your workflow.",
        good: "cuts two steps from the deploy.",
    }));
    v.push(Box::new(FillerVerb {
        id: "thin.filler.unlocks",
        phrase: "unlocks",
        msgs: &["unlocks. there's no lock here.", "unlocks. name the door."],
        severity: Severity::Warning,
        fix: None,
        bad: "unlocks new possibilities.",
        good: "you can now chain pipes.",
    }));
    v.push(Box::new(FillerVerb {
        id: "thin.filler.revolutionizes",
        phrase: "revolutionizes",
        msgs: &["revolutionizes. no. cut.", "revolutionizes. please."],
        severity: Severity::Error,
        fix: None,
        bad: "revolutionizes logging.",
        good: "writes logs as sqlite rows.",
    }));
    v.push(Box::new(FillerVerb {
        id: "thin.filler.transforms-your",
        phrase: "transforms your",
        msgs: &[
            "transforms your. sales-deck phrasing.",
            "transforms your. pick a concrete verb.",
        ],
        severity: Severity::Warning,
        fix: None,
        bad: "transforms your stack.",
        good: "replaces three services with one binary.",
    }));
}
