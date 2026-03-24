//! filler openers — phrases that stall before saying anything.

use super::{find_phrase, line_col, rotate, Category, Finding, Rule, Severity};
use crate::tokenize::Paragraph;

struct Opener {
    id: &'static str,
    phrase: &'static str,
    msgs: &'static [&'static str],
    severity: Severity,
    bad: &'static str,
    good: &'static str,
}

impl Rule for Opener {
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
        "filler opener. start closer to the point."
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
                fix: None,
            });
        }
        out
    }
}

pub fn push_all(v: &mut Vec<Box<dyn Rule>>) {
    v.push(Box::new(Opener {
        id: "thin.filler-opener.todays-fast-paced",
        phrase: "in today's fast-paced world",
        msgs: &[
            "every day is today. cut this.",
            "in today's fast-paced. sure. cut.",
        ],
        severity: Severity::Error,
        bad: "in today's fast-paced world, developers need speed.",
        good: "developers need speed.",
    }));
    v.push(Box::new(Opener {
        id: "thin.filler-opener.digital-age",
        phrase: "in the digital age",
        msgs: &[
            "we've been digital since 1960. cut.",
            "in the digital age. every age is.",
        ],
        severity: Severity::Warning,
        bad: "in the digital age, data matters.",
        good: "data matters.",
    }));
    v.push(Box::new(Opener {
        id: "thin.filler-opener.when-it-comes-to",
        phrase: "when it comes to",
        msgs: &[
            "when it comes to. cut the preamble.",
            "when it comes to. just name the thing.",
        ],
        severity: Severity::Warning,
        bad: "when it comes to parsers, speed matters.",
        good: "for parsers, speed matters.",
    }));
    v.push(Box::new(Opener {
        id: "thin.filler-opener.in-this-article",
        phrase: "in this article",
        msgs: &[
            "don't announce. just say it.",
            "in this article. the reader knows.",
        ],
        severity: Severity::Warning,
        bad: "in this article, we explore x.",
        good: "here's x.",
    }));
    v.push(Box::new(Opener {
        id: "thin.filler-opener.realm-of",
        phrase: "in the realm of",
        msgs: &[
            "realm of. not a fantasy novel.",
            "in the realm of. tavern-speak. cut.",
        ],
        severity: Severity::Warning,
        bad: "in the realm of databases.",
        good: "among databases.",
    }));
    v.push(Box::new(Opener {
        id: "thin.filler-opener.dive-deep",
        phrase: "dive deep into",
        msgs: &[
            "water metaphor. no water here.",
            "dive deep into a cli tool. dry.",
        ],
        severity: Severity::Error,
        bad: "let's dive deep into sql.",
        good: "here's sql.",
    }));
    v.push(Box::new(Opener {
        id: "thin.filler-opener.without-further-ado",
        phrase: "without further ado",
        msgs: &[
            "without further ado = with further ado.",
            "ado. stop ado-ing.",
        ],
        severity: Severity::Warning,
        bad: "without further ado, here's the code.",
        good: "here's the code.",
    }));
    v.push(Box::new(Opener {
        id: "thin.filler-opener.goes-without-saying",
        phrase: "it goes without saying",
        msgs: &["then don't say it.", "goes without saying. so skip it."],
        severity: Severity::Warning,
        bad: "it goes without saying that tests matter.",
        good: "tests matter.",
    }));
    v.push(Box::new(Opener {
        id: "thin.filler-opener.end-of-the-day",
        phrase: "at the end of the day",
        msgs: &[
            "at the end of the day = 'anyway'.",
            "at the end of the day. morning still has a point.",
        ],
        severity: Severity::Warning,
        bad: "at the end of the day, simplicity wins.",
        good: "simplicity wins.",
    }));
}
