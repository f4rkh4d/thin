//! paragraph + sentence splitter. no external nlp.

/// a paragraph with its byte range in the source.
#[derive(Debug, Clone)]
pub struct Paragraph<'a> {
    pub text: &'a str,
    pub start: usize,
    pub end: usize,
}

/// a sentence with its byte range in the source.
#[derive(Debug, Clone)]
pub struct Sentence<'a> {
    pub text: &'a str,
    pub start: usize,
    pub end: usize,
}

/// split a source string into paragraphs by blank lines.
pub fn paragraphs(src: &str) -> Vec<Paragraph<'_>> {
    let mut out = Vec::new();
    let bytes = src.as_bytes();
    let mut i = 0;
    let n = bytes.len();
    while i < n {
        // skip blank lines
        while i < n && (bytes[i] == b'\n' || bytes[i] == b'\r') {
            i += 1;
        }
        if i >= n {
            break;
        }
        let start = i;
        // walk until we hit a blank line (two consecutive newlines)
        while i < n {
            if bytes[i] == b'\n' {
                // look ahead for another newline, possibly through whitespace-only line
                let mut j = i + 1;
                while j < n && (bytes[j] == b' ' || bytes[j] == b'\t' || bytes[j] == b'\r') {
                    j += 1;
                }
                if j >= n || bytes[j] == b'\n' {
                    break;
                }
            }
            i += 1;
        }
        let end = i;
        let text = &src[start..end];
        if !text.trim().is_empty() {
            out.push(Paragraph { text, start, end });
        }
    }
    out
}

/// split a paragraph into sentences. naive: period/!/? followed by space or end.
/// paragraph_start is the byte offset of `text` in the full source.
pub fn sentences<'a>(text: &'a str, paragraph_start: usize) -> Vec<Sentence<'a>> {
    let mut out = Vec::new();
    let bytes = text.as_bytes();
    let n = bytes.len();
    let mut start = 0;
    let mut i = 0;
    while i < n {
        let c = bytes[i];
        if c == b'.' || c == b'!' || c == b'?' {
            // include trailing punctuation, consume repeats
            let mut j = i + 1;
            while j < n && (bytes[j] == b'.' || bytes[j] == b'!' || bytes[j] == b'?') {
                j += 1;
            }
            // boundary if followed by whitespace or end
            if j >= n || bytes[j] == b' ' || bytes[j] == b'\n' || bytes[j] == b'\t' {
                let end = j;
                let s = &text[start..end];
                if !s.trim().is_empty() {
                    out.push(Sentence {
                        text: s,
                        start: paragraph_start + start,
                        end: paragraph_start + end,
                    });
                }
                // skip whitespace
                while j < n && (bytes[j] == b' ' || bytes[j] == b'\n' || bytes[j] == b'\t') {
                    j += 1;
                }
                start = j;
                i = j;
                continue;
            }
        }
        i += 1;
    }
    if start < n {
        let s = &text[start..];
        if !s.trim().is_empty() {
            out.push(Sentence {
                text: s,
                start: paragraph_start + start,
                end: paragraph_start + n,
            });
        }
    }
    out
}

/// count words in a string (whitespace-separated, non-empty).
pub fn word_count(s: &str) -> usize {
    s.split_whitespace().count()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn paragraphs_split_on_blank_line() {
        let src = "first para.\nstill first.\n\nsecond para.\n\nthird.";
        let p = paragraphs(src);
        assert_eq!(p.len(), 3);
        assert!(p[0].text.contains("first"));
        assert_eq!(p[2].text.trim(), "third.");
    }

    #[test]
    fn sentences_basic() {
        let p = "one sentence. two! three?";
        let s = sentences(p, 0);
        assert_eq!(s.len(), 3);
    }

    #[test]
    fn word_count_basic() {
        assert_eq!(word_count("one two three"), 3);
        assert_eq!(word_count(""), 0);
        assert_eq!(word_count("  hi  "), 1);
    }
}
