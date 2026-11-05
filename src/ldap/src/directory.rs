use crate::dn;
use ldap3_proto::simple::{LdapPartialAttribute, LdapSearchResultEntry};
use rauthy_data::entity::clients::Client;
use rauthy_data::entity::groups::Group;
use rauthy_data::entity::users::User;
use rauthy_data::entity::users_values::{UidCandidate, UserValues};
use rauthy_error::{ErrorResponse, ErrorResponseType};
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

pub struct DirUser {
    pub id: String,
    pub uid: String,
    pub email: String,
    pub given_name: String,
    pub family_name: Option<String>,
    pub groups: Vec<String>,
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
        let attrs = vec![
            (
                "objectClass",
                vec![
                    "top".into(),
                    "person".into(),
                    "organizationalPerson".into(),
                    "inetOrgPerson".into(),
                ],
            ),
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

/// A user a search can be narrowed down to, used at or above `server.ssp_threshold`.
#[derive(Debug, PartialEq, Eq, PartialOrd, Ord)]
pub enum Target {
    Id(String),
    Uid(String),
    Mail(String),
}

/// Loads the users and groups visible to `client`.
/// Users must have a local password and a `uid`, be enabled, not expired and pass the client's
/// group restriction. With `targets`, only these users are loaded instead of all of them, with the same
/// `uid`s a full load would assign. Group `member`s then only contain the loaded users.
pub async fn load(
    client: &Client,
    targets: Option<Vec<Target>>,
) -> Result<(Vec<DirUser>, Vec<String>), ErrorResponse> {
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

    let (all, mut uids) = match targets {
        None => load_uids().await?,
        Some(targets) => load_targets(targets).await?,
    };
    let users = all
        .into_iter()
        .filter(|u| {
            u.password.is_some()
                && u.enabled
                && u.check_expired().is_ok()
                && client.validate_user_groups(u).is_ok()
        })
        .filter_map(|u| {
            Some(DirUser {
                groups: u.get_groups(),
                uid: uids.remove(&u.id)?,
                id: u.id,
                email: u.email,
                given_name: u.given_name,
                family_name: u.family_name,
            })
        })
        .collect();

    Ok((users, groups))
}

/// Loads only the `targets` with targeted queries, see [load_uids] for the result.
async fn load_targets(targets: Vec<Target>) -> Result<Uids, ErrorResponse> {
    let mut users: Vec<User> = Vec::with_capacity(targets.len());
    for target in targets {
        let user = match target {
            Target::Id(id) => User::find(id).await,
            Target::Mail(email) => User::find_by_email(email).await,
            Target::Uid(uid) => match resolve_email(&uid).await? {
                Some(email) => User::find_by_email(email).await,
                None => continue,
            },
        };
        match user {
            Ok(u) if !users.iter().any(|known| known.id == u.id) => users.push(u),
            Ok(_) => {}
            Err(err) if err.error == ErrorResponseType::NotFound => {}
            Err(err) => return Err(err),
        }
    }

    let mut uids = HashMap::with_capacity(users.len());
    for u in &users {
        let Some(name) = UserValues::find_preferred_username(&u.id).await? else {
            continue;
        };
        let rows = UserValues::find_uid_candidates(&name.to_lowercase()).await?;
        if let Some(uid) = uid_of(&u.id, rows) {
            uids.insert(u.id.clone(), uid);
        }
    }
    Ok((users, uids))
}

/// Resolves a bind `uid` to the email of the user it is published for, if any.
pub async fn resolve_email(uid: &str) -> Result<Option<String>, ErrorResponse> {
    let uid_lc = uid.to_lowercase();
    let rows = UserValues::find_uid_candidates(&uid_lc).await?;
    Ok(uid_owner(&uid_lc, rows))
}

/// Returns the email of the user in `rows` whose `uid` is `uid_lc`. `rows` must contain every
/// user whose `preferred_username` or email equals `uid_lc`.
fn uid_owner(uid_lc: &str, rows: Vec<UidCandidate>) -> Option<String> {
    let (rows, uids) = assign_candidate_uids(rows);
    rows.into_iter()
        .find(|r| {
            uids.get(&r.0)
                .is_some_and(|uid| uid.to_lowercase() == uid_lc)
        })
        .map(|r| r.1)
}

/// Returns the `uid` of user `id`. `rows` must contain every user whose `preferred_username` or
/// email equals that user's `preferred_username`.
fn uid_of(id: &str, rows: Vec<UidCandidate>) -> Option<String> {
    assign_candidate_uids(rows).1.remove(id)
}

fn assign_candidate_uids(
    mut rows: Vec<UidCandidate>,
) -> (Vec<UidCandidate>, HashMap<String, String>) {
    rows.sort_by(|a, b| a.0.cmp(&b.0));
    rows.dedup_by(|a, b| a.0 == b.0);
    let ids: Vec<(String, String)> = rows.iter().map(|r| (r.0.clone(), r.1.clone())).collect();
    let preferred = rows
        .iter()
        .filter_map(|r| Some((r.0.clone(), r.2.clone()?)))
        .collect();
    let uids = assign_uids(&ids, &preferred);
    (rows, uids)
}

type Uids = (Vec<User>, HashMap<String, String>);

/// Loads all users and the `uid`s of the users that have one by user ID.
async fn load_uids() -> Result<Uids, ErrorResponse> {
    let users = User::find_all().await?;
    let preferred = UserValues::find_all_preferred_usernames()
        .await?
        .into_iter()
        .collect();
    let ids: Vec<(String, String)> = users
        .iter()
        .map(|u| (u.id.clone(), u.email.clone()))
        .collect();
    let uids = assign_uids(&ids, &preferred);
    Ok((users, uids))
}

/// Assigns each `(id, email)` with a `preferred_username` that name as its `uid`. Users without
/// one get no `uid` and are not part of the directory, so an email is never a `uid`. A name that is
/// not unique case-insensitively, or that equals another user's email, gets no `uid` either,
/// because `uid` filters are case-insensitive and apps would otherwise pick the wrong user.
/// Non-ASCII names are ignored, because the bind lookup folds case in SQL, which only folds ASCII.
fn assign_uids(
    users: &[(String, String)],
    preferred: &HashMap<String, String>,
) -> HashMap<String, String> {
    let emails: HashMap<String, &str> = users
        .iter()
        .map(|(id, email)| (email.to_lowercase(), id.as_str()))
        .collect();
    let mut counts: HashMap<String, usize> = HashMap::new();
    for n in preferred.values().filter(|n| n.is_ascii()) {
        *counts.entry(n.to_lowercase()).or_default() += 1;
    }

    users
        .iter()
        .filter_map(|(id, _)| {
            let name = preferred.get(id)?;
            let lc = name.to_lowercase();
            if name.is_ascii()
                && counts.get(&lc) == Some(&1)
                && emails.get(&lc).is_none_or(|owner| *owner == id)
            {
                Some((id.clone(), name.clone()))
            } else {
                warn!("LDAP uid '{name}' is not unique or not ASCII, skipping user {id}");
                None
            }
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
            (s("7"), s("g@x.io")),
            (s("8"), s("h@x.io")),
        ];
        let preferred = HashMap::from([
            (s("1"), s("alice")),
            (s("3"), s("Carol")),
            (s("4"), s("carol")),  // case-insensitive duplicate, both get no uid
            (s("5"), s("B@x.io")), // another user's email
            (s("6"), s("F@x.io")), // the user's own email
            (s("7"), s("\u{212A}im")), // non-ASCII, ignored, so `kim` stays unique
            (s("8"), s("kim")),
        ]);
        let uids = assign_uids(&users, &preferred);
        let expected = HashMap::from([
            (s("1"), s("alice")),
            (s("6"), s("F@x.io")),
            (s("8"), s("kim")),
        ]);
        assert_eq!(uids, expected);

        let rows: Vec<UidCandidate> = users
            .iter()
            .map(|(id, email)| (id.clone(), email.clone(), preferred.get(id).cloned()))
            .collect();
        let owner = |uid: &str| uid_owner(uid, rows.clone());
        assert_eq!(owner("alice").as_deref(), Some("a@x.io"));
        assert_eq!(owner("kim").as_deref(), Some("h@x.io"));
        assert_eq!(owner("f@x.io").as_deref(), Some("f@x.io"));
        // emails are never a uid
        assert_eq!(owner("b@x.io"), None);
        assert_eq!(owner("a@x.io"), None);
        assert_eq!(owner("carol"), None);
        // targeted loads assign the same uids as the full load
        for (id, _) in &users {
            assert_eq!(uid_of(id, rows.clone()), uids.get(id).cloned());
        }
        // only the rows the query returns for `b@x.io`: user 2 by email, user 5 by name
        assert_eq!(uid_of("5", vec![rows[1].clone(), rows[4].clone()]), None);
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
            },
            DirUser {
                id: "id2".into(),
                uid: "bob@x.io".into(),
                email: "bob@x.io".into(),
                given_name: "Bob".into(),
                family_name: None,
                groups: vec![],
            },
        ];
        let entries = build("app", &base, &users, &["admins".into()]);
        assert_eq!(entries.len(), 3 + 2 + 1);

        let alice = &entries[3];
        assert_eq!(alice.dn, "uid=alice,ou=people,cn=app,ou=clients,dc=x");
        assert_eq!(alice.values("CN").next().unwrap(), "Alice Liddell");
        assert!(alice.values("objectClass").any(|v| v == "inetOrgPerson"));
        assert_eq!(
            alice.values("memberOf").collect::<Vec<_>>(),
            vec!["cn=admins,ou=groups,cn=app,ou=clients,dc=x"]
        );

        let bob = &entries[4];
        assert!(bob.values("sn").next().is_none());

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
