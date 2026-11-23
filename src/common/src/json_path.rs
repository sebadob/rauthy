//! Reduced JSONPath for claim mapping, plus the claim value matcher.
//!
//! Rauthy owns this grammar; it is a documented subset of RFC 9535, not a full implementation.
//! A path is parsed at config save (which is what makes an invalid path un-storable) and compiled
//! once per provider, so the login path only evaluates it.
//!
//! ```text
//! path        := '$' segment*
//! segment     := '.' child | '.' '.' descendant | bracket
//! bracket     := '[' ws ( '*' | string | index ) ws ']'
//! child       := '*' | name
//! descendant  := '*' | name | <the bracket form of a child>
//! name        := (ASCII_ALPHA | '_' | non-ASCII) (name chars | ASCII_DIGIT)*
//! string      := "'" ... "'" | '"' ... '"'      // JSON escapes + \/ + \uXXXX
//! index       := int32, `-` selects from the end of an array
//! ```
//!
//! Not supported, and rejected at config save: filter expressions (`?...`), slices (`1:3`),
//! selector unions (`,`) and the RFC's `length`/`count`/`match`/`search` functions. Anything that
//! is not part of the grammar above is an `Err`, never a silently inert query.
//!
//! Two deliberate deviations from RFC 9535, both for claim-mapping UX:
//!
//! * A trailing wildcard on a non-collection selects the node itself: `$.role.*` matches a
//!   `"role": "engineering"` claim, the RFC selects 0 nodes. Admins do not have to know whether a
//!   claim is an array or a single value.
//! * Shorthand names may start with an uppercase letter (`$.UserGroups`), which RFC 9535 allows
//!   and `serde_json_path` 0.7 rejected.

use rauthy_error::{ErrorResponse, ErrorResponseType};
use regex::Regex;
use serde_json::Value;
use std::fmt::Write;
use std::sync::Arc;

/// The parser cursor: every `(byte position, char)` of the path, peekable so the grammar can
/// decide on the next character without copying the remaining string.
type Chars<'a> = std::iter::Peekable<std::str::CharIndices<'a>>;

/// A parsed claim path: the segments after the leading `$`, in evaluation order.
#[derive(Debug, Clone)]
pub struct JsonPath {
    segments: Vec<Segment>,
}

/// A path segment: a child selector, or the same selector as a descendant (`..`).
#[derive(Debug, Clone)]
enum Segment {
    Child(Selector),
    Descendant(Selector),
}

/// The selector of a segment.
#[derive(Debug, Clone)]
enum Selector {
    Name(String),
    Wildcard,
    Index(i64),
}

/// The claim value matcher, derived from the sigil prefix of the configured value:
/// `~x` is a regex, `^^x` a literal `^x`, `^x` a prefix, anything else an exact compare.
#[derive(Debug, Clone)]
pub enum Matcher {
    Exact(String),
    Prefix(String),
    Regex(Arc<Regex>),
}

/// The buffers one claim evaluation needs, reused across every rule of a provider so a federated
/// login allocates once instead of once per rule and per segment.
pub struct ClaimNodes<'a> {
    cur: Vec<&'a Value>,
    next: Vec<&'a Value>,
}

impl JsonPath {
    /// Parses Rauthy's reduced JSONPath subset. Everything outside the module grammar is an `Err`.
    pub fn parse(path: &str) -> Result<Self, ErrorResponse> {
        let mut it = path.char_indices().peekable();
        if it.next().map(|(_, c)| c) != Some('$') {
            return Err(path_err(path, "must start with `$`"));
        }

        let mut segments = Vec::with_capacity(4);
        while let Some(&(pos, c)) = it.peek() {
            match c {
                '.' => {
                    it.next();
                    match it.peek() {
                        None => return Err(path_err(path, "trailing `.`")),
                        Some((_, '.')) => {
                            it.next();
                            segments.push(Segment::Descendant(descendant_selector(&mut it, path)?));
                        }
                        // a `.` directly before a `[` is accepted as a separator only
                        Some((_, '[')) => {}
                        Some(_) => segments.push(Segment::Child(child_selector(&mut it, path)?)),
                    }
                }
                '[' => {
                    it.next();
                    segments.push(Segment::Child(bracket_selector(&mut it, path)?));
                }
                _ => {
                    return Err(path_err(
                        path,
                        &format!("unexpected `{c}` at position {pos}"),
                    ));
                }
            }
        }

        Ok(Self { segments })
    }

