// rule-level unit tests live under tests/rules/.
#[path = "rules/em_dash.rs"]
mod em_dash;

#[path = "rules/empty_adjective.rs"]
mod empty_adjective;

#[path = "rules/parallel.rs"]
mod parallel;

#[path = "rules/ai_phrase.rs"]
mod ai_phrase;

#[path = "rules/length.rs"]
mod length;

#[path = "rules/redundancy.rs"]
mod redundancy;

#[path = "rules/corporate.rs"]
mod corporate;

#[path = "rules/filler_verb.rs"]
mod filler_verb;
