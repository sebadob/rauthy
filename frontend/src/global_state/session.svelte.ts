import type {SessionResponse} from "$api/types/session.ts";

let _session: undefined | SessionResponse = $state();

export function useSession() {
    return {
        get(): undefined | SessionResponse {
            return _session;
        },
        set(session: SessionResponse) {
            _session = session;
        }
    }
}
