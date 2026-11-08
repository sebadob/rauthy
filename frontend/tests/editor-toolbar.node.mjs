// Built-UI test for the markdown editor toolbar, used in the admin "Send E-Mail" dialog: a button
// must apply its markdown to the text the user selected before clicking it.
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

const TOOLS = [
    ['Heading 1', '# hello'],
    ['Bold', '**hello**'],
    ['Italic', '*hello*'],
    ['List', '- hello'],
    ['Code', '`hello`'],
];

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

test('toolbar buttons apply their markdown to the selection', async () => {
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

        for (const [name, expected] of TOOLS) {
            await editor.fill('hello');
            await editor.press('ControlOrMeta+a');
            await dialog.getByRole('button', { name, exact: true }).click();
            assert.equal(await editor.innerText(), expected, name);
        }
    } finally {
        await context.close();
    }
});
