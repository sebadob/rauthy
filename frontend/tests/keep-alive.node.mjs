// Built-UI contract test for the idle keep-alive of the pages that already fetch their session:
// the ticker must never duplicate that first request, it is scheduled from the deadline the session
// answer carries.
import assert from 'node:assert/strict';
import { after, before, test } from 'node:test';
import { launchChromium, readBuiltHtml, serveUi } from './helpers/browser.mjs';

/** @type {any} */
let browser;
/** @type {any} */
let html;
before(async () => {
    html = await readBuiltHtml('account.html');
    browser = await launchChromium();
});
after(async () => browser?.close());

const documentPath = '/auth/v1/account';
const sessionInfoPath = '/auth/v1/oidc/sessioninfo';
const keepAlivePath = '/auth/v1/keep_alive';
const userPath = '/auth/v1/users/account-user';

const SESSION = {
    id: 'session-id',
    csrf_token: 'csrf-token',
    user_id: 'account-user',
    roles: '',
    groups: '',
    state: 'Auth',
};

const USER = {
    id: 'account-user',
    email: 'account@localhost',
    language: 'en',
    roles: [],
    groups: [],
    enabled: true,
    email_verified: true,
    created_at: 1700000000,
    account_type: 'password',
    user_values: {},
};

test('account: the ticker is scheduled from the session answer, it never duplicates it', async t => {
    const context = await browser.newContext({ serviceWorkers: 'block', locale: 'en-US' });
    t.after(() => context.close());
    const page = await context.newPage();
    /** @type {string[]} */
    const requests = [];
    /** @type {number[]} */
    const sessionInfos = [];
    /** @type {number[]} */
    const beats = [];
    page.on(
        'request',
        /** @param {any} request */ request => {
            const url = new URL(request.url());
            if (request.method() !== 'GET') {
                return;
            }
            if (url.pathname === sessionInfoPath) {
                sessionInfos.push(Date.now());
            }
            if (url.pathname === keepAlivePath) {
                beats.push(Date.now());
            }
        },
    );
    await serveUi({
        context,
        documentPath,
        html,
        requests,
        handle: (
            /** @type {any} */ _route,
            /** @type {any} */ _request,
            /** @type {any} */ url,
        ) => {
            if (url.pathname === sessionInfoPath) {
                // The idle deadline is 70 s away, so the ticker's first beat lands on its 10 s floor.
                return {
                    json: {
                        ...SESSION,
                        exp: new Date(Date.now() + 3600_000).toISOString(),
                        timeout: new Date(Date.now() + 70_000).toISOString(),
                    },
                };
            }
            if (url.pathname === userPath) {
                // Without its user the account page redirects to the login page, and the ticker of
                // this document would die with it.
                return { json: USER };
            }
            if (url.pathname === keepAlivePath) {
                return { json: { timeout: new Date(Date.now() + 70_000).toISOString() } };
            }
        },
    });

    const t0 = Date.now();
    await page.goto(`http://localhost${documentPath}`);

    // The page's own request is the only one at load: the deadline it answers is 70 s away, so the
    // first beat of the ticker is the 10 s floor of `deadline - 60 s`, not page load.
    await page.waitForTimeout(3_000);
    assert.equal(sessionInfos.length, 1, `sessioninfo requests: ${sessionInfos.length}`);
    assert.equal(beats.length, 0, `beats at load: ${beats.length}`);

    await page.waitForTimeout(9_000);
    assert.ok(beats.length >= 1, `expected a scheduled beat - got ${beats.length}`);
    const first = beats[0] - t0;
    assert.ok(first >= 9_000 && first <= 12_000, `the beat runs at deadline - 60 s: ${first} ms`);
    assert.equal(sessionInfos.length, 1, `sessioninfo requests: ${sessionInfos.length}`);

    t.after(() => context.close());
});
