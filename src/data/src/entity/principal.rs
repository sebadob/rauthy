use crate::entity::api_keys::{AccessGroup, AccessRights, ApiKey};
use crate::entity::sessions::{Session, SessionState};
use crate::rauthy_config::RauthyConfig;
use actix_web::{HttpRequest, web};
use rauthy_common::constants::{RAUTHY_ADMIN_GROUP_PREFIX, RAUTHY_ADMIN_ROLE};
use rauthy_error::{ErrorResponse, ErrorResponseType};
use std::collections::BTreeSet;
use tracing::trace;

/// Used internally to check authentication and access rights for endpoints and actions
#[derive(Debug, Clone, Default)]
pub struct Principal {
    pub session: Option<Session>,
    pub api_key: Option<ApiKey>,
    pub roles: Vec<String>,
}

impl Principal {
    #[inline(always)]
    pub fn from_req(
        principal: web::ReqData<Option<Principal>>,
    ) -> Result<Principal, ErrorResponse> {
        principal.into_inner().ok_or_else(|| {
            ErrorResponse::new(
                ErrorResponseType::Unauthorized,
                "Only allowed as a logged in user",
            )
        })
    }

    #[inline(always)]
    pub fn get_session(&self) -> Result<&Session, ErrorResponse> {
        if let Some(s) = &self.session {
            Ok(s)
        } else {
            Err(ErrorResponse::new(
                ErrorResponseType::Unauthorized,
                "No valid session",
            ))
        }
    }

    #[inline(always)]
    pub fn get_session_csrf_token(&self) -> Result<&str, ErrorResponse> {
        Ok(self.get_session()?.csrf_token.as_str())
    }

    #[inline(always)]
    pub fn is_admin(&self) -> bool {
        self.roles.iter().any(|r| r.as_str() == RAUTHY_ADMIN_ROLE)
    }

    /// `true` if this principal holds at least one delegated group-admin role
    /// (`rauthy_admin:<prefix>`), regardless of whether it is also a full admin.
    #[inline]
    pub fn is_group_admin(&self) -> bool {
        self.roles
            .iter()
            .any(|r| r.as_str().starts_with(RAUTHY_ADMIN_GROUP_PREFIX))
    }

    /// `true` if this principal acts purely as a delegated group admin: it is a group
    /// admin, but neither a full `rauthy_admin` nor authenticated via an ApiKey. Used
    /// to apply the group-admin field-level restrictions only when they should apply.
    #[inline]
    pub fn is_session_group_admin(&self) -> bool {
        self.api_key.is_none() && !self.is_admin() && self.is_group_admin()
    }

    /// `true` if any `rauthy_admin:<prefix>[*]` role of this principal matches the
    /// given group `name`. Exact match by default; a trailing `*` makes it a prefix
    /// glob, so `rauthy_admin:*` matches every (non-empty) group name. Empty group
    /// names never match, so a user without any group is only manageable by a full
    /// `rauthy_admin`.
    fn group_admin_matches(&self, name: &str) -> bool {
        if name.is_empty() {
            return false;
        }
        self.roles.iter().any(
            |role| match role.as_str().strip_prefix(RAUTHY_ADMIN_GROUP_PREFIX) {
                Some(rest) => match rest.strip_suffix('*') {
                    Some(prefix) => name.starts_with(prefix),
                    None => name == rest,
                },
                None => false,
            },
        )
    }

    /// `true` if this group admin manages at least one of the given `groups`.
    pub fn group_admin_manages_any<'a, I>(&self, groups: I) -> bool
    where
        I: IntoIterator<Item = &'a str>,
    {
        groups.into_iter().any(|g| self.group_admin_matches(g))
    }

    /// Pure decision for the group-admin path: may a group admin manage a user with
    /// the given `target_roles` and `target_groups`? Encodes the v1 escalation guards:
    /// - the target must not itself be any kind of `rauthy_admin` (neither a full admin
    ///   nor a group admin), and
    /// - the target must be a member of at least one group this admin manages.
    ///
    /// This does **not** verify that `self` has a valid session or is a group admin at
    /// all; callers gate on session/MFA and [`Self::is_group_admin`] first (see
    /// [`Self::validate_group_admin_can_manage`]).
    pub fn group_admin_can_manage<'a, R, G>(&self, target_roles: R, target_groups: G) -> bool
    where
        R: IntoIterator<Item = &'a str>,
        G: IntoIterator<Item = &'a str>,
    {
        let target_is_admin = target_roles
            .into_iter()
            .any(|r| r == RAUTHY_ADMIN_ROLE || r.starts_with(RAUTHY_ADMIN_GROUP_PREFIX));
        if target_is_admin {
            return false;
        }
        self.group_admin_manages_any(target_groups)
    }

