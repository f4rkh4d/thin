//! rule trait + registry.

use serde::Serialize;

pub mod ai_phrase;
pub mod corporate;
pub mod em_dash;
pub mod empty_adjective;
pub mod filler_opener;
pub mod filler_verb;
pub mod length;
pub mod parallel;
pub mod passive;
pub mod redundancy;

use crate::tokenize::Paragraph;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize)]
#[serde(rename_all = "kebab-case")]
pub enum Severity {
    Error,
    Warning,
    Info,
}

impl Severity {
    pub fn as_str(&self) -> &'static str {
        match self {
            Severity::Error => "error",
            Severity::Warning => "warning",
            Severity::Info => "info",
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize)]
#[serde(rename_all = "kebab-case")]
pub enum Category {
    Filler,
    AiPhrase,
    Passive,
    Length,
    Redundancy,
    Corporate,
    Punctuation,
    EmptyAdjective,
    Parallel,
}

impl Category {
    pub fn as_str(&self) -> &'static str {
        match self {
            Category::Filler => "filler",
            Category::AiPhrase => "ai-phrase",
            Category::Passive => "passive",
            Category::Length => "length",
            Category::Redundancy => "redundancy",
            Category::Corporate => "corporate",
            Category::Punctuation => "punctuation",
            Category::EmptyAdjective => "empty-adjective",
            Category::Parallel => "parallel",
        }
    }
}

#[derive(Debug, Clone, Serialize)]
pub struct Finding {
    pub rule_id: &'static str,
    pub category: Category,
    pub severity: Severity,
    pub message: String,
    pub start: usize,
    pub end: usize,
    pub line: usize,
    pub col: usize,
    pub snippet: String,
    /// optional safe replacement for --fix.
    pub fix: Option<String>,
}

/// a single rule. implemented per detector.
pub trait Rule: Send + Sync {
    fn id(&self) -> &'static str;
    fn name(&self) -> &'static str;
    fn category(&self) -> Category;
    fn default_severity(&self) -> Severity;
    /// short description printed by --list-rules.
    fn description(&self) -> &'static str;
    /// bad + good example for --rule <id>.
    fn examples(&self) -> (&'static str, &'static str);
    /// scan the full source and emit findings. paragraphs are precomputed.
    fn scan(&self, src: &str, paragraphs: &[Paragraph<'_>]) -> Vec<Finding>;
}

/// pick a message from a list based on a hash of the matched text.
/// rotates phrasing so repeated flags don't all read identical.
pub fn rotate<'a>(msgs: &'a [&'static str], key: &str) -> &'a str {
    if msgs.is_empty() {
        return "";
    }
    let mut h: u32 = 2166136261;
    for b in key.as_bytes() {
        h ^= *b as u32;
        h = h.wrapping_mul(16777619);
    }
    msgs[(h as usize) % msgs.len()]
}

/// compute (line, col) from a byte offset in `src`, 1-indexed.
pub fn line_col(src: &str, offset: usize) -> (usize, usize) {
    let mut line = 1;
    let mut col = 1;
    for (i, c) in src.char_indices() {
        if i >= offset {
            break;
        }
        if c == '\n' {
            line += 1;
            col = 1;
        } else {
            col += 1;
        }
    }
    (line, col)
}

/// the full registry of rules.
pub fn registry() -> Vec<Box<dyn Rule>> {
    let mut v: Vec<Box<dyn Rule>> = Vec::new();
    v.push(Box::new(em_dash::EmDashCluster));
    empty_adjective::push_all(&mut v);
    filler_verb::push_all(&mut v);
    filler_opener::push_all(&mut v);
    parallel::push_all(&mut v);
    ai_phrase::push_all(&mut v);
    v.push(Box::new(passive::PassiveCluster));
    length::push_all(&mut v);
    redundancy::push_all(&mut v);
    corporate::push_all(&mut v);
    v
}

/// case-insensitive substring search. returns byte ranges into `haystack` where `needle` appears,
/// only when aligned on word boundaries. `needle` may contain spaces.
pub fn find_phrase(haystack: &str, needle: &str) -> Vec<(usize, usize)> {
    if needle.is_empty() {
        return Vec::new();
    }
    let hay_lower = haystack.to_lowercase();
    let needle_lower = needle.to_lowercase();
    let nb = needle_lower.as_bytes();
    let hb = hay_lower.as_bytes();
    let mut out = Vec::new();
    if nb.len() > hb.len() {
        return out;
    }
    let mut i = 0;
    while i + nb.len() <= hb.len() {
        if &hb[i..i + nb.len()] == nb {
            let before_ok = i == 0 || !is_word_char(hb[i - 1]);
            let after_ok = i + nb.len() == hb.len() || !is_word_char(hb[i + nb.len()]);
            if before_ok && after_ok {
                // map back to original string bytes. lowercase can change byte length for some
                // unicode, but for ascii inputs (our needles are all ascii) offsets match.
                out.push((i, i + nb.len()));
                i += nb.len();
                continue;
            }
        }
        i += 1;
    }
    out
}

fn is_word_char(b: u8) -> bool {
    b.is_ascii_alphanumeric() || b == b'_' || b == b'\'' || b == b'-'
}
