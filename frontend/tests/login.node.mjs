// Built-UI contract test for the login page: only a 401 from POST /oidc/authorize means "needs a
// password" or "invalid credentials". Any other unexpected status must surface the server message.
import assert from 'node:assert/strict';
import { after, before, test } from 'node:test';
import { launchChromium, readBuiltHtml, serveUi } from './helpers/browser.mjs';

/** @type {any} */
let browser;
/** @type {any} */
let html;
before(async () => {
    html = await readBuiltHtml('oidc/authorize.html');
    browser = await launchChromium();
});
after(async () => browser?.close());

const documentPath = '/auth/v1/oidc/authorize';
const authorizeReq = `POST ${documentPath}`;
const keepAlivePath = '/auth/v1/keep_alive';

/**
 * Opens the login page. `handle` is the answer of `POST /oidc/authorize`, `keepAlive` the answer of
 * the idle ticker's `GET /auth/v1/keep_alive`, and every document navigation (the first `goto` and
 * any reload the page does itself) is recorded in `navigations`.
 * @param {number} status
 * @param {Record<string, string>} [headers]
 * @param {{ status: number, headers?: Record<string, string>, timeoutAfterMs?: number }} [keepAlive]
 */
async function openLogin(status, headers = {}, keepAlive = undefined) {
    const context = await browser.newContext({ serviceWorkers: 'block', locale: 'en-US' });
    const page = await context.newPage();
    /** @type {string[]} */
    const requests = [];
    /** @type {string[]} */
    const navigations = [];
    /** @type {number[]} */
    const beats = [];
    page.on(
        'request',
        /** @param {any} request */ request => {
            const url = new URL(request.url());
            if (request.method() !== 'GET') {
                return;
            }
            if (url.pathname === documentPath) {
                navigations.push(url.search);
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
        handle: (/** @type {any} */ _route, /** @type {any} */ request, /** @type {any} */ url) => {
            if (url.pathname === documentPath && request.method() === 'POST') {
                return {
                    status,
                    headers,
                    json: { error: 'SomeError', message: `server said ${status}` },
                };
            }
            if (url.pathname === keepAlivePath && request.method() === 'GET') {
                if (!keepAlive) {
                    return { status: 404 };
                }
                return {
                    status: keepAlive.status,
                    headers: keepAlive.headers ?? {},
                    json: {
                        timeout: new Date(
                            Date.now() + (keepAlive.timeoutAfterMs ?? 0),
                        ).toISOString(),
                    },
                };
            }
        },
    });
    await page.goto(
        `http://localhost${documentPath}?client_id=test&redirect_uri=http%3A%2F%2Flocalhost%2Fcb`,
    );
    await page.locator('input[name=email]').fill('user@localhost');
    const submit = () => page.locator('form button[type=submit]').click();
    return { context, page, requests, navigations, beats, submit };
}

for (const status of [404, 409, 413]) {
    test(`login: HTTP ${status} shows the server message, not a password prompt`, async t => {
        const { context, page, requests, submit } = await openLogin(status);
        t.after(() => context.close());
        await submit();
        await page.getByText(`server said ${status}`, { exact: true }).waitFor();
        assert.equal(await page.locator('input[name=password]').count(), 0);
        assert.equal(await page.getByText('Password forgotten?').count(), 0);
        assert.equal(await page.getByText('Invalid credentials').count(), 0);
        assert.ok(requests.includes(authorizeReq));
    });
}

test('login: HTTP 401 asks for the password, then reports invalid credentials', async t => {
    const { context, page, submit } = await openLogin(401);
    t.after(() => context.close());
    await submit();
    await page.locator('input[name=password]').waitFor();
    await page.locator('input[name=password]').fill('wrong-password');
    await submit();
    await page.getByText('Invalid credentials', { exact: true }).waitFor();
    await page.getByText('Password forgotten?').waitFor();
});

async function waitForReloads(/** @type {string[]} */ navigations, /** @type {number} */ want) {
    const deadline = Date.now() + 5000;
    while (navigations.length < want && Date.now() < deadline) {
        await new Promise(resolve => setTimeout(resolve, 50));
    }
}

test('login: an expired page session reloads itself, it never claims bad credentials', async t => {
    const { context, page, navigations, submit } = await openLogin(401, {
        'x-session-expired': 'true',
    });
    t.after(() => context.close());
    await submit();
    await waitForReloads(navigations, 2);
    assert.equal(navigations.length, 2, `document reloads: ${navigations.join(' | ')}`);
    assert.equal(await page.getByText('Invalid credentials').count(), 0);
});

test('login: the expired session reload runs once, then the server message shows', async t => {
    const { context, page, navigations, submit } = await openLogin(401, {
        'x-session-expired': 'true',
    });
    t.after(() => context.close());

    await submit();
    await waitForReloads(navigations, 2);
    assert.equal(navigations.length, 2, `document reloads: ${navigations.join(' | ')}`);

    // the restarted page is answered the same way - the guard must stop the loop
    await page.locator('input[name=email]').fill('user@localhost');
    await submit();
    await page.getByText('server said 401', { exact: true }).waitFor();
    assert.equal(navigations.length, 2, `document reloads: ${navigations.join(' | ')}`);
});

test('login: the idle ticker beats at the server deadline, never faster than 10 s', async t => {
    // The answer is a deadline 70 s away, so the next beat is the 10 s floor, not the 60 s before
    // the deadline - this is what pins both bounds at once.
    const { context, page, beats } = await openLogin(
        401,
        {},
        {
            status: 200,
            timeoutAfterMs: 70_000,
        },
    );
    t.after(() => context.close());

    await page.waitForTimeout(12_000);
    assert.ok(beats.length >= 2, `expected at least two beats - got ${beats.length}`);
    const gap = beats[1] - beats[0];
    assert.ok(gap >= 9_000 && gap <= 11_500, `the 10 s floor sets the gap - got ${gap} ms`);
});

test('login: the idle ticker restarts a dead page session before the form is used', async t => {
    const { context, page, navigations, beats } = await openLogin(
        401,
        {},
        {
            status: 401,
            headers: { 'x-session-expired': 'true' },
        },
    );
    t.after(() => context.close());

    // The first beat sees the dead session, the reload restarts the flow, and the second beat of
    // the reloaded page is inside the guard window, so the loop stops there.
    await waitForReloads(navigations, 2);
    assert.equal(navigations.length, 2, `document reloads: ${navigations.join(' | ')}`);
    await page.waitForTimeout(1_000);
    assert.equal(beats.length, 2, `beats: ${beats.length}`);
    assert.equal(navigations.length, 2, 'the guard must stop the reload loop');
    assert.equal(await page.getByText('Invalid credentials').count(), 0);
});