    /// Validates that the principal is an authenticated, delegated group admin.
    ///
    /// This mirrors [`Self::validate_admin_session`] for group admins: a valid session,
    /// the same admin-MFA enforcement, and at least one `rauthy_admin:<prefix>` role. It
    /// looks at no target user, so it is a cheap gate to run before fetching anything from
    /// the database.
    #[inline]
    pub fn validate_group_admin_session(&self) -> Result<(), ErrorResponse> {
        self.validate_session_auth()?;

        if !self.is_group_admin() {
            return Err(ErrorResponse::new(
                ErrorResponseType::Forbidden,
                "Group admin access only",
            ));
        }

        if RauthyConfig::get().vars.mfa.admin_force_mfa && !self.has_mfa_active() {
            return Err(ErrorResponse::new(
                ErrorResponseType::MfaRequired,
                "Admin access only allowed with MFA active",
            ));
        }

        Ok(())
    }

    /// Cheap authorization gate for endpoints a full admin or a delegated group admin may
    /// reach. Run before any database lookup, so an unauthenticated or otherwise
    /// unauthorized caller cannot trigger extra queries.
    ///
    /// A full `rauthy_admin` session or a matching `ApiKey` grants full, un-scoped access;
    /// a group-admin session is additionally allowed, in which case the caller must run the
    /// per-target [`Self::validate_group_admin_can_manage`] check once the target is loaded.
    pub fn validate_api_key_or_group_admin(
        &self,
        access_group: AccessGroup,
        access_rights: AccessRights,
    ) -> Result<(), ErrorResponse> {
        // most Admin UI requests are a full admin session, so check that first
        if self.is_admin() {
            return self.validate_admin_session();
        }
        // a logged-in delegated group admin
        if self.is_group_admin() {
            return self.validate_group_admin_session();
        }
        // otherwise it must be an ApiKey, the least likely case with this feature
        self.validate_api_key(access_group, access_rights)
    }

