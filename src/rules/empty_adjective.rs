//! empty adjectives — words that say nothing.

use super::{find_phrase, line_col, rotate, Category, Finding, Rule, Severity};
use crate::tokenize::Paragraph;

struct PhraseRule {
    id: &'static str,
    phrase: &'static str,
    msgs: &'static [&'static str],
    severity: Severity,
    bad: &'static str,
    good: &'static str,
}

impl Rule for PhraseRule {
    fn id(&self) -> &'static str {
        self.id
    }
    fn name(&self) -> &'static str {
        self.phrase
    }
    fn category(&self) -> Category {
        Category::EmptyAdjective
    }
    fn default_severity(&self) -> Severity {
        self.severity
    }
    fn description(&self) -> &'static str {
        "empty adjective. carries no information."
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
                category: Category::EmptyAdjective,
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
    v.push(Box::new(PhraseRule {
        id: "thin.empty-adjective.seamless",
        phrase: "seamless",
        msgs: &[
            "seamless is a jacket thing. cut.",
            "nothing is seamless. name the seam.",
        ],
        severity: Severity::Error,
        bad: "a seamless experience.",
        good: "no setup, no config, runs on launch.",
    }));
    v.push(Box::new(PhraseRule {
        id: "thin.empty-adjective.robust",
        phrase: "robust",
        msgs: &[
            "robust means nothing. pick a property.",
            "robust is a stand-in word. replace.",
        ],
        severity: Severity::Warning,
        bad: "a robust system.",
        good: "handles 10k rps without dropping.",
    }));
    v.push(Box::new(PhraseRule {
        id: "thin.empty-adjective.blazingly-fast",
        phrase: "blazingly fast",
        msgs: &[
            "every rust readme ever. show a number.",
            "blazingly fast says 'no benchmarks'.",
        ],
        severity: Severity::Error,
        bad: "blazingly fast parser.",
        good: "parses 400mb/s on an m2.",
    }));
    v.push(Box::new(PhraseRule {
        id: "thin.empty-adjective.lightning-fast",
        phrase: "lightning fast",
        msgs: &[
            "lightning is hot plasma. your binary isn't.",
            "lightning fast. show the number.",
        ],
        severity: Severity::Warning,
        bad: "lightning fast queries.",
        good: "p99 under 3ms on 10m rows.",
    }));
    v.push(Box::new(PhraseRule {
        id: "thin.empty-adjective.powerful",
        phrase: "powerful",
        msgs: &[
            "powerful is filler. name the power.",
            "powerful. name one thing it does.",
        ],
        severity: Severity::Warning,
        bad: "a powerful toolkit.",
        good: "a toolkit that diffs binaries.",
    }));
    v.push(Box::new(PhraseRule {
        id: "thin.empty-adjective.intelligent",
        phrase: "intelligent",
        msgs: &[
            "intelligent means nothing here. cut.",
            "intelligent. show the logic.",
        ],
        severity: Severity::Warning,
        bad: "intelligent search.",
        good: "search that matches by prefix and by body.",
    }));
    v.push(Box::new(PhraseRule {
        id: "thin.empty-adjective.elegant",
        phrase: "elegant",
        msgs: &[
            "elegant is a compliment, not a feature.",
            "elegant code. prove it.",
        ],
        severity: Severity::Info,
        bad: "elegant api.",
        good: "one function. two args. returns a stream.",
    }));
    v.push(Box::new(PhraseRule {
        id: "thin.empty-adjective.modern",
        phrase: "modern",
        msgs: &["modern = when? pick a year.", "modern. name the decade."],
        severity: Severity::Warning,
        bad: "a modern approach.",
        good: "uses tokio, no locks.",
    }));
    v.push(Box::new(PhraseRule {
        id: "thin.empty-adjective.cutting-edge",
        phrase: "cutting-edge",
        msgs: &["cutting-edge ages in two weeks.", "cutting-edge. specify."],
        severity: Severity::Warning,
        bad: "cutting-edge tech.",
        good: "built on wgpu + wayland.",
    }));
    v.push(Box::new(PhraseRule {
        id: "thin.empty-adjective.state-of-the-art",
        phrase: "state-of-the-art",
        msgs: &[
            "state-of-the-art. cite the paper.",
            "state-of-the-art according to whom.",
        ],
        severity: Severity::Warning,
        bad: "state-of-the-art compression.",
        good: "beats zstd -19 by 8% on text.",
    }));
    v.push(Box::new(PhraseRule {
        id: "thin.empty-adjective.next-gen",
        phrase: "next-gen",
        msgs: &[
            "next-gen. which gen are we on.",
            "next-gen = last-gen + marketing.",
        ],
        severity: Severity::Warning,
        bad: "next-gen platform.",
        good: "an http server, written this year.",
    }));
    v.push(Box::new(PhraseRule {
        id: "thin.empty-adjective.revolutionary",
        phrase: "revolutionary",
        msgs: &[
            "nothing is revolutionary. cut.",
            "revolutionary is a press release word.",
        ],
        severity: Severity::Error,
        bad: "a revolutionary approach.",
        good: "a slightly-smaller json parser.",
    }));
    v.push(Box::new(PhraseRule {
        id: "thin.empty-adjective.game-changing",
        phrase: "game-changing",
        msgs: &["every saas says this. skip.", "game-changing. which game."],
        severity: Severity::Error,
        bad: "game-changing workflow.",
        good: "one command instead of four.",
    }));
    v.push(Box::new(PhraseRule {
        id: "thin.empty-adjective.industry-leading",
        phrase: "industry-leading",
        msgs: &[
            "which industry. leading who.",
            "industry-leading. compared to?",
        ],
        severity: Severity::Warning,
        bad: "industry-leading speed.",
        good: "faster than ripgrep on 1gb logs.",
    }));
    v.push(Box::new(PhraseRule {
        id: "thin.empty-adjective.best-in-class",
        phrase: "best-in-class",
        msgs: &[
            "best-in-class. what class. who judges.",
            "best-in-class. receipts?",
        ],
        severity: Severity::Warning,
        bad: "best-in-class security.",
        good: "memory-safe by construction. audited in march.",
    }));
    v.push(Box::new(PhraseRule {
        id: "thin.empty-adjective.world-class",
        phrase: "world-class",
        msgs: &["world-class. olympics of what.", "world-class. drop it."],
        severity: Severity::Warning,
        bad: "world-class performance.",
        good: "runs in 300ms cold start.",
    }));
}
