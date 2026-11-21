import { fetchGet } from '#api/fetch.ts';
import type { KeepAliveResponse } from '#api/types/generic.ts';
import { isBrowser } from '#utils/helpers.ts';

const URI_KEEP_ALIVE = '/auth/v1/keep_alive';

// Each beat runs this close before the session's idle timeout deadline. It is what keeps an open
// login page, the account dashboard and the admin UI alive: one request per open tab per
// SESSION_TIMEOUT.
const KEEP_ALIVE_BEFORE_DEADLINE_MS = 60_000;

// The floor of every beat. The backend only refreshes `last_seen` when it is older than 10 s, so a
// shorter beat would answer a deadline it cannot advance.
const KEEP_ALIVE_MIN_BEAT_MS = 10_000;

// `setTimeout` fires immediately for delays above 2147483647 ms, which a long SESSION_TIMEOUT
// produces. A longer timeout then simply gets more beats than it strictly needs.
const KEEP_ALIVE_MAX_BEAT_MS = 12 * 60 * 60 * 1000;

let timer: undefined | number = undefined;
let deadline = 0;
let started = false;

/**
 * Runs the idle keep-alive beat for the lifetime of this document, exactly once per page load.
 * `deadlineMs` is the session's idle timeout deadline as a unix epoch in milliseconds, and pages
 * pass the one they already got from their own first request. Without it the first beat fires
 * immediately, because that is what learns the deadline - the login page has no session request of
 * its own. `onExpired` runs on a 401, which means the session is gone and only a restart helps:
 * reload on the login page, `redirectToLogin` in the account dashboard and the admin UI. The chain
 * stops there, the restart is what continues it.
 */
export function startKeepAlive(onExpired: () => void, deadlineMs?: number): void {
    if (started || !isBrowser()) {
        return;
    }
    started = true;

    // A browser can freeze a background tab, which stops the timer while the deadline keeps
    // running, so the beat is re-armed on every visibility change.
    window.addEventListener('visibilitychange', () => {
        if (deadline - Date.now() <= KEEP_ALIVE_BEFORE_DEADLINE_MS + KEEP_ALIVE_MIN_BEAT_MS) {
            beat(onExpired);
        } else {
            schedule(onExpired);
        }
    });
    window.addEventListener('pagehide', () => {
        if (timer !== undefined) {
            window.clearTimeout(timer);
        }
    });

    if (Number.isFinite(deadlineMs)) {
        deadline = deadlineMs as number;
        schedule(onExpired);
    } else {
        beat(onExpired);
    }
}

async function beat(onExpired: () => void): Promise<void> {
    const res = await fetchGet<KeepAliveResponse>(URI_KEEP_ALIVE, 'json', 'noRedirect');

    if (res.status === 401) {
        timer = undefined;
        onExpired();
        return;
    }

    // Anything else (offline, 5xx) keeps the last known deadline and simply beats again on the
    // floor, so a backend outage is not a logout.
    const timeout = res.status === 200 ? Date.parse(res.body?.timeout ?? '') : NaN;
    if (!Number.isNaN(timeout)) {
        deadline = timeout;
    }
    schedule(onExpired);
}

function schedule(onExpired: () => void): void {
    if (timer !== undefined) {
        window.clearTimeout(timer);
    }
    const delay = Math.min(
        Math.max(deadline - Date.now() - KEEP_ALIVE_BEFORE_DEADLINE_MS, KEEP_ALIVE_MIN_BEAT_MS),
        KEEP_ALIVE_MAX_BEAT_MS,
    );
    timer = window.setTimeout(() => void beat(onExpired), delay);
}
