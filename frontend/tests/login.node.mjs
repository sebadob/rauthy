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

/**
 * Opens the login page. `handle` is the answer of `POST /oidc/authorize`, and every document
 * navigation (the first `goto` and any reload the page does itself) is recorded in `navigations`.
 * @param {number} status
 * @param {Record<string, string>} [headers]
 */
async function openLogin(status, headers = {}) {
    const context = await browser.newContext({ serviceWorkers: 'block', locale: 'en-US' });
    const page = await context.newPage();
    /** @type {string[]} */
    const requests = [];
    /** @type {string[]} */
    const navigations = [];
    page.on(
        'request',
        /** @param {any} request */ request => {
            const url = new URL(request.url());
            if (request.method() === 'GET' && url.pathname === documentPath) {
                navigations.push(url.search);
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
        },
    });
    await page.goto(
        `http://localhost${documentPath}?client_id=test&redirect_uri=http%3A%2F%2Flocalhost%2Fcb`,
    );
    await page.locator('input[name=email]').fill('user@localhost');
    const submit = () => page.locator('form button[type=submit]').click();
    return { context, page, requests, navigations, submit };
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
