use crate::dn;
use ldap3_proto::simple::{LdapPartialAttribute, LdapSearchResultEntry};
use rauthy_data::entity::clients::Client;
use rauthy_data::entity::groups::Group;
use rauthy_data::entity::pam::users::PamUser;
use rauthy_data::entity::users::User;
use rauthy_error::{ErrorResponse, ErrorResponseType};
use std::collections::HashMap;

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
/// Users must have a local password and a PAM username as `uid`, be enabled, not expired and pass
/// the client's group restriction. With `targets`, only these users are loaded instead of all of
/// them. Group `member`s then only contain the loaded users.
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
        None => load_all().await?,
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
                uid: uids.remove(&u.email)?,
                id: u.id,
                email: u.email,
                given_name: u.given_name,
                family_name: u.family_name,
            })
        })
        .collect();

    Ok((users, groups))
}

/// The users and their PAM usernames by email.
type Uids = (Vec<User>, HashMap<String, String>);

async fn load_all() -> Result<Uids, ErrorResponse> {
    let users = User::find_all().await?;
    let uids = PamUser::find_all()
        .await?
        .into_iter()
        .map(|p| (p.email, p.name))
        .collect();
    Ok((users, uids))
}

/// Loads only the `targets` with targeted queries.
async fn load_targets(targets: Vec<Target>) -> Result<Uids, ErrorResponse> {
    let mut users: Vec<User> = Vec::with_capacity(targets.len());
    let mut uids = HashMap::with_capacity(targets.len());
    for target in targets {
        let user = match target {
            Target::Id(id) => User::find(id).await,
            Target::Mail(email) => User::find_by_email(email).await,
            Target::Uid(uid) => match resolve_email(&uid).await? {
                Some(email) => User::find_by_email(email).await,
                None => continue,
            },
        };
        let user = match user {
            Ok(u) if !users.iter().any(|known| known.id == u.id) => u,
            Ok(_) => continue,
            Err(err) if err.error == ErrorResponseType::NotFound => continue,
            Err(err) => return Err(err),
        };
        if let Some(pam) = not_found_none(PamUser::find_by_user_id(user.id.clone()).await)? {
            uids.insert(pam.email, pam.name);
        }
        users.push(user);
    }
    Ok((users, uids))
}

/// Resolves a bind `uid` to the email of the user with this PAM username, if any. PAM usernames
/// are always lowercase, so this is unique even though LDAP compares `uid`s case-insensitively.
pub async fn resolve_email(uid: &str) -> Result<Option<String>, ErrorResponse> {
    Ok(not_found_none(PamUser::find_by_name(uid.to_lowercase()).await)?.map(|p| p.email))
}

fn not_found_none<T>(res: Result<T, ErrorResponse>) -> Result<Option<T>, ErrorResponse> {
    match res {
        Ok(v) => Ok(Some(v)),
        Err(err) if err.error == ErrorResponseType::NotFound => Ok(None),
        Err(err) => Err(err),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

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
