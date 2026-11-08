// Built-UI test for the "Add New" provider dialog: a lookup after "Reset" must still work.
// All requests are intercepted; no backend needs to run.
import assert from 'node:assert/strict';
import { after, before, test } from 'node:test';
import {
    injectTemplates,
    launchChromium,
    ORIGIN,
    readBuiltHtml,
    serveUi,
} from './helpers/browser.mjs';

const DOCUMENT_PATH = '/auth/v1/admin/providers';
const ISSUER = 'https://idp.example.com';

const SESSION = {
    id: 'session-id',
    csrf_token: 'csrf-token',
    user_id: 'admin-id',
    roles: 'rauthy_admin',
    groups: '',
    exp: '1700003600',
    timeout: 'session_long',
    state: 'Auth',
};

const LOOKUP = {
    issuer: `${ISSUER}/`,
    authorization_endpoint: `${ISSUER}/authorize`,
    token_endpoint: `${ISSUER}/token`,
    userinfo_endpoint: `${ISSUER}/userinfo`,
    scope: 'openid profile email',
    use_pkce: true,
    client_secret_basic: true,
    client_secret_post: true,
};

/** @type {any} */
let browser;
/** @type {string} */
let html;

before(async () => {
    browser = await launchChromium();
    html = injectTemplates(await readBuiltHtml('admin/providers.html'), {});
});

after(async () => {
    await browser?.close();
});

test('a lookup after "Reset" shows the discovered config again', async () => {
    const context = await browser.newContext({ serviceWorkers: 'block' });
    /** @type {string[]} */
    const requests = [];
    await serveUi({
        context,
        documentPath: DOCUMENT_PATH,
        html,
        requests,
        handle: (/** @type {any} */ _route, /** @type {any} */ request, /** @type {URL} */ url) => {
            if (url.pathname === '/auth/v1/oidc/sessioninfo') return { json: SESSION };
            if (url.pathname === '/auth/v1/auth_check_admin') return { status: 200, body: '' };
            if (request.method() === 'POST' && url.pathname === '/auth/v1/providers') {
                return { json: [] };
            }
            if (url.pathname === '/auth/v1/providers/lookup') return { json: LOOKUP };
        },
    });

    const page = await context.newPage();
    // Svelte reports the broken binding through the console (`props_invalid_value`). Requests the
    // test does not care about are answered with 404, so those console errors are ignored.
    /** @type {string[]} */
    const errors = [];
    page.on('pageerror', (/** @type {Error} */ err) => errors.push(err.message));
    page.on('console', (/** @type {any} */ msg) => {
        if (msg.type() === 'error' && msg.text().includes('svelte.dev/e/')) errors.push(msg.text());
    });

    try {
        await page.goto(`${ORIGIN}${DOCUMENT_PATH}`);
        await page.getByRole('button', { name: 'Add New' }).click();
        const dialog = page.getByRole('dialog');
        const issuer = dialog.getByRole('textbox', { name: 'Issuer URL' });
        const lookup = dialog.getByRole('button', { name: 'Lookup' });
        const save = dialog.getByRole('button', { name: 'Save' });

        await issuer.fill(ISSUER);
        await lookup.click();
        await save.waitFor();

        await dialog.getByRole('button', { name: 'Reset' }).click();
        await issuer.fill(ISSUER);
        await lookup.click();
        await save.waitFor({ timeout: 5_000 });

        assert.deepEqual(errors, []);
        assert.deepEqual(
            requests.filter(r => r === 'POST /auth/v1/providers/lookup'),
            ['POST /auth/v1/providers/lookup', 'POST /auth/v1/providers/lookup'],
        );
    } finally {
        await context.close();
    }
});
