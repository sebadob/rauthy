// Built-UI test for the admin "Send E-Mail" dialog: "Send" needs a subject and a body, and a body
// that was typed and then deleted again does not count.
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

const DOCUMENT_PATH = '/auth/v1/admin/users';

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

/** @type {Record<string, any>} */
const EMPTY_LISTS = {
    '/auth/v1/users': [],
    '/auth/v1/roles': [],
    '/auth/v1/groups': [],
    '/auth/v1/email': [],
    '/auth/v1/providers/minimal': [],
};

/** @type {any} */
let browser;
/** @type {string} */
let html;

before(async () => {
    browser = await launchChromium();
    html = injectTemplates(await readBuiltHtml('admin/users.html'), {});
});

after(async () => {
    await browser?.close();
});

test('Send is disabled again once the body is deleted', async () => {
    const context = await browser.newContext({ serviceWorkers: 'block' });
    await serveUi({
        context,
        documentPath: DOCUMENT_PATH,
        html,
        requests: [],
        handle: (/** @type {any} */ _route, /** @type {any} */ request, /** @type {URL} */ url) => {
            if (url.pathname === '/auth/v1/oidc/sessioninfo') return { json: SESSION };
            if (url.pathname === '/auth/v1/auth_check_admin') return { status: 200, body: '' };
            if (request.method() === 'GET' && url.pathname in EMPTY_LISTS) {
                return { json: EMPTY_LISTS[url.pathname] };
            }
        },
    });

    try {
        const page = await context.newPage();
        await page.goto(`${ORIGIN}${DOCUMENT_PATH}`);
        await page.getByRole('button', { name: 'Send E-Mail' }).click();
        const dialog = page.getByRole('dialog');
        const editor = dialog.getByRole('textbox', { name: 'Edit Text' });
        const send = dialog.getByRole('button', { name: 'Send' });

        await dialog.getByRole('textbox', { name: 'Subject' }).fill('Maintenance');
        await editor.fill('Hello');
        await assertDisabled(send, false);
        await editor.fill('');
        await assertDisabled(send, true);
        // the same when deleting the text like a user does
        await editor.fill('Hello');
        await assertDisabled(send, false);
        await editor.press('ControlOrMeta+a');
        await editor.press('Backspace');
        await assertDisabled(send, true);
    } finally {
        await context.close();
    }
});

// `Button` renders `isDisabled` as `aria-disabled`, the element itself stays enabled.
async function assertDisabled(/** @type {any} */ button, /** @type {boolean} */ disabled) {
    await button.page().waitForTimeout(100);
    assert.equal(await button.getAttribute('aria-disabled'), String(disabled));
}