    /// Per-target check for the group-admin path: errors unless this group admin may manage
    /// a user with the given `target_roles` / `target_groups` (see
    /// [`Self::group_admin_can_manage`]). A no-op for full admins and ApiKeys, so it is safe
    /// to call unconditionally after [`Self::validate_api_key_or_group_admin`].
    ///
    /// `target_roles` / `target_groups` are the comma-split roles and groups of the user
    /// being administered (e.g. `user.roles_iter()` / `user.groups_iter()`).
    pub fn validate_group_admin_can_manage<'a, R, G>(
        &self,
        target_roles: R,
        target_groups: G,
    ) -> Result<(), ErrorResponse>
    where
        R: IntoIterator<Item = &'a str>,
        G: IntoIterator<Item = &'a str>,
    {
        if self.api_key.is_some() || self.is_admin() {
            return Ok(());
        }
        if !self.group_admin_can_manage(target_roles, target_groups) {
            return Err(ErrorResponse::new(
                ErrorResponseType::Forbidden,
                "You are not allowed to manage this user",
            ));
        }
        Ok(())
    }

    /// Validates that an update of an existing user is allowed for a group admin.
    ///
    /// No-op for full `rauthy_admin`s and ApiKeys. For a group admin it enforces the
    /// v1 field-level guards:
    /// - the roles must match the target's current roles (a group admin can never modify
    ///   any role), and
    /// - group memberships may only be added or removed for groups this admin manages;
    ///   any membership in an unmanaged group must stay exactly as it was.
    pub fn validate_group_admin_user_change<'a, CR, CG>(
        &self,
        current_roles: CR,
        current_groups: CG,
        new_roles: &[String],
        new_groups: &[String],
    ) -> Result<(), ErrorResponse>
    where
        CR: IntoIterator<Item = &'a str>,
        CG: IntoIterator<Item = &'a str>,
    {
        if self.api_key.is_some() || self.is_admin() {
            return Ok(());
        }

        let current_roles: BTreeSet<&str> = current_roles
            .into_iter()
            .filter(|r| !r.is_empty())
            .collect();
        let new_roles: BTreeSet<&str> = new_roles
            .iter()
            .map(|r| r.as_str())
            .filter(|r| !r.is_empty())
            .collect();
        if current_roles != new_roles {
            return Err(ErrorResponse::new(
                ErrorResponseType::Forbidden,
                "Group admins cannot modify user roles",
            ));
        }

        let current_groups: BTreeSet<&str> = current_groups
            .into_iter()
            .filter(|g| !g.is_empty())
            .collect();
        let new_groups: BTreeSet<&str> = new_groups
            .iter()
            .map(|g| g.as_str())
            .filter(|g| !g.is_empty())
            .collect();
        for changed in current_groups.symmetric_difference(&new_groups) {
            if !self.group_admin_matches(changed) {
                return Err(ErrorResponse::new(
                    ErrorResponseType::Forbidden,
                    "Group admins can only change memberships of groups they manage",
                ));
            }
        }
        Ok(())
    }

    /// Validates that creating a new user is allowed for a group admin.
    ///
    /// No-op for full `rauthy_admin`s and ApiKeys. For a group admin it enforces that
    /// the new user gets no role assigned and is placed into at least one, and only,
    /// groups this admin manages (so the created account stays within the admin's
    /// scope and cannot be elevated).
    pub fn validate_group_admin_user_create(
        &self,
        new_roles: &[String],
        new_groups: Option<&Vec<String>>,
    ) -> Result<(), ErrorResponse> {
        if self.api_key.is_some() || self.is_admin() {
            return Ok(());
        }

        if new_roles.iter().any(|r| !r.is_empty()) {
            return Err(ErrorResponse::new(
                ErrorResponseType::Forbidden,
                "Group admins cannot assign roles",
            ));
        }

        // the new user must be placed into at least one group, so it stays in scope
        let groups: Vec<&str> = new_groups
            .map(|g| {
                g.iter()
                    .map(|s| s.as_str())
                    .filter(|s| !s.is_empty())
                    .collect()
            })
            .unwrap_or_default();
        if groups.is_empty() {
            return Err(ErrorResponse::new(
                ErrorResponseType::Forbidden,
                "Group admins must assign the new user to a group they manage",
            ));
        }
        for g in groups {
            if !self.group_admin_matches(g) {
                return Err(ErrorResponse::new(
                    ErrorResponseType::Forbidden,
                    "Group admins can only assign groups they manage",
                ));
            }
        }
        Ok(())
    }

    #[inline(always)]
    pub fn is_user(&self, id: &str) -> Result<(), ErrorResponse> {
        if self.user_id() != Ok(id) {
            Err(ErrorResponse::new(
                ErrorResponseType::Forbidden,
                "You are not allowed to modify this user",
            ))
        } else {
            Ok(())
        }
    }

    #[inline(always)]
    pub fn has_mfa_active(&self) -> bool {
        self.session.as_ref().map(|s| s.is_mfa).unwrap_or(false)
    }

    /// Returns the `user_id` linked to this session. Will `Err` if this principal does not
    /// have a valid session.
    #[inline(always)]
    pub fn user_id(&self) -> Result<&str, ErrorResponse> {
        self.session
            .as_ref()
            .and_then(|s| s.user_id.as_deref())
            .ok_or_else(|| {
                ErrorResponse::new(ErrorResponseType::Forbidden, "No session for principal")
            })
    }

    #[inline(always)]
    pub fn validate_api_key(
        &self,
        access_group: AccessGroup,
        access_rights: AccessRights,
    ) -> Result<(), ErrorResponse> {
        if let Some(api_key) = &self.api_key {
            match api_key.validate_access(&access_group, &access_rights) {
                Ok(_) => Ok(()),
                Err(_) => Err(ErrorResponse::new(
                    ErrorResponseType::Forbidden,
                    format!(
                        "Bad permissions for given ApiKey. Needed: {access_group:?} / \
                        {access_rights:?}"
                    ),
                )),
            }
        } else {
            Err(ErrorResponse::new(
                ErrorResponseType::Unauthorized,
                "No API Key found",
            ))
        }
    }

    /// Validates the Principal's session that it is authenticated and either an admin session, or
    /// that the given `id` belongs to the user of this session.
    #[inline(always)]
    pub fn validate_admin_or_user(&self, id: &str) -> Result<(), ErrorResponse> {
        self.validate_session_auth()?;

        if self.is_admin() || self.user_id() == Ok(id) {
            Ok(())
        } else {
            Err(ErrorResponse::new(
                ErrorResponseType::Forbidden,
                "You are not allowed to modify this user",
            ))
        }
    }

    /// Validates the Principal's session to only allow authorized Rauthy admin access.
    #[inline(always)]
    pub fn validate_admin_session(&self) -> Result<(), ErrorResponse> {
        self.validate_session_auth()?;

        if !self.is_admin() {
            return Err(ErrorResponse::new(
                ErrorResponseType::Forbidden,
                "Rauthy admin access only",
            ));
        }

        if RauthyConfig::get().vars.mfa.admin_force_mfa && !self.has_mfa_active() {
            return Err(ErrorResponse::new(
                ErrorResponseType::MfaRequired,
                "Rauthy admin access only allowed with MFA active",
            ));
        }

        Ok(())
    }

    /// Validates an ApiKey OR a valid admin session.
    /// If both are given, the ApiKey will have the higher priority since it is more specific.
    /// Returns an error with an invalid ApiKey even when a valid session exists.
    #[inline(always)]
    pub fn validate_api_key_or_admin_session(
        &self,
        access_group: AccessGroup,
        access_rights: AccessRights,
    ) -> Result<(), ErrorResponse> {
        match self.validate_api_key(access_group, access_rights) {
            Ok(_) => Ok(()),

            Err(err) => {
                if err.error == ErrorResponseType::Forbidden {
                    // in this case, return the Error from the ApiKey with the hint
                    // for the needed permissions in any case -> better DX and debugging
                    // without real security issues
                    Err(err)
                } else {
                    self.validate_admin_session()
                }
            }
        }
    }

    /// Validates the principal, that it is either an admin or the user matches the
    /// given `user_id`
    #[inline(always)]
    pub fn validate_user_or_admin(&self, user_id: &str) -> Result<(), ErrorResponse> {
        let session = self.validate_session_auth()?;
        if !self.is_admin() && session.user_id.as_deref() != Some(user_id) {
            Err(ErrorResponse::new(
                ErrorResponseType::Forbidden,
                "Access is forbidden with this user",
            ))
        } else {
            Ok(())
        }
    }

    /// Validates the given user_id against this Principal.
    pub fn validate_user_session(&self, user_id: &str) -> Result<(), ErrorResponse> {
        let session = self.validate_session_auth()?;
        if session.user_id.as_deref() == Some(user_id) {
            Ok(())
        } else {
            trace!("Validating the session failed - was not in auth state");
            Err(ErrorResponse::new(
                ErrorResponseType::Forbidden,
                "Invalid user ID",
            ))
        }
    }

    #[inline(always)]
    pub fn validate_session_auth(&self) -> Result<&Session, ErrorResponse> {
        if let Some(session) = &self.session {
            if session.state()? == SessionState::Auth {
                Ok(session)
            } else {
                trace!("Validating the session failed - was not in auth state");
                Err(ErrorResponse::new(
                    ErrorResponseType::Unauthorized,
                    "Unauthorized Session",
                ))
            }
        } else {
            trace!("Validating the session failed - no session found");
            Err(ErrorResponse::new(
                ErrorResponseType::Unauthorized,
                "No valid session",
            ))
        }
    }

    #[inline(always)]
    pub fn validate_session_auth_or_init(&self) -> Result<(), ErrorResponse> {
        if let Some(session) = &self.session {
            let state = session.state()?;
            if state == SessionState::Auth || state == SessionState::Init {
                Ok(())
            } else {
                trace!("Validating the session failed - was not in init or auth state");
                Err(ErrorResponse::new(
                    ErrorResponseType::Unauthorized,
                    "Unauthorized Session",
                ))
            }
        } else {
            trace!("Validating the session failed - no session found");
            Err(ErrorResponse::new(
                ErrorResponseType::Unauthorized,
                "No valid session",
            ))
        }
    }

    #[inline(always)]
    pub fn validate_session_csrf_exception(&self, req: &HttpRequest) -> Result<(), ErrorResponse> {
        let s = self.get_session()?;
        s.validate_csrf(req)?;
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn principal(roles: &[&str]) -> Principal {
        Principal {
            roles: roles.iter().map(|r| r.to_string()).collect(),
            ..Default::default()
        }
    }

    #[test]
    fn test_is_group_admin() {
        assert!(principal(&["rauthy_admin:dev"]).is_group_admin());
        assert!(principal(&["user", "rauthy_admin:dev"]).is_group_admin());
        assert!(principal(&["rauthy_admin:*"]).is_group_admin());
        // a full admin is NOT a group admin via this check (no colon)
        assert!(!principal(&["rauthy_admin"]).is_group_admin());
        assert!(!principal(&["user", "admin"]).is_group_admin());
        assert!(!principal(&[]).is_group_admin());
    }

    #[test]
    fn test_group_admin_exact_match_is_default() {
        let p = principal(&["rauthy_admin:dev"]);
        assert!(p.group_admin_matches("dev"));
        // exact match must NOT behave like a prefix
        assert!(!p.group_admin_matches("developers"));
        assert!(!p.group_admin_matches("devops"));
        assert!(!p.group_admin_matches("ops"));
        assert!(!p.group_admin_matches(""));
    }

    #[test]
    fn test_group_admin_trailing_star_is_prefix_glob() {
        let p = principal(&["rauthy_admin:dev*"]);
        assert!(p.group_admin_matches("dev"));
        assert!(p.group_admin_matches("developers"));
        assert!(p.group_admin_matches("devops"));
        assert!(!p.group_admin_matches("ops"));
        assert!(!p.group_admin_matches(""));
    }

    #[test]
    fn test_group_admin_star_is_super_admin() {
        // `rauthy_admin:*` is a prefix glob with an empty prefix -> matches every
        // (non-empty) group, with zero special-case code (the user-super-admin).
        let p = principal(&["rauthy_admin:*"]);
        assert!(p.group_admin_matches("dev"));
        assert!(p.group_admin_matches("anything"));
        assert!(p.group_admin_matches("ops"));
        // empty group name never matches -> group-less users need a full admin
        assert!(!p.group_admin_matches(""));
    }

    #[test]
    fn test_group_admin_empty_prefix_matches_nothing() {
        // `rauthy_admin:` (no glob) is an exact match against the empty string and
        // therefore matches no real group - harmless, as discussed in #1538.
        let p = principal(&["rauthy_admin:"]);
        assert!(!p.group_admin_matches("dev"));
        assert!(!p.group_admin_matches(""));
        assert!(!p.group_admin_manages_any(["dev", "ops"]));
    }

    #[test]
    fn test_group_admin_manages_any_multiple_roles() {
        let p = principal(&["rauthy_admin:dev", "rauthy_admin:sales"]);
        assert!(p.group_admin_manages_any(["sales"]));
        assert!(p.group_admin_manages_any(["dev"]));
        assert!(p.group_admin_manages_any(["other", "dev"]));
        assert!(!p.group_admin_manages_any(["other", "ops"]));
    }

    #[test]
    fn test_escalation_guard_cannot_manage_full_admin() {
        let p = principal(&["rauthy_admin:dev"]);
        // target is in a managed group, but is itself a full rauthy_admin -> denied
        assert!(!p.group_admin_can_manage(["rauthy_admin", "user"], ["dev"]));
    }

    #[test]
    fn test_escalation_guard_cannot_manage_other_group_admin() {
        let p = principal(&["rauthy_admin:dev"]);
        // target is in a managed group, but is itself another group admin -> denied
        assert!(!p.group_admin_can_manage(["rauthy_admin:sales", "user"], ["dev"]));
    }

    #[test]
    fn test_escalation_guard_cannot_manage_unmatched_group() {
        let p = principal(&["rauthy_admin:dev"]);
        // ordinary target, but not in any managed group -> denied (even by id)
        assert!(!p.group_admin_can_manage(["user"], ["ops", "sales"]));
        // a target with no groups at all -> denied
        assert!(!p.group_admin_can_manage(["user"], [""]));
        assert!(!p.group_admin_can_manage(["user"], std::iter::empty::<&str>()));
    }

    #[test]
    fn test_group_admin_can_manage_matching_ordinary_user() {
        let p = principal(&["rauthy_admin:dev"]);
        assert!(p.group_admin_can_manage(["user"], ["dev"]));
        assert!(p.group_admin_can_manage(["user", "other"], ["other", "dev"]));
    }

    #[test]
    fn test_super_admin_can_manage_any_ordinary_user() {
        let p = principal(&["rauthy_admin:*"]);
        assert!(p.group_admin_can_manage(["user"], ["whatever"]));
        // but still never another admin
        assert!(!p.group_admin_can_manage(["rauthy_admin"], ["whatever"]));
        assert!(!p.group_admin_can_manage(["rauthy_admin:dev"], ["whatever"]));
    }

    fn s(v: &[&str]) -> Vec<String> {
        v.iter().map(|x| x.to_string()).collect()
    }

    #[test]
    fn test_change_guard_noop_for_full_admin() {
        let p = principal(&["rauthy_admin"]);
        // a full admin may change anything, incl. roles
        assert!(
            p.validate_group_admin_user_change(
                ["user"],
                ["dev"],
                &s(&["rauthy_admin"]),
                &s(&["ops"])
            )
            .is_ok()
        );
    }

    #[test]
    fn test_change_guard_group_admin_cannot_change_roles() {
        let p = principal(&["rauthy_admin:dev"]);
        // role added -> rejected
        assert!(
            p.validate_group_admin_user_change(
                ["user"],
                ["dev"],
                &s(&["user", "extra"]),
                &s(&["dev"])
            )
            .is_err()
        );
        // role removed -> rejected
        assert!(
            p.validate_group_admin_user_change(
                ["user", "extra"],
                ["dev"],
                &s(&["user"]),
                &s(&["dev"])
            )
            .is_err()
        );
        // roles identical (order-insensitive) -> ok
        assert!(
            p.validate_group_admin_user_change(
                ["user", "extra"],
                ["dev"],
                &s(&["extra", "user"]),
                &s(&["dev"])
            )
            .is_ok()
        );
    }

    #[test]
    fn test_change_guard_group_membership_scoping() {
        let p = principal(&["rauthy_admin:dev"]);
        // add a managed group -> ok
        assert!(
            p.validate_group_admin_user_change(
                ["user"],
                ["other"],
                &s(&["user"]),
                &s(&["other", "dev"])
            )
            .is_ok()
        );
        // remove a managed group -> ok
        assert!(
            p.validate_group_admin_user_change(
                ["user"],
                ["other", "dev"],
                &s(&["user"]),
                &s(&["other"])
            )
            .is_ok()
        );
        // add an unmanaged group -> rejected
        assert!(
            p.validate_group_admin_user_change(
                ["user"],
                ["dev"],
                &s(&["user"]),
                &s(&["dev", "ops"])
            )
            .is_err()
        );
        // remove an unmanaged group (e.g. trying to strip a membership outside scope) -> rejected
        assert!(
            p.validate_group_admin_user_change(
                ["user"],
                ["dev", "ops"],
                &s(&["user"]),
                &s(&["dev"])
            )
            .is_err()
        );
        // unmanaged group left untouched while touching a managed one -> ok
        assert!(
            p.validate_group_admin_user_change(
                ["user"],
                ["ops"],
                &s(&["user"]),
                &s(&["ops", "dev"])
            )
            .is_ok()
        );
    }

    #[test]
    fn test_create_guard_group_admin() {
        let p = principal(&["rauthy_admin:dev"]);
        // ok: no roles, into a managed group
        assert!(
            p.validate_group_admin_user_create(&s(&[]), Some(&s(&["dev"])))
                .is_ok()
        );
        // rejected: assigns a role
        assert!(
            p.validate_group_admin_user_create(&s(&["user"]), Some(&s(&["dev"])))
                .is_err()
        );
        // rejected: no managed group
        assert!(
            p.validate_group_admin_user_create(&s(&[]), Some(&s(&["ops"])))
                .is_err()
        );
        // rejected: no groups at all
        assert!(p.validate_group_admin_user_create(&s(&[]), None).is_err());
        // rejected: one managed + one unmanaged group
        assert!(
            p.validate_group_admin_user_create(&s(&[]), Some(&s(&["dev", "ops"])))
                .is_err()
        );
    }

    #[test]
    fn test_create_guard_noop_for_full_admin() {
        let p = principal(&["rauthy_admin"]);
        assert!(
            p.validate_group_admin_user_create(&s(&["rauthy_admin"]), None)
                .is_ok()
        );
    }
}
