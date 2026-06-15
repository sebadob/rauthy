import { fetchGet } from '$api/fetch';
import type { SessionInfoResponse } from '$api/types/session';
import { isBrowser, redirectToLogin } from '$utils/helpers';
import {
    isAdmin as rolesAreAdmin,
    isAnyAdmin as rolesAreAnyAdmin,
    isGroupAdmin as rolesAreGroupAdmin,
    managedGroups as managedGroupsFor,
    managesGroup as managesGroupFor,
} from '$utils/adminScope';

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
        // delegated group-admin helpers (#1538), derived from the session roles so callers
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
