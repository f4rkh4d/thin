//! ai-phrase seeds. the highest-signal single-word / short-phrase tells.

use super::{find_phrase, line_col, rotate, Category, Finding, Rule, Severity};
use crate::tokenize::Paragraph;

struct AiPhrase {
    id: &'static str,
    phrase: &'static str,
    msgs: &'static [&'static str],
    severity: Severity,
    bad: &'static str,
    good: &'static str,
}

impl Rule for AiPhrase {
    fn id(&self) -> &'static str {
        self.id
    }
    fn name(&self) -> &'static str {
        self.phrase
    }
    fn category(&self) -> Category {
        Category::AiPhrase
    }
    fn default_severity(&self) -> Severity {
        self.severity
    }
    fn description(&self) -> &'static str {
        "ai-signature phrase. common in generated text."
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
                category: Category::AiPhrase,
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
    v.push(Box::new(AiPhrase {
        id: "thin.ai-phrase.delve",
        phrase: "delve",
        msgs: &[
            "delve is a red flag in 2026. rewrite.",
            "delve. every llm loves this word.",
        ],
        severity: Severity::Error,
        bad: "let's delve into the api.",
        good: "here's the api.",
    }));
    v.push(Box::new(AiPhrase {
        id: "thin.ai-phrase.important-to-note",
        phrase: "it's important to note",
        msgs: &[
            "if it's important, just say it.",
            "important to note = note.",
        ],
        severity: Severity::Error,
        bad: "it's important to note that caching helps.",
        good: "caching helps.",
    }));
    v.push(Box::new(AiPhrase {
        id: "thin.ai-phrase.worth-mentioning",
        phrase: "it's worth mentioning",
        msgs: &[
            "worth mentioning = mention.",
            "worth mentioning. just mention.",
        ],
        severity: Severity::Warning,
        bad: "it's worth mentioning the trade-off.",
        good: "there's a trade-off.",
    }));
    v.push(Box::new(AiPhrase {
        id: "thin.ai-phrase.crucial-to-understand",
        phrase: "it's crucial to understand",
        msgs: &[
            "crucial to understand = note.",
            "crucial to understand. just explain.",
        ],
        severity: Severity::Warning,
        bad: "it's crucial to understand atomicity.",
        good: "atomicity matters here.",
    }));
    v.push(Box::new(AiPhrase {
        id: "thin.ai-phrase.in-conclusion",
        phrase: "in conclusion",
        msgs: &[
            "the reader sees the end coming. don't announce.",
            "in conclusion. stop narrating.",
        ],
        severity: Severity::Error,
        bad: "in conclusion, sqlite wins.",
        good: "sqlite wins.",
    }));
    v.push(Box::new(AiPhrase {
        id: "thin.ai-phrase.to-summarize",
        phrase: "to summarize",
        msgs: &[
            "to summarize. or just summarize.",
            "to summarize. cut; write the summary.",
        ],
        severity: Severity::Warning,
        bad: "to summarize, it's fast.",
        good: "it's fast.",
    }));
    v.push(Box::new(AiPhrase {
        id: "thin.ai-phrase.certainly",
        phrase: "certainly!",
        msgs: &[
            "certainly! chatbot opener. cut.",
            "certainly! dead giveaway.",
        ],
        severity: Severity::Error,
        bad: "certainly! here's the code.",
        good: "here's the code.",
    }));
    v.push(Box::new(AiPhrase {
        id: "thin.ai-phrase.of-course",
        phrase: "of course!",
        msgs: &[
            "of course! reads as assistant boilerplate.",
            "of course! skip the greeting.",
        ],
        severity: Severity::Warning,
        bad: "of course! here are three options.",
        good: "three options:",
    }));
    v.push(Box::new(AiPhrase {
        id: "thin.ai-phrase.testament-to",
        phrase: "a testament to",
        msgs: &[
            "testament to. biblical phrasing. cut.",
            "testament to. name the evidence.",
        ],
        severity: Severity::Error,
        bad: "a testament to rust's design.",
        good: "proof: the binary is 200kb.",
    }));
    v.push(Box::new(AiPhrase {
        id: "thin.ai-phrase.navigate-complexities",
        phrase: "navigate the complexities of",
        msgs: &[
            "navigate is for ships. drop it.",
            "navigate the complexities of. pick a verb.",
        ],
        severity: Severity::Error,
        bad: "navigate the complexities of oauth.",
        good: "handle oauth.",
    }));
    v.push(Box::new(AiPhrase {
        id: "thin.ai-phrase.crux-of-the-matter",
        phrase: "the crux of the matter",
        msgs: &[
            "crux of the matter. overwritten phrasing.",
            "crux of the matter. cut.",
        ],
        severity: Severity::Warning,
        bad: "the crux of the matter is throughput.",
        good: "throughput is the problem.",
    }));
    v.push(Box::new(AiPhrase {
        id: "thin.ai-phrase.ever-evolving-landscape",
        phrase: "in the ever-evolving landscape of",
        msgs: &[
            "ever-evolving landscape. every ai blog.",
            "ever-evolving landscape. cut entirely.",
        ],
        severity: Severity::Error,
        bad: "in the ever-evolving landscape of web dev.",
        good: "web dev keeps changing.",
    }));
    v.push(Box::new(AiPhrase {
        id: "thin.ai-phrase.journey-through",
        phrase: "a journey through",
        msgs: &[
            "not a journey. a tutorial.",
            "a journey through. we're not on a bus.",
        ],
        severity: Severity::Warning,
        bad: "a journey through the codebase.",
        good: "a tour of the codebase.",
    }));
    v.push(Box::new(AiPhrase {
        id: "thin.ai-phrase.unlocking-the-potential",
        phrase: "unlocking the potential of",
        msgs: &[
            "unlocking the potential. ad-copy. cut.",
            "unlocking the potential. no locks here.",
        ],
        severity: Severity::Warning,
        bad: "unlocking the potential of your team.",
        good: "your team ships faster.",
    }));
}
