//! DN parsing and building for the fixed directory layout:
//!
//! ```text
//! cn=<client_id>,ou=clients,<base_dn>
//!   uid=<uid>,ou=people,cn=<client_id>,ou=clients,<base_dn>
//!   cn=<group>,ou=groups,cn=<client_id>,ou=clients,<base_dn>
//! ```

/// A parsed bind DN.
#[derive(Debug, PartialEq)]
pub enum BindDn {
    Client(String),
    User { client_id: String, uid: String },
}

/// Splits a DN into `(lowercase attribute, value)` RDNs and resolves `\` escapes.
/// Returns `None` for malformed or multi-valued RDNs.
pub fn parse(dn: &str) -> Option<Vec<(String, String)>> {
    if dn.trim().is_empty() {
        return Some(Vec::new());
    }

    let mut rdns = Vec::new();
    let mut key = String::new();
    let mut val: Vec<u8> = Vec::new();
    let mut in_val = false;
    let mut bytes = dn.bytes();

    while let Some(b) = bytes.next() {
        match b {
            b'\\' => {
                let c = bytes.next()?;
                if c.is_ascii_hexdigit() {
                    let lo = bytes.next()?;
                    let hex = [c, lo];
                    val.push(u8::from_str_radix(std::str::from_utf8(&hex).ok()?, 16).ok()?);
                } else {
                    val.push(c);
                }
            }
            b'=' if !in_val => in_val = true,
            b',' if in_val => {
                rdns.push(rdn(&key, &val)?);
                key.clear();
                val.clear();
                in_val = false;
            }
            b'+' if in_val => return None,
            _ if in_val => val.push(b),
            _ => key.push(b as char),
        }
    }
    if !in_val {
        return None;
    }
    rdns.push(rdn(&key, &val)?);

    Some(rdns)
}

fn rdn(key: &str, val: &[u8]) -> Option<(String, String)> {
    let key = key.trim().to_lowercase();
    let val = String::from_utf8(val.to_vec()).ok()?.trim().to_string();
    if key.is_empty() || val.is_empty() {
        return None;
    }
    Some((key, val))
}

/// Escapes an RDN value (RFC 4514).
pub fn escape(v: &str) -> String {
    let mut out = String::with_capacity(v.len());
    let last = v.chars().count().saturating_sub(1);
    for (i, c) in v.chars().enumerate() {
        let edge = (i == 0 && (c == ' ' || c == '#')) || (i == last && c == ' ');
        if edge || matches!(c, ',' | '+' | '"' | '\\' | '<' | '>' | ';' | '=') {
            out.push('\\');
        }
        out.push(c);
    }
    out
}

/// Canonical form for comparisons: lowercase, no extra whitespace, consistent escaping.
pub fn normalize(dn: &str) -> Option<String> {
    Some(
        parse(dn)?
            .into_iter()
            .map(|(k, v)| format!("{k}={}", escape(&v.to_lowercase())))
            .collect::<Vec<_>>()
            .join(","),
    )
}

/// `true` if the normalized `dn` equals `base` or is below it.
pub fn is_within(dn: &str, base: &str) -> bool {
    base.is_empty() || dn == base || dn.ends_with(&format!(",{base}"))
}

/// Number of RDNs of a normalized DN.
pub fn depth(dn: &str) -> usize {
    parse(dn).map(|r| r.len()).unwrap_or(0)
}

pub fn client_base(client_id: &str, base_dn: &str) -> String {
    format!("cn={},ou=clients,{base_dn}", escape(client_id))
}

pub fn user_dn(uid: &str, client_base: &str) -> String {
    format!("uid={},ou=people,{client_base}", escape(uid))
}

pub fn group_dn(name: &str, client_base: &str) -> String {
    format!("cn={},ou=groups,{client_base}", escape(name))
}

/// Parses a bind DN below `base_dn` (which must be lowercase and unescaped-safe).
pub fn parse_bind(dn: &str, base_dn: &str) -> Option<BindDn> {
    let rdns = parse(dn)?;
    let base = parse(base_dn)?;
    if rdns.len() < base.len() {
        return None;
    }
    let (head, tail) = rdns.split_at(rdns.len() - base.len());
    let base_matches = tail
        .iter()
        .zip(base.iter())
        .all(|((k, v), (bk, bv))| k == bk && v.eq_ignore_ascii_case(bv));
    if !base_matches {
        return None;
    }

    let attr = |i: usize, key: &str| head.get(i).filter(|(k, _)| k == key).map(|(_, v)| v);
    let ou = |i: usize, name: &str| attr(i, "ou").is_some_and(|v| v.eq_ignore_ascii_case(name));

    match head.len() {
        2 if ou(1, "clients") => Some(BindDn::Client(attr(0, "cn")?.clone())),
        4 if ou(1, "people") && ou(3, "clients") => Some(BindDn::User {
            client_id: attr(2, "cn")?.clone(),
            uid: attr(0, "uid")?.clone(),
        }),
        _ => None,
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    const BASE: &str = "dc=rauthy,dc=local";

    #[test]
    fn bind_dns() {
        assert_eq!(
            parse_bind("cn=jellyfin,ou=clients,dc=rauthy,dc=local", BASE),
            Some(BindDn::Client("jellyfin".into()))
        );
        assert_eq!(
            parse_bind(
                "UID=Alice, ou=People, CN=jellyfin, OU=Clients, DC=Rauthy, DC=local",
                BASE
            ),
            Some(BindDn::User {
                client_id: "jellyfin".into(),
                uid: "Alice".into()
            })
        );
        assert_eq!(
            parse_bind(&user_dn("a+b@x.io", &client_base("c", BASE)), BASE),
            Some(BindDn::User {
                client_id: "c".into(),
                uid: "a+b@x.io".into()
            })
        );
        assert_eq!(parse_bind("cn=x,ou=clients,dc=other", BASE), None);
        assert_eq!(parse_bind("cn=x,ou=people,dc=rauthy,dc=local", BASE), None);
        assert_eq!(parse_bind("", BASE), None);
        assert_eq!(
            parse_bind("cn=a+uid=b,ou=clients,dc=rauthy,dc=local", BASE),
            None
        );
    }

    #[test]
    fn escaping() {
        assert_eq!(escape("a,b+c"), "a\\,b\\+c");
        assert_eq!(escape(" a "), "\\ a\\ ");
        assert_eq!(parse("cn=a\\2cb,dc=x").unwrap()[0].1, "a,b");
        assert_eq!(parse("cn=a\\,b,dc=x").unwrap()[0].1, "a,b");
        assert_eq!(parse("cn"), None);
    }

    #[test]
    fn scope_helpers() {
        let base = normalize("cn=c, ou=Clients,dc=rauthy,dc=local").unwrap();
        assert_eq!(base, "cn=c,ou=clients,dc=rauthy,dc=local");
        let user = normalize(&user_dn("bob", &base)).unwrap();
        assert!(is_within(&user, &base));
        assert!(!is_within(&base, &user));
        assert!(!is_within("cn=xc,ou=clients,dc=rauthy,dc=local", &base));
        assert_eq!(depth(&user) - depth(&base), 2);
    }
}
