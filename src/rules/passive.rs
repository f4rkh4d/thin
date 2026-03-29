//! passive voice heuristic. flag paragraphs where more than half the sentences are passive.

use super::{line_col, rotate, Category, Finding, Rule, Severity};
use crate::tokenize::{sentences, Paragraph};

pub struct PassiveCluster;

const MSGS: &[&str] = &[
    "passive. who did it?",
    "3 passives in a row. pick subjects.",
    "passive cluster. name the actor.",
];

// be-verbs followed by a past participle (heuristic: word ends with "ed" or common irregulars).
const BE_VERBS: &[&str] = &["is", "was", "were", "been", "being", "are", "be"];

const IRREGULAR_PP: &[&str] = &[
    "done",
    "made",
    "built",
    "seen",
    "written",
    "taken",
    "given",
    "shown",
    "held",
    "kept",
    "left",
    "found",
    "brought",
    "thought",
    "bought",
    "caught",
    "taught",
    "said",
    "sent",
    "set",
    "put",
    "run",
    "paid",
    "known",
    "gone",
    "come",
    "chosen",
    "broken",
    "spoken",
    "driven",
    "risen",
    "fallen",
    "hidden",
    "eaten",
    "forgotten",
    "forgiven",
    "cut",
    "read",
    "lost",
    "met",
    "led",
];

fn is_past_participle(word: &str) -> bool {
    let w = word
        .trim_end_matches(|c: char| !c.is_alphanumeric())
        .to_lowercase();
    if w.len() >= 3 && w.ends_with("ed") {
        return true;
    }
    IRREGULAR_PP.iter().any(|pp| *pp == w)
}

fn is_passive(sentence: &str) -> bool {
    let lower = sentence.to_lowercase();
    let words: Vec<&str> = lower.split_whitespace().collect();
    for w in words.windows(2) {
        if BE_VERBS.contains(&w[0]) && is_past_participle(w[1]) {
            return true;
        }
    }
    // also "been X" or "being X" with one-word gap
    for w in words.windows(3) {
        if BE_VERBS.contains(&w[0]) && is_past_participle(w[2]) {
            return true;
        }
    }
    false
}

impl Rule for PassiveCluster {
    fn id(&self) -> &'static str {
        "thin.passive.cluster"
    }
    fn name(&self) -> &'static str {
        "passive-heavy paragraph"
    }
    fn category(&self) -> Category {
        Category::Passive
    }
    fn default_severity(&self) -> Severity {
        Severity::Warning
    }
    fn description(&self) -> &'static str {
        "a paragraph where more than half of sentences are passive voice."
    }
    fn examples(&self) -> (&'static str, &'static str) {
        (
            "the file was opened. the bytes were read. the data was parsed.",
            "we opened the file. we read the bytes. we parsed the data.",
        )
    }
    fn scan(&self, src: &str, paragraphs: &[Paragraph<'_>]) -> Vec<Finding> {
        let mut out = Vec::new();
        for p in paragraphs {
            let sents = sentences(p.text, p.start);
            if sents.len() < 2 {
                continue;
            }
            let passive_count = sents.iter().filter(|s| is_passive(s.text)).count();
            if passive_count * 2 > sents.len() {
                let start = p.start;
                let end = p.end;
                let (line, col) = line_col(src, start);
                let snippet = p.text.chars().take(60).collect::<String>();
                out.push(Finding {
                    rule_id: self.id(),
                    category: self.category(),
                    severity: self.default_severity(),
                    message: rotate(MSGS, &snippet).to_string(),
                    start,
                    end,
                    line,
                    col,
                    snippet,
                    fix: None,
                });
            }
        }
        out
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn detects_passive() {
        assert!(is_passive("the file was opened"));
        assert!(is_passive("the bytes were read"));
        assert!(is_passive("the record is stored"));
    }

    #[test]
    fn skips_active() {
        assert!(!is_passive("we open the file"));
        assert!(!is_passive("i wrote the parser"));
    }
}