    /// Evaluates the path and leaves every selected node in `nodes`, in document order.
    pub fn eval<'a>(&self, root: &'a Value, nodes: &mut ClaimNodes<'a>) {
        nodes.cur.clear();
        nodes.cur.push(root);

        let last = self.segments.len().saturating_sub(1);
        for (idx, segment) in self.segments.iter().enumerate() {
            nodes.next.clear();
            // The trailing wildcard rule applies per node: the last segment's wildcard selects a
            // leaf node itself, so a frontier that mixes scalars and collections keeps both.
            let leaf_wildcard =
                idx == last && matches!(segment, Segment::Child(Selector::Wildcard));
            match segment {
                Segment::Child(selector) => {
                    for node in &nodes.cur {
                        selector.apply(node, &mut nodes.next, leaf_wildcard);
                    }
                }
                Segment::Descendant(selector) => {
                    // RFC 9535 §2.5.2: the descendant node list includes the node itself.
                    for node in &nodes.cur {
                        selector.apply(node, &mut nodes.next, false);
                        walk_descendants(node, selector, &mut nodes.next);
                    }
                }
            }

            std::mem::swap(&mut nodes.cur, &mut nodes.next);
        }
    }
}

impl Selector {
    /// Appends every node this selector picks from `node`. `leaf_wildcard` is the trailing
    /// wildcard rule: a wildcard on a leaf node selects that node, not its (missing) children.
    fn apply<'a>(&self, node: &'a Value, out: &mut Vec<&'a Value>, leaf_wildcard: bool) {
        match self {
            Self::Name(name) => {
                if let Value::Object(map) = node
                    && let Some(value) = map.get(name)
                {
                    out.push(value);
                }
            }
            Self::Wildcard => match node {
                Value::Object(map) => out.extend(map.values()),
                Value::Array(arr) => out.extend(arr),
                _ => {
                    if leaf_wildcard {
                        out.push(node);
                    }
                }
            },
            Self::Index(index) => {
                if let Value::Array(arr) = node {
                    let idx = if *index < 0 {
                        arr.len() as i64 + index
                    } else {
                        *index
                    };
                    if let Ok(idx) = usize::try_from(idx)
                        && let Some(value) = arr.get(idx)
                    {
                        out.push(value);
                    }
                }
            }
        }
    }
}

impl Matcher {
    /// Compiles the matcher of a configured claim value. An invalid regex is an `Err`, which makes
    /// a broken pattern a config save error naming the rule instead of a silently inert match.
    pub fn from_claim_value(value: &str) -> Result<Self, ErrorResponse> {
        // `~` is checked first so `^~x` is a literal `~x`, and `^^` before `^` so a literal
        // leading `^` can be typed by doubling it.
        if let Some(pattern) = value.strip_prefix('~') {
            let regex = Regex::new(pattern).map_err(|err| {
                ErrorResponse::new(
                    ErrorResponseType::BadRequest,
                    format!("Invalid claim value regex `{pattern}`: {err}"),
                )
            })?;
            Ok(Self::Regex(Arc::new(regex)))
        } else if let Some(rest) = value.strip_prefix("^^") {
            Ok(Self::Exact(format!("^{rest}")))
        } else if let Some(prefix) = value.strip_prefix('^') {
            Ok(Self::Prefix(prefix.to_string()))
        } else {
            Ok(Self::Exact(value.to_string()))
        }
    }

    #[inline]
    fn is_match(&self, node: &str) -> bool {
        match self {
            Self::Exact(value) => node == value,
            Self::Prefix(prefix) => node.starts_with(prefix.as_str()),
            Self::Regex(regex) => regex.is_match(node),
        }
    }
}

impl<'a> ClaimNodes<'a> {
    pub fn with_capacity(capacity: usize) -> Self {
        Self {
            cur: Vec::with_capacity(capacity),
            next: Vec::with_capacity(capacity),
        }
    }

