import { fetchGet } from '$api/fetch';
import type { SessionInfoResponse } from '$api/types/session';
import { isBrowser, redirectToLogin } from '$utils/helpers';

let _session: undefined | SessionInfoResponse = $state();

export function useSession(redirectState: 'admin' | 'account') {
    if (!_session && isBrowser()) {
        fetchGet<SessionInfoResponse>('/auth/v1/oidc/sessioninfo', 'json', 'noRedirect').then(res => {
            if (res.status === 401) {
                redirectToLogin(redirectState);
            }
            _session = res.body;
            // TODO should we maybe start an interval for keep-alive?
        });
    }

    return {
        get(): undefined | SessionInfoResponse {
            return _session;
        },
        set(session: SessionInfoResponse) {
            _session = session;
        },
    };
}
