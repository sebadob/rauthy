use crate::directory::Entry;
use crate::dn;
use ldap3_proto::proto::LdapSubstringFilter;
use ldap3_proto::simple::LdapFilter;

/// Attributes holding DNs, compared in normalized form.
const DN_ATTRS: [&str; 3] = ["member", "memberof", "uniquemember"];

/// Matches a search filter against an entry. Attribute names and values are case-insensitive.
/// Unsupported filter types (ordering, approx, extensible) never match, also not when negated.
pub fn matches(filter: &LdapFilter, entry: &Entry) -> bool {
    eval(filter, entry).unwrap_or(false)
}

/// Three-valued evaluation (RFC 4511 4.5.1.7): `None` is Undefined, which `not` keeps.
fn eval(filter: &LdapFilter, entry: &Entry) -> Option<bool> {
    let res = match filter {
        LdapFilter::And(fs) => {
            let vals: Vec<_> = fs.iter().map(|f| eval(f, entry)).collect();
            if vals.contains(&Some(false)) {
                false
            } else if vals.contains(&None) {
                return None;
            } else {
                true
            }
        }
        LdapFilter::Or(fs) => {
            let vals: Vec<_> = fs.iter().map(|f| eval(f, entry)).collect();
            if vals.contains(&Some(true)) {
                true
            } else if vals.contains(&None) {
                return None;
            } else {
                false
            }
        }
        LdapFilter::Not(f) => !eval(f, entry)?,
        LdapFilter::Present(attr) => entry.values(attr).next().is_some(),
        LdapFilter::Equality(attr, val) => {
            if DN_ATTRS.contains(&attr.to_lowercase().as_str()) {
                let val = dn::normalize(val);
                entry.values(attr).any(|v| dn::normalize(v) == val)
            } else {
                let val = val.to_lowercase();
                entry.values(attr).any(|v| v.to_lowercase() == val)
            }
        }
        LdapFilter::Substring(attr, sub) => entry
            .values(attr)
            .any(|v| substring(&v.to_lowercase(), sub)),
        _ => return None,
    };
    Some(res)
}

fn substring(mut rest: &str, sub: &LdapSubstringFilter) -> bool {
    if let Some(initial) = &sub.initial {
        match rest.strip_prefix(initial.to_lowercase().as_str()) {
            Some(r) => rest = r,
            None => return false,
        }
    }
    if let Some(final_) = &sub.final_ {
        match rest.strip_suffix(final_.to_lowercase().as_str()) {
            Some(r) => rest = r,
            None => return false,
        }
    }
    for any in &sub.any {
        let any = any.to_lowercase();
        match rest.find(&any) {
            Some(pos) => rest = &rest[pos + any.len()..],
            None => return false,
        }
    }
    true
}

#[cfg(test)]
mod tests {
    use super::*;
    use ldap3_proto::parse_ldap_filter_str;

    fn entry() -> Entry {
        Entry {
            dn: "uid=alice,ou=people,dc=x".into(),
            attrs: vec![
                ("objectClass", vec!["inetOrgPerson".into(), "top".into()]),
                ("uid", vec!["alice".into()]),
                ("mail", vec!["Alice@Example.com".into()]),
                ("memberOf", vec!["cn=admins,ou=groups,dc=x".into()]),
            ],
        }
    }

    fn m(f: &str) -> bool {
        matches(&parse_ldap_filter_str(f).unwrap(), &entry())
    }

    #[test]
    fn filters() {
        assert!(m("(objectClass=*)"));
        assert!(m("(OBJECTCLASS=InetOrgPerson)"));
        assert!(m(
            "(&(objectClass=inetOrgPerson)(|(uid=bob)(mail=alice@example.com)))"
        ));
        assert!(!m("(&(uid=alice)(!(mail=alice@example.com)))"));
        let member_of = |v: &str| LdapFilter::Equality("memberOf".into(), v.into());
        assert!(matches(&member_of("CN=Admins, OU=groups, DC=x"), &entry()));
        assert!(!matches(&member_of("cn=users,ou=groups,dc=x"), &entry()));
        assert!(!m("(sn=*)"));
        assert!(m("(mail=ali*)"));
        assert!(m("(mail=*@example.com)"));
        assert!(m("(mail=a*ce*ex*com)"));
        assert!(!m("(mail=a*xyz*)"));
        assert!(!m("(mail=alice@example.com*m)"));
        assert!(!m("(uid>=a)"));
        assert!(!m("(!(uid>=a))"));
        assert!(!m("(&(uid=alice)(!(uid>=a)))"));
        assert!(m("(|(uid=alice)(uid>=a))"));
        assert!(!m("(!(|(uid=bob)(uid>=a)))"));
        assert!(m("(!(&(uid=bob)(uid>=a)))"));
    }
}