    /// The nodes the last evaluation selected.
    pub fn selected(&self) -> &[&'a Value] {
        &self.cur
    }

    /// True when the string form of any selected node matches the matcher.
    #[inline]
    pub fn matches(&self, matcher: &Matcher, buf: &mut String) -> bool {
        self.cur
            .iter()
            .any(|node| matcher.is_match(node_str(node, buf)))
    }
}

impl Default for ClaimNodes<'_> {
    fn default() -> Self {
        Self::with_capacity(8)
    }
}

/// The claim matcher over the string form of every node the path selects: the inner `str` of a
/// `Value::String`, `Display` for everything else, so an admin typing `true` or `42` matches a
/// bool / number claim.
///
/// The full parse-and-match comparison in one call, the entry point this module's tests use to
/// assert the matcher's contract. The login path never re-parses: it evaluates the compiled rules
/// of `AuthProvider::compiled_mappings`.
///
/// # Panics
///
/// Never. Writing to a `String` cannot fail.
#[cfg(test)]
pub fn has_match(
    input: &Value,
    buf: &mut String,
    path: &str,
    value_expected: &str,
) -> Result<bool, ErrorResponse> {
    let path = JsonPath::parse(path)?;
    let matcher = Matcher::from_claim_value(value_expected)?;

    let mut nodes = ClaimNodes::default();
    path.eval(input, &mut nodes);
    for node in nodes.selected() {
        if matcher.is_match(node_str(node, buf)) {
            return Ok(true);
        }
    }

    Ok(false)
}

/// The string form of a claim node, quoted for strings. A `Value`'s own `Display` quotes strings,
/// which is why the match compares this destructured form and not `{value}`.
#[inline]
fn node_str<'a>(node: &'a Value, buf: &'a mut String) -> &'a str {
    match node {
        Value::String(s) => s.as_str(),
        _ => {
            buf.clear();
            write!(buf, "{node}").expect("Writing to a String never fails");
            buf.as_str()
        }
    }
}

fn walk_descendants<'a>(node: &'a Value, selector: &Selector, out: &mut Vec<&'a Value>) {
    match node {
        Value::Object(map) => {
            for child in map.values() {
                selector.apply(child, out, false);
                walk_descendants(child, selector, out);
            }
        }
        Value::Array(arr) => {
            for child in arr {
                selector.apply(child, out, false);
                walk_descendants(child, selector, out);
            }
        }
        _ => {}
    }
}

fn path_err(path: &str, msg: &str) -> ErrorResponse {
    ErrorResponse::new(
        ErrorResponseType::BadRequest,
        format!("Invalid JSONPath `{path}`: {msg}"),
    )
}

fn child_selector(it: &mut Chars<'_>, path: &str) -> Result<Selector, ErrorResponse> {
    if it.peek().copied().map(|(_, c)| c) == Some('*') {
        it.next();
        return Ok(Selector::Wildcard);
    }
    Ok(Selector::Name(name(it, path)?))
}

fn descendant_selector(it: &mut Chars<'_>, path: &str) -> Result<Selector, ErrorResponse> {
    match it.peek().copied().map(|(_, c)| c) {
        Some('*') => {
            it.next();
            Ok(Selector::Wildcard)
        }
        Some('[') => {
            it.next();
            bracket_selector(it, path)
        }
        Some(_) => Ok(Selector::Name(name(it, path)?)),
        None => Err(path_err(path, "dangling `..`")),
    }
}

fn bracket_selector(it: &mut Chars<'_>, path: &str) -> Result<Selector, ErrorResponse> {
    skip_ws(it);
    let selector = match it.peek().copied().map(|(_, c)| c) {
        Some('*') => {
            it.next();
            Selector::Wildcard
        }
        Some('\'' | '"') => Selector::Name(quoted_name(it, path)?),
        Some(c) if c == '-' || c.is_ascii_digit() => index(it, path)?,
        Some(c) => return Err(path_err(path, &format!("unexpected `{c}` in `[]`"))),
        None => return Err(path_err(path, "unclosed `[`")),
    };

    skip_ws(it);
    match it.next() {
        Some((_, ']')) => Ok(selector),
        _ => Err(path_err(path, "expected a closing `]`")),
    }
}

