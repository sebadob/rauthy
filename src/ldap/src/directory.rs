use crate::dn;
use ldap3_proto::simple::{LdapPartialAttribute, LdapSearchResultEntry};
use rauthy_data::entity::clients::Client;
use rauthy_data::entity::groups::Group;
use rauthy_data::entity::pam::users::PamUser;
use rauthy_data::entity::users::User;
use rauthy_data::entity::users_values::{UidCandidate, UserValues};
use rauthy_error::ErrorResponse;
use std::collections::HashMap;
use tracing::warn;

pub const OID_WHOAMI: &str = "1.3.6.1.4.1.4203.1.11.3";

pub struct Entry {
    pub dn: String,
    pub attrs: Vec<(&'static str, Vec<String>)>,
}

impl Entry {
    pub fn values(&self, attr: &str) -> impl Iterator<Item = &String> {
        self.attrs
            .iter()
            .filter(move |(name, _)| name.eq_ignore_ascii_case(attr))
            .flat_map(|(_, vals)| vals)
    }

    /// Converts into a result entry with only the `requested` attributes.
    pub fn into_result(self, requested: &[String]) -> LdapSearchResultEntry {
        let all = requested.is_empty() || requested.iter().any(|a| a == "*" || a == "+");
        let attributes = self
            .attrs
            .into_iter()
            .filter(|(name, vals)| {
                !vals.is_empty() && (all || requested.iter().any(|r| r.eq_ignore_ascii_case(name)))
            })
            .map(|(name, vals)| LdapPartialAttribute {
                atype: name.to_string(),
                vals: vals.into_iter().map(String::into_bytes).collect(),
            })
            .collect();
        LdapSearchResultEntry {
            dn: self.dn,
            attributes,
        }
    }
}

pub struct PosixAttrs {
    pub uid_number: u32,
    pub gid_number: u32,
    pub home: String,
    pub shell: String,
}

pub struct DirUser {
    pub id: String,
    pub uid: String,
    pub email: String,
    pub given_name: String,
    pub family_name: Option<String>,
    pub groups: Vec<String>,
    pub posix: Option<PosixAttrs>,
}

pub fn root_dse(base_dn: &str) -> Entry {
    Entry {
        dn: String::new(),
        attrs: vec![
            ("objectClass", vec!["top".into()]),
            ("namingContexts", vec![base_dn.to_string()]),
            ("supportedLDAPVersion", vec!["3".into()]),
            ("supportedExtension", vec![OID_WHOAMI.into()]),
            ("vendorName", vec!["Rauthy".into()]),
        ],
    }
}

/// Builds all entries of one client's subtree.
pub fn build(
    client_id: &str,
    client_base: &str,
    users: &[DirUser],
    groups: &[String],
) -> Vec<Entry> {
    let ou = |name: &str| Entry {
        dn: format!("ou={name},{client_base}"),
        attrs: vec![
            (
                "objectClass",
                vec!["top".into(), "organizationalUnit".into()],
            ),
            ("ou", vec![name.into()]),
        ],
    };

    let mut entries = vec![
        Entry {
            dn: client_base.to_string(),
            attrs: vec![
                (
                    "objectClass",
                    vec!["top".into(), "applicationProcess".into()],
                ),
                ("cn", vec![client_id.into()]),
            ],
        },
        ou("people"),
        ou("groups"),
    ];

    for u in users {
        let name = match &u.family_name {
            Some(f) => format!("{} {f}", u.given_name),
            None => u.given_name.clone(),
        };
        let mut object_class = vec![
            "top".into(),
            "person".into(),
            "organizationalPerson".into(),
            "inetOrgPerson".into(),
        ];
        let mut attrs = vec![
            ("uid", vec![u.uid.clone()]),
            ("entryUUID", vec![u.id.clone()]),
            ("mail", vec![u.email.clone()]),
            ("cn", vec![name.clone()]),
            ("displayName", vec![name]),
            ("givenName", vec![u.given_name.clone()]),
            ("sn", u.family_name.clone().into_iter().collect()),
            (
                "memberOf",
                u.groups
                    .iter()
                    .filter(|g| groups.contains(g))
                    .map(|g| dn::group_dn(g, client_base))
                    .collect(),
            ),
        ];
        if let Some(p) = &u.posix {
            object_class.push("posixAccount".into());
            attrs.extend([
                ("uidNumber", vec![p.uid_number.to_string()]),
                ("gidNumber", vec![p.gid_number.to_string()]),
                ("homeDirectory", vec![p.home.clone()]),
                ("loginShell", vec![p.shell.clone()]),
            ]);
        }
        attrs.insert(0, ("objectClass", object_class));
        entries.push(Entry {
            dn: dn::user_dn(&u.uid, client_base),
            attrs,
        });
    }

    for g in groups {
        let members: Vec<String> = users
            .iter()
            .filter(|u| u.groups.contains(g))
            .map(|u| dn::user_dn(&u.uid, client_base))
            .collect();
        entries.push(Entry {
            dn: dn::group_dn(g, client_base),
            attrs: vec![
                (
                    "objectClass",
                    vec![
                        "top".into(),
                        "groupOfNames".into(),
                        "groupOfUniqueNames".into(),
                    ],
                ),
                ("cn", vec![g.clone()]),
                ("member", members.clone()),
                ("uniqueMember", members),
            ],
        });
    }

    entries
}

/// Loads the users and groups visible to `client`.
/// Users must have a local password, be enabled, not expired and pass the client's group
/// restriction.
///
/// shortcut: loads all users on every search, fine for small directories; add caching or
/// paging if large directories need it. Binds use targeted queries, see [resolve_email].
pub async fn load(client: &Client) -> Result<(Vec<DirUser>, Vec<String>), ErrorResponse> {
    let groups: Vec<String> = Group::find_all()
        .await?
        .into_iter()
        .map(|g| g.name)
        .filter(|name| {
            client
                .restrict_group_prefix
                .as_ref()
                .is_none_or(|p| name.starts_with(p))
        })
        .collect();

    let (all, pam, mut uids) = load_uids().await?;
    let users = all
        .into_iter()
        .filter(|u| {
            u.password.is_some()
                && u.enabled
                && u.check_expired().is_ok()
                && client.validate_user_groups(u).is_ok()
        })
        .map(|u| {
            let p = pam.get(&u.email);
            DirUser {
                groups: u.get_groups(),
                posix: p.map(|p| PosixAttrs {
                    uid_number: p.id,
                    gid_number: p.gid,
                    home: p.home_dir(),
                    shell: p.shell.clone(),
                }),
                uid: uids.remove(&u.id).unwrap_or_else(|| u.email.clone()),
                id: u.id,
                email: u.email,
                given_name: u.given_name,
                family_name: u.family_name,
            }
        })
        .collect();

    Ok((users, groups))
}

/// Resolves a bind `uid` to the user's email with the same `uid`s the directory publishes.
/// Anything that is not a published `uid` is treated as the email itself.
pub async fn resolve_email(uid: &str) -> Result<String, ErrorResponse> {
    let uid_lc = uid.to_lowercase();
    let mut rows = UserValues::find_uid_candidates(&uid_lc).await?;
    // a candidate's `uid` also depends on the users that share one of its other names
    let names: Vec<String> = rows
        .iter()
        .flat_map(|(_, _, pam, pref)| [pam, pref])
        .flatten()
        .map(|n| n.to_lowercase())
        .filter(|n| *n != uid_lc)
        .collect();
    for name in names {
        rows.extend(UserValues::find_uid_candidates(&name).await?);
    }
    Ok(uid_owner(&uid_lc, rows).unwrap_or_else(|| uid.to_string()))
}

/// Returns the email of the user in `rows` whose `uid` is `uid_lc`. `rows` must contain every
/// user that shares a name or email with a user named `uid_lc`.
fn uid_owner(uid_lc: &str, mut rows: Vec<UidCandidate>) -> Option<String> {
    rows.sort_by(|a, b| a.0.cmp(&b.0));
    rows.dedup_by(|a, b| a.0 == b.0);
    let ids: Vec<(String, String)> = rows.iter().map(|r| (r.0.clone(), r.1.clone())).collect();
    let pam_names = rows
        .iter()
        .filter_map(|r| Some((r.1.clone(), r.2.clone()?)))
        .collect();
    let preferred = rows
        .iter()
        .filter_map(|r| Some((r.0.clone(), r.3.clone()?)))
        .collect();
    let uids = assign_uids(&ids, &pam_names, &preferred);
    rows.into_iter()
        .find(|r| uids[&r.0].to_lowercase() == uid_lc)
        .map(|r| r.1)
}

type Uids = (Vec<User>, HashMap<String, PamUser>, HashMap<String, String>);

/// Loads all users, their PAM users by email, and their `uid`s by user ID.
async fn load_uids() -> Result<Uids, ErrorResponse> {
    let users = User::find_all().await?;
    let pam: HashMap<String, PamUser> = PamUser::find_all()
        .await?
        .into_iter()
        .map(|p| (p.email.clone(), p))
        .collect();
    let pam_names = pam
        .iter()
        .map(|(e, p)| (e.clone(), p.name.clone()))
        .collect();
    let preferred = UserValues::find_all_preferred_usernames()
        .await?
        .into_iter()
        .collect();
    let ids: Vec<(String, String)> = users
        .iter()
        .map(|u| (u.id.clone(), u.email.clone()))
        .collect();
    let uids = assign_uids(&ids, &pam_names, &preferred);
    Ok((users, pam, uids))
}

/// Assigns each `(id, email)` its `uid`: the PAM username, else the `preferred_username`, else
/// the email. A PAM name wins over the same `preferred_username`. A name that is not unique
/// case-insensitively, among names of its kind and all emails, falls back to the email, because
/// `uid` filters are case-insensitive and apps would otherwise pick the wrong user.
fn assign_uids(
    users: &[(String, String)],
    pam_names: &HashMap<String, String>,
    preferred: &HashMap<String, String>,
) -> HashMap<String, String> {
    let emails: HashMap<String, &str> = users
        .iter()
        .map(|(id, email)| (email.to_lowercase(), id.as_str()))
        .collect();
    let count = |names: &mut dyn Iterator<Item = &String>| {
        let mut counts: HashMap<String, usize> = HashMap::new();
        for n in names {
            *counts.entry(n.to_lowercase()).or_default() += 1;
        }
        counts
    };
    let pam_count = count(&mut pam_names.values());
    let pref_count = count(&mut preferred.values());
    let unique = |id: &str, name: &str, counts: &HashMap<String, usize>| {
        let lc = name.to_lowercase();
        counts.get(&lc) == Some(&1) && emails.get(&lc).is_none_or(|owner| *owner == id)
    };

    users
        .iter()
        .map(|(id, email)| {
            let pam = pam_names.get(email);
            let pref = preferred.get(id);
            let uid = pam
                .filter(|n| unique(id, n, &pam_count))
                .or_else(|| {
                    pref.filter(|n| {
                        !pam_count.contains_key(&n.to_lowercase()) && unique(id, n, &pref_count)
                    })
                })
                .cloned()
                .unwrap_or_else(|| {
                    if let Some(name) = pam.or(pref) {
                        warn!("LDAP uid '{name}' is not unique, using the email of user {id}");
                    }
                    email.clone()
                });
            (id.clone(), uid)
        })
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn uids() {
        let s = |v: &str| v.to_string();
        let users = vec![
            (s("1"), s("a@x.io")),
            (s("2"), s("b@x.io")),
            (s("3"), s("c@x.io")),
            (s("4"), s("d@x.io")),
            (s("5"), s("e@x.io")),
            (s("6"), s("f@x.io")),
        ];
        let pam = HashMap::from([(s("a@x.io"), s("alice")), (s("f@x.io"), s("frank"))]);
        let preferred = HashMap::from([
            (s("2"), s("Alice")), // loses against the PAM name
            (s("3"), s("Carol")),
            (s("4"), s("carol")),  // case-insensitive duplicate, both fall back
            (s("5"), s("B@x.io")), // another user's email
            (s("6"), s("eve")),    // the PAM name wins for the same user
        ]);
        let uids = assign_uids(&users, &pam, &preferred);
        assert_eq!(uids["1"], "alice");
        assert_eq!(uids["2"], "b@x.io");
        assert_eq!(uids["3"], "c@x.io");
        assert_eq!(uids["4"], "d@x.io");
        assert_eq!(uids["5"], "e@x.io");
        assert_eq!(uids["6"], "frank");

        let rows: Vec<UidCandidate> = users
            .iter()
            .map(|(id, email)| {
                (
                    id.clone(),
                    email.clone(),
                    pam.get(email).cloned(),
                    preferred.get(id).cloned(),
                )
            })
            .collect();
        let owner = |uid: &str| uid_owner(uid, rows.clone());
        assert_eq!(owner("alice").as_deref(), Some("a@x.io"));
        assert_eq!(owner("frank").as_deref(), Some("f@x.io"));
        assert_eq!(owner("b@x.io").as_deref(), Some("b@x.io"));
        assert_eq!(owner("carol"), None);
        assert_eq!(owner("eve"), None);
        // only the rows the queries return for `alice`: the users named or emailed `alice`
        assert_eq!(
            uid_owner("alice", vec![rows[0].clone(), rows[1].clone()]).as_deref(),
            Some("a@x.io")
        );
    }

    #[test]
    fn entries() {
        let base = dn::client_base("app", "dc=x");
        let users = vec![
            DirUser {
                id: "id1".into(),
                uid: "alice".into(),
                email: "alice@x.io".into(),
                given_name: "Alice".into(),
                family_name: Some("Liddell".into()),
                groups: vec!["admins".into(), "hidden".into()],
                posix: Some(PosixAttrs {
                    uid_number: 100_000,
                    gid_number: 100_000,
                    home: "/home/alice".into(),
                    shell: "/bin/bash".into(),
                }),
            },
            DirUser {
                id: "id2".into(),
                uid: "bob@x.io".into(),
                email: "bob@x.io".into(),
                given_name: "Bob".into(),
                family_name: None,
                groups: vec![],
                posix: None,
            },
        ];
        let entries = build("app", &base, &users, &["admins".into()]);
        assert_eq!(entries.len(), 3 + 2 + 1);

        let alice = &entries[3];
        assert_eq!(alice.dn, "uid=alice,ou=people,cn=app,ou=clients,dc=x");
        assert_eq!(alice.values("CN").next().unwrap(), "Alice Liddell");
        assert_eq!(alice.values("uidNumber").next().unwrap(), "100000");
        assert!(alice.values("objectClass").any(|v| v == "posixAccount"));
        assert_eq!(
            alice.values("memberOf").collect::<Vec<_>>(),
            vec!["cn=admins,ou=groups,cn=app,ou=clients,dc=x"]
        );

        let bob = &entries[4];
        assert!(bob.values("sn").next().is_none());
        assert!(bob.values("uidNumber").next().is_none());

        let admins = &entries[5];
        assert_eq!(
            admins.values("member").collect::<Vec<_>>(),
            vec!["uid=alice,ou=people,cn=app,ou=clients,dc=x"]
        );

        let res = build("app", &base, &users, &[])
            .remove(3)
            .into_result(&["mail".into(), "UID".into()]);
        assert_eq!(res.attributes.len(), 2);
    }
}
