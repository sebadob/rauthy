# Delegated Group Admins

Sometimes you do not want to hand out the full `rauthy_admin` role just so someone can help with
day-to-day user management for their own team. Delegated group admins let you scope user
administration to the members of one or more groups, without giving access to clients, scopes,
roles, system config, or any user outside of those groups.

## The role grammar

A delegated group admin is just a normal role with a special name:

```
rauthy_admin:<group>
```

The `rauthy_admin` role (without a colon) is the full Rauthy admin and behaves exactly as before.
A role with the `rauthy_admin:` prefix turns its holders into a **group admin** for the matching
groups:

- `rauthy_admin:engineering` matches the group `engineering` **exactly**.
- `rauthy_admin:eng*` is a **prefix glob** and matches `eng`, `engineering`, `eng_platform`, etc.
- `rauthy_admin:*` matches **every** group, making the holder a user-super-admin (still only over
  users, never over clients / scopes / config).

You assign these roles like any other role. Create a role named e.g. `rauthy_admin:engineering`,
then assign it to the users who should administer that group.

```admonish note
This is non-breaking: unless you already had a custom role that happens to start with
`rauthy_admin:`, nothing changes. On startup, Rauthy logs a warning listing any pre-existing roles
that match this scheme so you can double-check they are intended.
```

## What a group admin can do

A group admin can manage users who are a member of at least one group it manages **and** who are
not themselves any kind of `rauthy_admin`. For those users it can:

- list and view users (the user list is not filtered, see below),
- create new users and place them into a group it manages,
- edit profile data and toggle `enabled` / `email_verified`,
- add or remove memberships, but only for groups it manages,
- reset MFA (delete passkeys),
- view and invalidate (force-logout) sessions.

In addition, a group admin gets a read-only view of the Sessions, Events, and Blacklist pages to
help with debugging user issues.

## What a group admin cannot do

The backend enforces all of the following, regardless of what the UI shows:

- it cannot manage any user that holds a `rauthy_admin` or `rauthy_admin:<...>` role (no
  privilege escalation, and group admins cannot manage each other),
- it cannot modify a user's roles, or change group memberships outside of its managed groups,
- it cannot set a user's password,
- it cannot delete users,
- it cannot manage clients, scopes, roles, groups, providers, API keys, or system config,
- it cannot add or remove blacklist entries (read-only),
- API keys are never group-scoped; delegation applies to logged-in sessions only.

## Why the user list is not filtered

The `users` table inlines roles and groups for fast logins, which makes filtering users by group on
the database side expensive. A group admin therefore sees the same (minified) user list as a full
admin. This is intentional: the security boundary is enforced on every **write**, not on list
visibility, and an admin is trusted by definition. It also enables a useful flow: a group admin can
open any user and add them to a group it manages, even if that user was created by someone else.

If you need strict tenant isolation (group admins of independent organizations must not see each
other's users), that is a separate, more restrictive mode that can be added as an opt-in in a future
iteration.
