//! clap entry + dispatch.

use crate::fix::apply_fixes;
use crate::json::{counts, FileReport};
use crate::profile::{effective_severity, Config, Preset};
use crate::render::render;
use crate::rules::{registry, Finding, Rule, Severity};
use crate::tokenize::paragraphs;
use clap::Parser;
use std::io::Read;
use std::path::{Path, PathBuf};

/// thin — finds ai writing without using ai.
#[derive(Parser, Debug)]
#[command(name = "thin", version, about = "finds ai writing in your readmes. without using ai.", long_about = None)]
pub struct Args {
    /// file(s) or glob(s) to lint.
    pub paths: Vec<String>,

    /// read from stdin.
    #[arg(long)]
    pub stdin: bool,

    /// apply safe auto-replacements in place.
    #[arg(long)]
    pub fix: bool,

    /// output format.
    #[arg(long, default_value = "pretty")]
    pub format: String,

    /// preset profile: balanced | frkhd | relaxed | corporate.
    #[arg(long, default_value = "balanced")]
    pub profile: String,

    /// path to a thin.toml config.
    #[arg(long)]
    pub config: Option<PathBuf>,

    /// list every rule with id and description.
    #[arg(long)]
    pub list_rules: bool,

    /// show details for one rule by id.
    #[arg(long)]
    pub rule: Option<String>,

    /// disable colors.
    #[arg(long)]
    pub no_color: bool,
}

/// main dispatch. returns the process exit code.
pub fn run(args: Args) -> i32 {
    let reg = registry();

    if args.list_rules {
        list_rules(&reg);
        return 0;
    }

    if let Some(id) = &args.rule {
        return show_rule(&reg, id);
    }

    // load config.
    let mut cfg = match &args.config {
        Some(p) => match Config::from_file(p) {
            Ok(c) => c,
            Err(e) => {
                eprintln!("thin: config error: {}", e);
                return 2;
            }
        },
        None => {
            let default = Path::new("thin.toml");
            if default.exists() {
                Config::from_file(default).unwrap_or_default()
            } else {
                Config::default()
            }
        }
    };
    // cli --profile overrides config preset.
    let preset = Preset::parse(&args.profile)
        .or(cfg.preset)
        .unwrap_or(Preset::Balanced);
    cfg.preset = Some(preset);

    // collect inputs.
    let mut inputs: Vec<(String, String)> = Vec::new();
    if args.stdin {
        let mut s = String::new();
        if let Err(e) = std::io::stdin().read_to_string(&mut s) {
            eprintln!("thin: stdin error: {}", e);
            return 2;
        }
        inputs.push(("<stdin>".to_string(), s));
    } else {
        if args.paths.is_empty() {
            eprintln!("thin: no input. pass a file, a glob, or --stdin.");
            return 2;
        }
        for pat in &args.paths {
            let path_buf = PathBuf::from(pat);
            if path_buf.is_file() {
                match std::fs::read_to_string(&path_buf) {
                    Ok(s) => inputs.push((pat.clone(), s)),
                    Err(e) => {
                        eprintln!("thin: {}: {}", pat, e);
                        return 2;
                    }
                }
                continue;
            }
            match glob::glob(pat) {
                Ok(entries) => {
                    let mut any = false;
                    for e in entries.flatten() {
                        if e.is_file() {
                            any = true;
                            match std::fs::read_to_string(&e) {
                                Ok(s) => inputs.push((e.display().to_string(), s)),
                                Err(err) => eprintln!("thin: {}: {}", e.display(), err),
                            }
                        }
                    }
                    if !any && !path_buf.exists() {
                        eprintln!("thin: no files match '{}'", pat);
                    }
                }
                Err(e) => {
                    eprintln!("thin: bad glob '{}': {}", pat, e);
                    return 2;
                }
            }
        }
    }

    let use_color = !args.no_color && atty_stdout();

    let mut total_errors = 0usize;
    let mut total_all = 0usize;
    let json_mode = args.format == "json";
    let mut json_reports: Vec<(String, Vec<Finding>)> = Vec::new();

    for (path, src) in &inputs {
        // path-level ignore
        if cfg.ignore_paths.iter().any(|p| path.contains(p)) {
            continue;
        }
        // file-level ignore pattern
        if cfg
            .ignore_patterns
            .iter()
            .any(|pat| src.contains(pat) && pat.contains("ignore-file"))
        {
            continue;
        }

        let paragraphs = paragraphs(src);
        let mut all_findings: Vec<Finding> = Vec::new();
        for rule in &reg {
            let Some(sev) = effective_severity(
                rule.id(),
                rule.category(),
                rule.default_severity(),
                preset,
                &cfg.rule_overrides,
            ) else {
                continue;
            };
            let mut fs = rule.scan(src, &paragraphs);
            for f in fs.iter_mut() {
                f.severity = sev;
            }
            all_findings.extend(fs);
        }
        // apply inline ignores.
        filter_inline_ignores(src, &mut all_findings);

        all_findings.sort_by_key(|f| (f.start, f.end));

        if args.fix && path != "<stdin>" {
            let (patched, n) = apply_fixes(src, &all_findings);
            if n > 0 {
                if let Err(e) = std::fs::write(path, &patched) {
                    eprintln!("thin: write {}: {}", path, e);
                    return 2;
                }
                println!("{}: applied {} fix(es)", path, n);
            } else {
                println!("{}: no auto-fixable findings", path);
            }
            continue;
        }

        total_all += all_findings.len();
        total_errors += all_findings
            .iter()
            .filter(|f| f.severity == Severity::Error)
            .count();

        if json_mode {
            json_reports.push((path.clone(), all_findings));
        } else {
            print!("{}", render(path, src, &all_findings, use_color));
        }
    }

    if json_mode {
        let reports: Vec<FileReport> = json_reports
            .iter()
            .map(|(p, f)| FileReport {
                path: p,
                findings: f,
                counts: counts(f),
            })
            .collect();
        match serde_json::to_string_pretty(&reports) {
            Ok(s) => println!("{}", s),
            Err(e) => {
                eprintln!("thin: json: {}", e);
                return 2;
            }
        }
    } else if !args.fix {
        eprintln!(
            "\nthin: {} total · {} errors across {} file(s)",
            total_all,
            total_errors,
            inputs.len()
        );
    }

    if total_errors > 0 {
        1
    } else {
        0
    }
}

