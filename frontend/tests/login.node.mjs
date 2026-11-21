// Built-UI contract tests for the password prompt, credential errors and session recovery.
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

/** @typedef {{ status: number, error?: string, headers?: Record<string, string> }} Reply */
const passwordRequired = { status: 400, error: 'PasswordRequired' };

/**
 * The mock can return either one reply or distinct replies for the email and password steps.
 * @param {Reply | ((password: string | undefined) => Reply)} authorize
 * @param {{ status: number, headers?: Record<string, string>, timeoutAfterMs?: number }} [keepAlive]
 */
async function openLogin(authorize, keepAlive = undefined) {
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
                const { status, headers, error } =
                    typeof authorize === 'function'
                        ? authorize(request.postDataJSON().password)
                        : authorize;
                return {
                    status,
                    headers,
                    json: { error: error ?? 'SomeError', message: `server said ${status}` },
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
    const fillEmail = () => page.locator('input[name=email]').fill('user@localhost');
    const submit = () => page.locator('form button[type=submit]').click();
    return { context, page, requests, navigations, beats, fillEmail, submit };
}

for (const status of [400, 401, 404, 409, 413]) {
    test(`login: HTTP ${status} shows the server message, not a password prompt`, async t => {
        const { context, page, requests, fillEmail, submit } = await openLogin({ status });
        t.after(() => context.close());
        await fillEmail();
        await submit();
        await page.getByText(`server said ${status}`, { exact: true }).waitFor();
        assert.equal(await page.locator('input[name=password]').count(), 0);
        assert.equal(await page.getByText('Password forgotten?').count(), 0);
        assert.equal(await page.getByText('Invalid credentials').count(), 0);
        assert.ok(requests.includes(authorizeReq));
    });
}

test('login: PasswordRequired opens the password field and InvalidCredentials shows the error', async t => {
    const { context, page, requests, fillEmail, submit } = await openLogin(password =>
        password ? { status: 400, error: 'InvalidCredentials' } : passwordRequired,
    );
    t.after(() => context.close());
    await fillEmail();
    await submit();
    await page.locator('input[name=password]').waitFor();
    await page.locator('input[name=password]').fill('wrong-password');
    await submit();
    await page.getByText('Invalid credentials', { exact: true }).waitFor();
    await page.getByText('Password forgotten?').waitFor();
    assert.equal(requests.filter(r => r === authorizeReq).length, 2);
});

test('login: clicking submit with an invalid email does not send it', async t => {
    const { context, page, submit } = await openLogin(passwordRequired);
    t.after(() => context.close());
    /** @type {string[]} */
    const submittedEmails = [];
    page.on('request', (/** @type {any} */ request) => {
        if (request.method() === 'POST' && new URL(request.url()).pathname === documentPath) {
            submittedEmails.push(request.postDataJSON().email);
        }
    });
    const email = page.locator('input[name=email]');
    await email.fill('invalid-address');
    await submit();
    await page.getByText('Provide valid E-Mail address').waitFor();

    await email.fill('user@localhost');
    await submit();
    await page.locator('input[name=password]').waitFor();
    assert.deepEqual(submittedEmails, ['user@localhost']);
});

async function waitForReloads(/** @type {string[]} */ navigations, /** @type {number} */ want) {
    const deadline = Date.now() + 5000;
    while (navigations.length < want && Date.now() < deadline) {
        await new Promise(resolve => setTimeout(resolve, 50));
    }
}

test('login: an expired page session reloads itself, it never claims bad credentials', async t => {
    const { context, page, navigations, fillEmail, submit } = await openLogin({
        status: 401,
        headers: { 'x-session-expired': 'true' },
    });
    t.after(() => context.close());
    await fillEmail();
    await submit();
    await waitForReloads(navigations, 2);
    assert.equal(navigations.length, 2, `document reloads: ${navigations.join(' | ')}`);
    assert.equal(await page.getByText('Invalid credentials').count(), 0);
});

test('login: the expired session reload runs once, then the server message shows', async t => {
    const { context, page, navigations, fillEmail, submit } = await openLogin({
        status: 401,
        headers: { 'x-session-expired': 'true' },
    });
    t.after(() => context.close());

    await fillEmail();
    await submit();
    await waitForReloads(navigations, 2);
    assert.equal(navigations.length, 2, `document reloads: ${navigations.join(' | ')}`);

    // the restarted page is answered the same way - the guard must stop the loop
    await fillEmail();
    await submit();
    await page.getByText('server said 401', { exact: true }).waitFor();
    assert.equal(navigations.length, 2, `document reloads: ${navigations.join(' | ')}`);
});

test('login: the idle ticker beats at the server deadline, never faster than 10 s', async t => {
    // The answer is a deadline 70 s away, so the next beat is the 10 s floor, not the 60 s before
    // the deadline - this is what pins both bounds at once.
    const { context, page, beats } = await openLogin(
        { status: 401 },
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
        { status: 401 },
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
