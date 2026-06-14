// Delegated group-admin helpers (see issue #1538).
//
// A role named exactly `rauthy_admin` is a full Rauthy admin. A role
// `rauthy_admin:<prefix>` makes the holder a group admin: `<prefix>` is an exact
// group-name match by default, a trailing `*` turns it into a prefix glob, and
// `rauthy_admin:*` therefore matches every group (the user-super-admin).
//
// This mirrors the backend logic in `src/data/src/entity/principal.rs` so the UI
// can disable / hide what a group admin is not allowed to do. The backend stays the
// single source of truth for authorization; these helpers only drive presentation.

const FULL_ADMIN = 'rauthy_admin';
const GROUP_ADMIN_PREFIX = 'rauthy_admin:';

interface GroupMatcher {
    prefix: string;
    glob: boolean;
}

export function parseRoles(roles: string | undefined): string[] {
    return (roles ?? '')
        .split(',')
        .map(r => r.trim())
        .filter(r => r.length > 0);
}

export function isFullAdmin(roles: string | undefined): boolean {
    return parseRoles(roles).includes(FULL_ADMIN);
}

export function isGroupAdmin(roles: string | undefined): boolean {
    return parseRoles(roles).some(r => r.startsWith(GROUP_ADMIN_PREFIX));
}

/// True if the principal may access the admin UI at all: a full admin or a group admin.
export function isAnyAdmin(roles: string | undefined): boolean {
    return isFullAdmin(roles) || isGroupAdmin(roles);
}

function groupMatchers(roles: string | undefined): GroupMatcher[] {
    return parseRoles(roles)
        .filter(r => r.startsWith(GROUP_ADMIN_PREFIX))
        .map(r => {
            const rest = r.slice(GROUP_ADMIN_PREFIX.length);
            return rest.endsWith('*')
                ? { prefix: rest.slice(0, -1), glob: true }
                : { prefix: rest, glob: false };
        });
}

/// True if a `rauthy_admin:<prefix>[*]` role manages the given group `name`.
/// Empty names never match, mirroring the backend.
export function managesGroup(roles: string | undefined, name: string): boolean {
    if (!name) {
        return false;
    }
    return groupMatchers(roles).some(m => (m.glob ? name.startsWith(m.prefix) : name === m.prefix));
}

/// Subset of `allGroups` that this principal manages (drives the prefix-bounded
/// group editor: group admins may only add / remove these).
export function managedGroups(roles: string | undefined, allGroups: string[]): string[] {
    return allGroups.filter(g => managesGroup(roles, g));
}