fn filter_inline_ignores(src: &str, findings: &mut Vec<Finding>) {
    // collect lines with ignore directives.
    let lines: Vec<&str> = src.split('\n').collect();
    let mut ignore_lines = std::collections::HashSet::new();
    let mut ignore_rules: std::collections::HashMap<usize, Vec<String>> =
        std::collections::HashMap::new();
    for (i, line) in lines.iter().enumerate() {
        let l = line.to_lowercase();
        if l.contains("thin: ignore-line") {
            ignore_lines.insert(i + 1);
        }
        if l.contains("thin: ignore-next") {
            ignore_lines.insert(i + 2);
        }
        if let Some(pos) = l.find("thin: ignore-rule ") {
            let tail = &line[pos + "thin: ignore-rule ".len()..];
            let id = tail.split_whitespace().next().unwrap_or("").to_string();
            ignore_rules.entry(i + 1).or_default().push(id.clone());
            ignore_rules.entry(i + 2).or_default().push(id);
        }
    }
    findings.retain(|f| {
        if ignore_lines.contains(&f.line) {
            return false;
        }
        if let Some(ids) = ignore_rules.get(&f.line) {
            if ids.iter().any(|i| i == f.rule_id) {
                return false;
            }
        }
        true
    });
}

fn list_rules(reg: &[Box<dyn Rule>]) {
    println!("thin rules · {} total", reg.len());
    for r in reg {
        println!(
            "  {:<45} [{}] {}",
            r.id(),
            r.category().as_str(),
            r.description()
        );
    }
}

fn show_rule(reg: &[Box<dyn Rule>], id: &str) -> i32 {
    let Some(r) = reg.iter().find(|r| r.id() == id) else {
        eprintln!("thin: no rule '{}'", id);
        return 2;
    };
    let (bad, good) = r.examples();
    println!("{} — {}", r.id(), r.name());
    println!("  category: {}", r.category().as_str());
    println!("  default:  {}", r.default_severity().as_str());
    println!("  detail:   {}", r.description());
    println!("  bad:      {}", bad);
    println!("  good:     {}", good);
    0
}

fn atty_stdout() -> bool {
    use std::io::IsTerminal;
    std::io::stdout().is_terminal()
}
