//! em-dash cluster detector. 3+ em-dashes in one paragraph = ai signature.

use super::{line_col, rotate, Category, Finding, Rule, Severity};
use crate::tokenize::Paragraph;

pub struct EmDashCluster;

const MSGS: &[&str] = &[
    "three em-dashes. your keyboard has commas.",
    "em-dash cluster. pick a lane.",
    "three em-dashes in one breath. breathe.",
];

impl Rule for EmDashCluster {
    fn id(&self) -> &'static str {
        "thin.em-dash.cluster"
    }
    fn name(&self) -> &'static str {
        "em-dash cluster"
    }
    fn category(&self) -> Category {
        Category::Punctuation
    }
    fn default_severity(&self) -> Severity {
        Severity::Error
    }
    fn description(&self) -> &'static str {
        "three or more em-dashes in a single paragraph. strongest ai-prose signal."
    }
    fn examples(&self) -> (&'static str, &'static str) {
        (
            "our product — fast, reliable — built with care — and yours.",
            "our product is fast and reliable. built with care. yours.",
        )
    }
    fn scan(&self, src: &str, paragraphs: &[Paragraph<'_>]) -> Vec<Finding> {
        let mut out = Vec::new();
        for p in paragraphs {
            // find byte positions of em-dash (U+2014, 0xE2 0x80 0x94) in this paragraph
            let bytes = p.text.as_bytes();
            let mut positions = Vec::new();
            let mut i = 0;
            while i + 3 <= bytes.len() {
                if bytes[i] == 0xE2 && bytes[i + 1] == 0x80 && bytes[i + 2] == 0x94 {
                    positions.push(i);
                    i += 3;
                } else {
                    i += 1;
                }
            }
            if positions.len() >= 3 {
                let start = p.start + positions[0];
                let end = p.start + positions[positions.len() - 1] + 3;
                let (line, col) = line_col(src, start);
                let snippet = src[start..end].to_string();
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