/// A shorthand name: RFC 9535's first char (ASCII alpha, `_`, non-ASCII) plus ASCII digits.
fn name(it: &mut Chars<'_>, path: &str) -> Result<String, ErrorResponse> {
    let mut name = String::with_capacity(16);
    while let Some((_, c)) = it.peek().copied() {
        let allowed = match name.len() {
            0 => c == '_' || c.is_ascii_alphabetic() || c as u32 > 127,
            _ => c == '_' || c.is_ascii_alphanumeric() || c as u32 > 127,
        };
        if !allowed {
            break;
        }
        name.push(c);
        it.next();
    }

    if name.is_empty() {
        return Err(path_err(
            path,
            "a name must start with a letter, `_` or a non-ASCII char",
        ));
    }
    Ok(name)
}

fn quoted_name(it: &mut Chars<'_>, path: &str) -> Result<String, ErrorResponse> {
    let quote = it.next().map(|(_, c)| c).unwrap_or('"');
    let mut name = String::with_capacity(16);

    while let Some((_, c)) = it.next() {
        match c {
            c if c == quote => return Ok(name),
            '\\' => match it.next() {
                Some((_, 'n')) => name.push('\n'),
                Some((_, 't')) => name.push('\t'),
                Some((_, 'b')) => name.push('\u{8}'),
                Some((_, 'f')) => name.push('\u{c}'),
                Some((_, 'r')) => name.push('\r'),
                Some((_, esc @ ('/' | '\\' | '\'' | '"'))) => name.push(esc),
                Some((_, 'u')) => name.push(unicode_escape(it, path)?),
                Some((pos, c)) => {
                    return Err(path_err(
                        path,
                        &format!("invalid escape `\\{c}` at position {pos}"),
                    ));
                }
                None => return Err(path_err(path, "unterminated quoted name")),
            },
            c if (c as u32) < 0x20 => {
                return Err(path_err(
                    path,
                    "control chars in a quoted name need an escape",
                ));
            }
            c => name.push(c),
        }
    }

    Err(path_err(path, "unterminated quoted name"))
}

fn unicode_escape(it: &mut Chars<'_>, path: &str) -> Result<char, ErrorResponse> {
    let code = hex4(it, path)?;
    if !(0xD800..=0xDBFF).contains(&code) {
        return char::from_u32(code).ok_or_else(|| path_err(path, "invalid `\\u` escape"));
    }

    // A high surrogate must be followed by the matching low surrogate, RFC 9535 §2.5.3.2.
    match (it.next(), it.next()) {
        (Some((_, '\\')), Some((_, 'u'))) => {
            let low = hex4(it, path)?;
            // Checked before the combination, so a non-surrogate `low` is an `Err` and never an
            // underflow of the subtraction below.
            if !matches!(low, 0xDC00..=0xDFFF) {
                return Err(path_err(path, "a high surrogate needs a low surrogate"));
            }
            let combined = 0x10000 + ((code - 0xD800) << 10) + (low - 0xDC00);
            char::from_u32(combined).ok_or_else(|| path_err(path, "invalid surrogate pair"))
        }
        _ => Err(path_err(path, "a high surrogate needs a low surrogate")),
    }
}

fn hex4(it: &mut Chars<'_>, path: &str) -> Result<u32, ErrorResponse> {
    let mut digits = String::with_capacity(4);
    for _ in 0..4 {
        match it.next() {
            Some((_, c)) if c.is_ascii_hexdigit() => digits.push(c),
            _ => return Err(path_err(path, "expected 4 hex digits after `\\u`")),
        }
    }
    u32::from_str_radix(&digits, 16).map_err(|_| path_err(path, "invalid `\\u` escape"))
}

