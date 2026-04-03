//! colored inline output.

use crate::rules::{Finding, Severity};
use anstyle::{AnsiColor, Style};
use std::fmt::Write;

pub fn render(path: &str, src: &str, findings: &[Finding], use_color: bool) -> String {
    let mut out = String::new();
    let header = Style::new().bold();
    if use_color {
        writeln!(out, "{}{}{}", header.render(), path, header.render_reset()).unwrap();
    } else {
        writeln!(out, "{}", path).unwrap();
    }

    if findings.is_empty() {
        let s = Style::new().fg_color(Some(AnsiColor::Green.into()));
        if use_color {
            writeln!(out, "  {}clean.{}", s.render(), s.render_reset()).unwrap();
        } else {
            writeln!(out, "  clean.").unwrap();
        }
        return out;
    }

    let lines: Vec<&str> = src.split('\n').collect();
    for f in findings {
        let line_idx = f.line.saturating_sub(1);
        let line = lines.get(line_idx).copied().unwrap_or("");
        let col = f.col.saturating_sub(1);
        let span = (f.end - f.start).max(1);
        let sev_style = match f.severity {
            Severity::Error => Style::new().fg_color(Some(AnsiColor::Red.into())).bold(),
            Severity::Warning => Style::new().fg_color(Some(AnsiColor::Yellow.into())).bold(),
            Severity::Info => Style::new().fg_color(Some(AnsiColor::Blue.into())).bold(),
        };
        let dim = Style::new().dimmed();
        let gutter = format!("{:>4}", f.line);
        if use_color {
            writeln!(
                out,
                "  {}{}{} │ {}",
                dim.render(),
                gutter,
                dim.render_reset(),
                line
            )
            .unwrap();
            let pad = " ".repeat(col);
            let underline: String = "~".repeat(span.min(line.len().saturating_sub(col).max(1)));
            writeln!(
                out,
                "       │ {pad}{}{}{}",
                sev_style.render(),
                underline,
                sev_style.render_reset()
            )
            .unwrap();
            writeln!(
                out,
                "       │ {pad}{}{}{} · {} {}[{}]{}",
                sev_style.render(),
                f.rule_id,
                sev_style.render_reset(),
                f.message,
                dim.render(),
                f.severity.as_str(),
                dim.render_reset(),
            )
            .unwrap();
        } else {
            writeln!(out, "  {} │ {}", gutter, line).unwrap();
            let pad = " ".repeat(col);
            let underline: String = "~".repeat(span.min(line.len().saturating_sub(col).max(1)));
            writeln!(out, "       │ {pad}{}", underline).unwrap();
            writeln!(
                out,
                "       │ {pad}{} · {} [{}]",
                f.rule_id,
                f.message,
                f.severity.as_str()
            )
            .unwrap();
        }
    }

    // summary
    let errors = findings
        .iter()
        .filter(|f| f.severity == Severity::Error)
        .count();
    let warnings = findings
        .iter()
        .filter(|f| f.severity == Severity::Warning)
        .count();
    let infos = findings
        .iter()
        .filter(|f| f.severity == Severity::Info)
        .count();
    writeln!(
        out,
        "  {} issues · {} errors · {} warnings · {} info",
        findings.len(),
        errors,
        warnings,
        infos
    )
    .unwrap();
    out
}
