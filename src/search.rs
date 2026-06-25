//! Ripgrep-as-a-library search core. One matcher + searcher shared by every
//! search surface: bodies, skeletons, index filenames, and raw source files.

use anyhow::{anyhow, Result};
use grep::matcher::Matcher;
use grep::regex::{RegexMatcher, RegexMatcherBuilder};
use grep::searcher::sinks::UTF8;
use grep::searcher::{Searcher, SearcherBuilder};
use std::path::Path;

/// Build a matcher matching the historical semantics: a literal query is
/// case-insensitive (the old `to_lowercase().contains()`), a regex query is
/// case-sensitive (the old `Regex::is_match`).
pub fn matcher(query: &str, use_regex: bool) -> Result<RegexMatcher> {
    let pattern = if use_regex {
        query.to_string()
    } else {
        regex::escape(query)
    };
    RegexMatcherBuilder::new()
        .case_insensitive(!use_regex)
        .build(&pattern)
        .map_err(|e| anyhow!("invalid pattern: {e}"))
}

/// Match the pattern against a single string (e.g. a filename or path).
pub fn is_match(m: &RegexMatcher, text: &str) -> bool {
    m.is_match(text.as_bytes()).unwrap_or(false)
}

fn searcher() -> Searcher {
    SearcherBuilder::new().line_number(true).build()
}

fn collect(line: &str) -> String {
    line.trim_end_matches(['\n', '\r']).to_string()
}

/// Every matching line in `bytes`: `(1-based line number, line text)`.
pub fn search_bytes(m: &RegexMatcher, bytes: &[u8]) -> Vec<(u64, String)> {
    let mut hits = Vec::new();
    let _ = searcher().search_slice(
        m,
        bytes,
        UTF8(|lnum, line| {
            hits.push((lnum, collect(line)));
            Ok(true)
        }),
    );
    hits
}

/// Every matching line in a file on disk: `(1-based line number, line text)`.
pub fn search_path(m: &RegexMatcher, path: &Path) -> Vec<(u64, String)> {
    let mut hits = Vec::new();
    let _ = searcher().search_path(
        m,
        path,
        UTF8(|lnum, line| {
            hits.push((lnum, collect(line)));
            Ok(true)
        }),
    );
    hits
}
