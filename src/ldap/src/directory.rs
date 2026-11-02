use crate::dn;
use ldap3_proto::simple::{LdapPartialAttribute, LdapSearchResultEntry};
use rauthy_data::entity::clients::Client;
use rauthy_data::entity::groups::Group;
use rauthy_data::entity::pam::users::PamUser;
use rauthy_data::entity::users::User;
use rauthy_data::entity::users_values::UserValues;
use rauthy_error::ErrorResponse;
use std::collections::{HashMap, HashSet};

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
/// paging if large directories need it.
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

    let pam: HashMap<String, PamUser> = PamUser::find_all()
        .await?
        .into_iter()
        .map(|p| (p.email.clone(), p))
        .collect();
    let pam_names: HashSet<&str> = pam.values().map(|p| p.name.as_str()).collect();
    let preferred: HashMap<String, String> = UserValues::find_all_preferred_usernames()
        .await?
        .into_iter()
        // a PAM name wins over the same preferred_username of another user, see `resolve_email`
        .filter(|(_, name)| !pam_names.contains(name.as_str()))
        .collect();

    let users = User::find_all()
        .await?
        .into_iter()
        .filter(|u| {
            u.password.is_some()
                && u.enabled
                && u.check_expired().is_ok()
                && client.validate_user_groups(u).is_ok()
        })
        .map(|u| {
            let p = pam.get(&u.email);
            let uid = p
                .map(|p| p.name.clone())
                .or_else(|| preferred.get(&u.id).cloned())
                .unwrap_or_else(|| u.email.clone());
            DirUser {
                groups: u.get_groups(),
                posix: p.map(|p| PosixAttrs {
                    uid_number: p.id,
                    gid_number: p.gid,
                    home: p.home_dir(),
                    shell: p.shell.clone(),
                }),
                uid,
                id: u.id,
                email: u.email,
                given_name: u.given_name,
                family_name: u.family_name,
            }
        })
        .collect();

    Ok((users, groups))
}

/// Resolves a bind `uid` to the user's email, in the same order the directory assigns `uid`s:
/// PAM username, then `preferred_username`, then the email itself.
pub async fn resolve_email(uid: &str) -> Result<String, ErrorResponse> {
    if let Ok(p) = PamUser::find_by_name(uid.to_string()).await {
        return Ok(p.email);
    }
    if let Some(id) = UserValues::find_user_id_by_preferred_username(uid).await? {
        return Ok(User::find(id).await?.email);
    }
    Ok(uid.to_string())
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
