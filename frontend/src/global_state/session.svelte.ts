import { fetchGet } from '$api/fetch';
import type { SessionInfoResponse } from '$api/types/session';
import { isBrowser, redirectToLogin } from '$utils/helpers';

// Delegated group-admin role grammar. A role named exactly `rauthy_admin`
// is a full Rauthy admin. A role `rauthy_admin:<prefix>` makes the holder a group admin:
// `<prefix>` is an exact group-name match by default, a trailing `*` turns it into a prefix
// glob, and `rauthy_admin:*` therefore matches every group (the user-super-admin).
//
// This mirrors the backend logic in `src/data/src/entity/principal.rs` so the UI can disable
// / hide what a group admin is not allowed to do. The backend stays the single source of
// truth for authorization; these helpers only drive presentation, so they are private to
// `useSession()`, which is the interface components use to manage their views.
const FULL_ADMIN = 'rauthy_admin';
const GROUP_ADMIN_PREFIX = 'rauthy_admin:';

interface GroupMatcher {
    prefix: string;
    glob: boolean;
}

function parseRoles(roles: string | undefined): string[] {
    return (roles ?? '')
        .split(',')
        .map(r => r.trim())
        .filter(r => r.length > 0);
}

function rolesAreAdmin(roles: string | undefined): boolean {
    return parseRoles(roles).includes(FULL_ADMIN);
}

function rolesAreGroupAdmin(roles: string | undefined): boolean {
    return parseRoles(roles).some(r => r.startsWith(GROUP_ADMIN_PREFIX));
}

// the principal may access the admin UI at all: a full admin or a group admin
function rolesAreAnyAdmin(roles: string | undefined): boolean {
    return rolesAreAdmin(roles) || rolesAreGroupAdmin(roles);
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

// a `rauthy_admin:<prefix>[*]` role manages the given group `name`; empty names never match,
// mirroring the backend
function managesGroupFor(roles: string | undefined, name: string): boolean {
    if (!name) {
        return false;
    }
    return groupMatchers(roles).some(m => (m.glob ? name.startsWith(m.prefix) : name === m.prefix));
}

// subset of `allGroups` that this principal manages (drives the prefix-bounded group editor:
// group admins may only add / remove these)
function managedGroupsFor(roles: string | undefined, allGroups: string[]): string[] {
    return allGroups.filter(g => managesGroupFor(roles, g));
}

let _session: undefined | SessionInfoResponse = $state();

export function useSession(redirectState: 'admin' | 'account') {
    if (!_session && isBrowser()) {
        fetchGet<SessionInfoResponse>('/auth/v1/oidc/sessioninfo', 'json', 'noRedirect').then(
            res => {
                if (res.status === 401) {
                    redirectToLogin(redirectState);
                }
                _session = res.body;
                // TODO should we maybe start an interval for keep-alive?
            },
        );
    }

    return {
        get(): undefined | SessionInfoResponse {
            return _session;
        },
        set(session: SessionInfoResponse) {
            _session = session;
        },
        // delegated group-admin helpers, derived from the session roles so callers
        // don't need to import the role grammar or pass roles around
        isAdmin(): boolean {
            return rolesAreAdmin(_session?.roles);
        },
        isGroupAdmin(): boolean {
            return rolesAreGroupAdmin(_session?.roles);
        },
        isAnyAdmin(): boolean {
            return rolesAreAnyAdmin(_session?.roles);
        },
        managesGroup(name: string): boolean {
            return managesGroupFor(_session?.roles, name);
        },
        managedGroups(allGroups: string[]): string[] {
            return managedGroupsFor(_session?.roles, allGroups);
        },
    };
}
