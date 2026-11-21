// Helpers for the config, docs and mass mail specs: raw REST calls that may fail on purpose,
// dedicated test users and request guards that keep these tests from touching shared data.
import { expect } from '@playwright/test';
import { api, csrfToken } from './api.mjs';
import { unique } from './env.mjs';

/**
 * Like `api()`, but returns the status instead of throwing, for requests that must be rejected.
 * @param {import('@playwright/test').Page} page
 * @param {'GET'|'POST'|'PUT'|'PATCH'|'DELETE'} method
 * @param {string} path relative to `/auth/v1/`
 * @param {any} [data]
 */
export async function apiRaw(page, method, path, data) {
    if (!page.url().startsWith('http')) await page.goto('/auth/v1/account');
    const res = await page.request.fetch(`/auth/v1/${path.replace(/^\//, '')}`, {
        method,
        headers: { 'x-csrf-token': await csrfToken(page), 'content-type': 'application/json' },
        data: data === undefined ? undefined : JSON.stringify(data),
    });
    const text = await res.text();
    return { status: res.status(), text };
}

/** The `PUT /password_policy` payload that recreates the given `GET` response exactly. */
export function policyRequest(/** @type {any} */ p) {
    /** @type {Record<string, number>} */
    const req = { length_min: p.length_min, length_max: p.length_max };
    for (const k of [
        'include_lower_case',
        'include_upper_case',
        'include_digits',
        'include_special',
        'not_recently_used',
        'valid_days',
    ]) {
        if (p[k]) req[k] = p[k];
    }
    return req;
}

/**
 * Creates a dedicated group with exactly one new user in it, so a mass mail filtered by this
 * group can only ever reach this user.
 * @param {import('@playwright/test').Page} page
 */
export async function createMailTarget(page) {
    const group = await api(page, 'POST', 'groups', { group: unique('e2e-mail') });
    const email = `${unique('e2e-mail')}@example.com`;
    const user = await api(page, 'POST', 'users', {
        email,
        given_name: 'Mail',
        family_name: 'Target',
        language: 'en',
        roles: [],
        groups: [group.name],
    });
    return {
        group,
        user,
        email,
        async cleanup() {
            await apiRaw(page, 'DELETE', `users/${user.id}`);
            await apiRaw(page, 'DELETE', `groups/${group.id}`);
        },
    };
}

/** A plain test user, e.g. to check that the password policy is enforced. */
export async function createUser(/** @type {import('@playwright/test').Page} */ page) {
    const email = `${unique('e2e-policy')}@example.com`;
    const user = await api(page, 'POST', 'users', {
        email,
        given_name: 'Policy',
        language: 'en',
        roles: [],
    });
    return { user, email };
}

/**
 * Records every request matching `method` and `path` so a test can assert that the UI did (or
 * did not) send it.
 * @param {import('@playwright/test').Page} page
 * @param {string} method
 * @param {RegExp} path
 */
export function recordRequests(page, method, path) {
    /** @type {any[]} */
    const seen = [];
    page.on('request', req => {
        if (req.method() === method && path.test(new URL(req.url()).pathname)) {
            seen.push(req.postDataJSON?.() ?? null);
        }
    });
    return seen;
}

/**
 * Lets `POST /email` through only for an `in_group` filter on `group`. Anything else (above all
 * a mail to every user) is aborted and fails the test.
 * @param {import('@playwright/test').Page} page
 * @param {string} group
 */
export async function guardMassMail(page, group) {
    await page.route(/\/auth\/v1\/email$/, async route => {
        if (route.request().method() !== 'POST') return route.continue();
        const body = route.request().postDataJSON();
        if (body.filter_type === 'in_group' && body.filter_value === group) {
            return route.continue();
        }
        await route.abort();
        expect(body, 'a mass mail may only target the dedicated test group').toMatchObject({
            filter_type: 'in_group',
            filter_value: group,
        });
    });
}

/**
 * Answers `POST /email` without sending anything and returns the captured payloads.
 * @param {import('@playwright/test').Page} page
 */
export async function mockMassMail(page) {
    /** @type {any[]} */
    const sent = [];
    await page.route(/\/auth\/v1\/email$/, async route => {
        if (route.request().method() !== 'POST') return route.continue();
        sent.push(route.request().postDataJSON());
        await route.fulfill({ status: 200, body: '' });
    });
    return sent;
}

/** Unix seconds now, for Mailcrab `since` filters. */
export const nowSecs = () => Math.floor(Date.now() / 1000);

/** `YYYY-MM-DD` of today + `days` in the browser time zone of the config (Europe/Berlin). */
export function dateInput(/** @type {number} */ days) {
    const d = new Date(Date.now() + days * 86_400_000);
    const parts = new Intl.DateTimeFormat('en-CA', {
        timeZone: 'Europe/Berlin',
        year: 'numeric',
        month: '2-digit',
        day: '2-digit',
    }).format(d);
    return parts;
}

/** Unix seconds of a local Europe/Berlin date and time, as the UI should compute it. */
export function berlinTs(/** @type {string} */ date, /** @type {string} */ time) {
    // Interpret the wall-clock time as UTC first, then correct by Berlin's offset at that moment.
    const asUtc = Date.parse(`${date}T${time}:00Z`);
    const offsetMin =
        (new Date(asUtc).getTime() -
            Date.parse(
                new Date(asUtc).toLocaleString('en-US', { timeZone: 'Europe/Berlin' }) + ' UTC',
            )) /
        60_000;
    return (asUtc + offsetMin * 60_000) / 1000;
}

/** Formats unix seconds like the UI (`formatDateFromTs`) for Europe/Berlin. */
export function fmtTs(/** @type {number} */ ts) {
    const p = Object.fromEntries(
        new Intl.DateTimeFormat('en-GB', {
            timeZone: 'Europe/Berlin',
            year: 'numeric',
            month: '2-digit',
            day: '2-digit',
            hour: '2-digit',
            minute: '2-digit',
            second: '2-digit',
            hour12: false,
        })
            .formatToParts(new Date(ts * 1000))
            .map(x => [x.type, x.value]),
    );
    return `${p.year}/${p.month}/${p.day} ${p.hour}:${p.minute}:${p.second}`;
}