fn index(it: &mut Chars<'_>, path: &str) -> Result<Selector, ErrorResponse> {
    let mut digits = String::with_capacity(8);
    if it.peek().copied().map(|(_, c)| c) == Some('-') {
        it.next();
        digits.push('-');
    }
    while let Some((_, c)) = it.peek().copied() {
        if !c.is_ascii_digit() {
            break;
        }
        digits.push(c);
        it.next();
    }

    match digits.as_str() {
        "-" => Err(path_err(path, "missing array index")),
        "-0" => Err(path_err(path, "array index cannot be `-0`")),
        d if d.len() > 1 && d.starts_with('0') => {
            Err(path_err(path, "array index cannot have leading zeros"))
        }
        d if d.len() > 2 && d.starts_with("-0") => {
            Err(path_err(path, "array index cannot have leading zeros"))
        }
        d => match d.parse::<i64>() {
            Ok(idx) => Ok(Selector::Index(idx)),
            Err(_) => Err(path_err(path, "array index out of range")),
        },
    }
}

fn skip_ws(it: &mut Chars<'_>) {
    while it
        .peek()
        .copied()
        .map(|(_, c)| c)
        .is_some_and(char::is_whitespace)
    {
        it.next();
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use serde_json::json;

    /// Every expected node count below is the `serde_json_path` 0.7 output for the same fixture:
    /// the RFC subset this parser supports must select exactly these nodes.
    fn fixture() -> Value {
        json!({
            "foo": {
                "bar": ["baz", "bop", 23],
                "bor": "yes",
                "x-groups": ["xg"]
            },
            "bor": "yup",
            "groups": ["a", "b"]
        })
    }

    fn nodes(path: &str) -> Vec<String> {
        let json = fixture();
        let path = JsonPath::parse(path).expect("valid test path");
        let mut nodes = ClaimNodes::default();
        path.eval(&json, &mut nodes);
        nodes.selected().iter().map(|v| v.to_string()).collect()
    }

    /// The **order** of selected nodes follows `serde_json`'s object key order, which the build's
    /// `preserve_order` feature decides, so each case asserts the count and the presence of a
    /// node, never its position. A rule is order-free by definition: it hits when any selected
    /// node hits.
    #[test]
    fn parity_with_the_crate_for_the_supported_subset() {
        // (path, expected node count, a node that must be selected) — the crate's measured output.
        let cases: Vec<(&str, usize, &str)> = vec![
            ("$.foo.bar[*]", 3, "\"baz\""),
            ("$.foo.*", 3, "[\"baz\",\"bop\",23]"),
            ("$.foo.bar.*", 3, "\"baz\""),
            ("$.foo.bor", 1, "\"yes\""),
            ("$.*.bor", 1, "\"yes\""),
            ("$..bor", 2, "\"yup\""),
            ("$..*", 12, "\"yup\""),
            ("$['foo']['x-groups']", 1, "[\"xg\"]"),
            ("$..['x-groups']", 1, "[\"xg\"]"),
            ("$.foo.bar[0]", 1, "\"baz\""),
            ("$.foo.bar[-1]", 1, "23"),
            ("$.groups[0]", 1, "\"a\""),
            ("$.groups[2]", 0, ""),
            ("$", 1, ""),
            ("$.foo[1]", 0, ""),
            ("$.foo['bar'][*]", 3, "\"baz\""),
            ("$..bar[*]", 3, "\"baz\""),
            ("$.groups[ 0 ]", 1, "\"a\""),
            ("$..foo.bor", 1, "\"yes\""),
            ("$.foo.*.bor", 0, ""),
        ];

        for (path, count, node) in cases {
            let got = nodes(path);
            assert_eq!(got.len(), count, "path {path} -> {got:?}");
            if !node.is_empty() {
                assert!(
                    got.iter().any(|selected| selected == node),
                    "path {path} -> {got:?} must select {node}"
                );
            }
        }
    }

    /// The documented deviation: a trailing wildcard on a leaf selects the leaf, so `$.role.*`
    /// matches a scalar claim. The crate selected 0 nodes here.
    #[test]
    fn trailing_wildcard_selects_a_leaf() {
        assert_eq!(nodes("$.foo.bor.*"), ["\"yes\""]);
        assert_eq!(nodes("$.groups.*"), ["\"a\"", "\"b\""]);
        assert_eq!(nodes("$.foo.bar[*].*"), ["\"baz\"", "\"bop\"", "23"]);
    }

    /// The trailing wildcard applies per node, so a frontier that mixes scalars and collections
    /// keeps the scalars instead of dropping them. Object key order is not defined, so the result
    /// is sorted.
    #[test]
    fn trailing_wildcard_keeps_scalars_next_to_collections() {
        let json = json!({ "a": ["x"], "b": "y", "c": { "d": "z" } });
        let path = JsonPath::parse("$.*.*").expect("valid path");
        let mut nodes = ClaimNodes::default();
        path.eval(&json, &mut nodes);
        let mut got: Vec<String> = nodes.selected().iter().map(ToString::to_string).collect();
        got.sort();
        assert_eq!(got, ["\"x\"", "\"y\"", "\"z\""]);
    }

    #[test]
    fn rejects_everything_outside_the_grammar() {
        let bad = [
            "",
            "foo",
            "$foo",
            "$.",
            "$..",
            "$...",
            "$.foo[",
            "$.foo[]",
            "$.foo[0:2]",
            "$.foo[?@.a]",
            "$.foo[?search(@, \"x\")]",
            "$.foo.bar[1],$.foo.bor",
            "$.foo.bar[01]",
            "$.foo.bar[-0]",
            "$.foo.bar[1x]",
            "$['a]",
            "$['a\\x']",
            "$['\\uD800']",
            "$['\\uD800A']",
            "$['\\ud83d\\u0041']",
            "$['\\ud83d\\ud83d']",
            "$['a\nb']",
            "$..['a]",
        ];
        for path in bad {
            assert!(
                JsonPath::parse(path).is_err(),
                "path {path:?} must not parse"
            );
        }
    }

    /// Paths the crate rejected in shorthand form that RFC 9535 allows, and that Rauthy needs for
    /// claims like `$.UserGroups`.
    #[test]
    fn accepts_names_the_crate_rejected() {
        assert_eq!(nodes("$.bor"), ["\"yup\""]);
        let json = json!({ "UserGroups": ["a"], "é": { "x": 1 } });
        let path = JsonPath::parse("$.UserGroups.*").expect("valid path");
        let mut nodes = ClaimNodes::default();
        path.eval(&json, &mut nodes);
        assert_eq!(nodes.selected().len(), 1);

        let path = JsonPath::parse("$['é'].x").expect("valid path");
        path.eval(&json, &mut nodes);
        assert_eq!(nodes.selected()[0].to_string(), "1");
    }

    /// Quoted names carry the JSON escapes, including `\u` and surrogate pairs, so a key like
    /// `x-groups` or a key with a space is addressable.
    #[test]
    fn quoted_name_escapes() {
        let json = json!({ "a b": 1, "a\nb": 2, "x-groups": 3, "é": 4, "😀": 5, "\"": 6 });
        for (path, expected) in [
            ("$['a b']", "1"),
            ("$['a\\nb']", "2"),
            ("$['x-groups']", "3"),
            ("$['\\u00e9']", "4"),
            ("$['\\ud83d\\ude00']", "5"),
            ("$[\"\\\"\"]", "6"),
        ] {
            let parsed = JsonPath::parse(path).expect("valid path");
            let mut nodes = ClaimNodes::default();
            parsed.eval(&json, &mut nodes);
            assert_eq!(
                nodes.selected().first().map(|v| v.to_string()),
                Some(expected.to_string()),
                "path {path}"
            );
        }
    }

    fn claims() -> Value {
        serde_json::from_str(
            r#"{
                "groups": ["tgchat:member", "tgchat:admin", "engineering", 42, true, null],
                "role": "engineering",
                "mfa": true,
                "count": 42,
                "odd": "42"
            }"#,
        )
        .expect("valid test json")
    }

    /// The parity oracle of the matcher fix: `serde_json_path` 0.7's comparison stringified both
    /// sides through `serde_json`, so a string claim gained quotes on the node and on the config
    /// value. String claims must keep matching exactly like they did with the crate.
    fn crate_match(input: &Value, path: &str, value_expected: &str) -> bool {
        let expected = Value::from(value_expected).to_string();
        let parsed = JsonPath::parse(path).expect("valid test path");
        let mut nodes = ClaimNodes::default();
        parsed.eval(input, &mut nodes);
        nodes.selected().iter().any(|node| {
            let quoted = if node.is_string() {
                node.to_string()
            } else {
                format!("\"{node}\"")
            };
            quoted == expected
        })
    }

    #[test]
    fn parity_with_main_for_string_claims() {
        let json = claims();
        let mut buf = String::new();
        for (path, value) in [
            ("$.role", "engineering"),
            ("$.groups.*", "engineering"),
            ("$.groups.*", "tgchat:admin"),
            ("$.groups.*", "tgchat:missing"),
            ("$.role", "Engineering"),
            ("$.odd", "42"),
            ("$.missing", "engineering"),
        ] {
            let mine = has_match(&json, &mut buf, path, value).expect("valid path");
            let main = crate_match(&json, path, value);
            assert_eq!(mine, main, "path: {path}, value: {value}");
        }
    }

    /// Non-string claims: the crate matched them by quoting the node to look like a JSON string,
    /// so an admin typing the raw value (`true`, `42`) gets a hit today.
    #[test]
    fn parity_with_main_for_non_string_claims() {
        let json = claims();
        let mut buf = String::new();
        for (path, value) in [
            ("$.mfa", "true"),
            ("$.count", "42"),
            ("$.groups.*", "true"),
            ("$.groups.*", "42"),
            ("$.groups.*", "null"),
        ] {
            let mine = has_match(&json, &mut buf, path, value).expect("valid path");
            let main = crate_match(&json, path, value);
            assert_eq!(mine, main, "path: {path}, value: {value}");
        }
    }

    /// Matcher sigils: plain = exact, `^` = prefix, `~` = regex, `^^` = a literal `^`.
    #[test]
    fn matcher_sigils() {
        let json = claims();
        let mut buf = String::new();

        for (value, expected) in [
            ("tgchat:admin", true),
            ("^tgchat:", true),
            ("^tgchat:admin", true),
            ("^admin", false),
            ("~^tgchat:.*", true),
            ("~tgchat:[a-z]+", true),
            ("~[", false), // invalid regex -> Err, checked below
            ("^^literal", false),
            ("engineering", true),
        ] {
            let got = has_match(&json, &mut buf, "$.groups.*", value);
            if value == "~[" {
                assert!(got.is_err(), "an invalid regex must be an Err");
            } else {
                assert_eq!(got.expect("valid matcher"), expected, "value: {value}");
            }
        }

        // `^^x` is the literal `^x`, `^~x` is the literal `~x`, and `~x` is the pattern `x` with
        // the sigil stripped — not a search for a leading `~`.
        let json = json!({ "values": ["^literal", "~tilde"] });
        assert!(has_match(&json, &mut buf, "$.values.*", "^^literal").expect("ok"));
        assert!(has_match(&json, &mut buf, "$.values.*", "^~tilde").expect("ok"));
        assert!(has_match(&json, &mut buf, "$.values.*", "~tilde").expect("ok"));
        assert!(!has_match(&json, &mut buf, "$.values.*", "~^tilde$").expect("ok"));
    }

    /// A regex rule is evaluated per node the path selects.
    #[test]
    fn regex_matcher_per_node() {
        let json = json!({ "groups": ["tgchat:member", "tgchat:admin", "other"] });
        let mut buf = String::new();
        assert!(has_match(&json, &mut buf, "$.groups.*", "~^tgchat:(admin|member)$").expect("ok"));
        assert!(!has_match(&json, &mut buf, "$.groups[0]", "~^tgchat:admin$").expect("ok"));
        assert!(has_match(&json, &mut buf, "$.groups[1]", "~^tgchat:admin$").expect("ok"));
    }

    /// A rule set with duplicate paths must never hold a frontier across a second evaluation.
    #[test]
    fn repeated_evaluation_with_shared_buffers() {
        let json = claims();
        let mut nodes = ClaimNodes::default();
        let mut buf = String::new();
        let path = JsonPath::parse("$.groups.*").expect("valid path");
        for _ in 0..3 {
            path.eval(&json, &mut nodes);
            assert_eq!(nodes.selected().len(), 6);
            assert!(
                Matcher::from_claim_value("^tgchat:")
                    .expect("valid")
                    .is_match(node_str(nodes.selected()[0], &mut buf))
            );
        }
    }
}
