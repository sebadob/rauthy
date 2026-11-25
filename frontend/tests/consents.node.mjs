// Built-UI contract test for the consent views: the account's "Privacy" tab and the admin's user
// "Consents" tab. A revoke deletes all sessions of the user, so the account sends the user to the
// login right after it instead of an error, while an admin keeps the list of another user.
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

const USER_ID = 'consent-user';
const ADMIN_ID = 'admin-id';
const ACCOUNT_PATH = '/auth/v1/account';
const ADMIN_PATH = '/auth/v1/admin/users';
const AUTHORIZE_PATH = '/auth/v1/oidc/authorize';

const USER = {
    id: USER_ID,
    email: 'consent@localhost',
    language: 'en',
    roles: [],
    groups: [],
    enabled: true,
    email_verified: true,
    created_at: 1700000000,
    account_type: 'password',
    user_values: {},
};

const CONSENTS = [
    {
        client_id: 'app',
        client_name: 'My App',
        scopes: ['openid', 'email'],
        created: 1700000000,
        updated: 1700086400,
        user_agent: 'Mozilla/5.0 Test',
    },
    {
        client_id: 'api',
        resource: 'https://api.example.com',
        scopes: ['openid'],
        created: 1700000000,
        updated: 1700000000,
    },
];

const USER_VALUES_CONFIG = {
    given_name: 'optional',
    family_name: 'optional',
    birthdate: 'hidden',
    street: 'hidden',
    zip: 'hidden',
    city: 'hidden',
    country: 'hidden',
    phone: 'hidden',
    tz: 'hidden',
    self_service_email_change_deny: false,
    preferred_username: {
        preferred_username: 'hidden',
        immutable: false,
        blacklist: [],
        pattern_html: '',
        email_fallback: false,
    },
};

const LOGOUT_WARNING =
    'This also signs you out of all your sessions on every device. You will have to log in again.';
const SESSIONS_WARNING =
    'All sessions of the user are deleted as well, the user is logged out everywhere.';

/** @param {string} userId @param {string} roles */
function session(userId, roles) {
    return {
        id: 'session-id',
        csrf_token: 'csrf-token',
        user_id: userId,
        roles,
        groups: '',
        exp: '1700003600',
        timeout: 'session_long',
        state: 'Auth',
    };
}

/** @type {any} */
let browser;
/** @type {string} */
let accountHtml;
/** @type {string} */
let adminHtml;

before(async () => {
    browser = await launchChromium();
    accountHtml = injectTemplates(await readBuiltHtml('account.html'), {});
    adminHtml = injectTemplates(await readBuiltHtml('admin/users.html'), {});
});

after(async () => {
    await browser?.close();
});

/**
 * Serves one of the pages with the consents of `USER_ID`. A DELETE answers like the backend and
 * empties the list, an authorize request is recorded as a login redirect.
 *
 * @param {{ documentPath: string, html: string, session: any, consents: any[] }} opts
 */
async function open({ documentPath, html, session, consents }) {
    const context = await browser.newContext({ serviceWorkers: 'block' });
    const s = {
        context,
        /** @type {string[]} */
        requests: [],
        /** @type {string[]} */
        deletes: [],
        consents,
        /** @type {URL | undefined} */
        login: undefined,
    };
    await serveUi({
        context,
        documentPath,
        html,
        requests: s.requests,
        handle: (/** @type {any} */ _route, /** @type {any} */ request, /** @type {URL} */ url) => {
            const method = request.method();
            const path = url.pathname;
            if (path === AUTHORIZE_PATH) {
                s.login = url;
                return { contentType: 'text/html', body: '<p>login</p>' };
            }
            if (path === '/auth/v1/oidc/sessioninfo') return { json: session };
            if (path === '/auth/v1/auth_check_admin') return { status: 200, body: '' };
            if (method === 'GET' && path === '/auth/v1/users') return { json: [USER] };
            if (path === '/auth/v1/users/values_config') return { json: USER_VALUES_CONFIG };
            if (path === `/auth/v1/users/${USER_ID}`) return { json: USER };
            if (path === `/auth/v1/users/${USER_ID}/consents`) {
                if (method === 'GET') return { json: s.consents };
                s.deletes.push(`${path}?${url.searchParams}`);
                s.consents = [];
                return { json: { consent_deleted: true, refresh_tokens_deleted: 1 } };
            }
            if (method === 'DELETE' && path === `/auth/v1/users/${USER_ID}/consents/all`) {
                s.deletes.push(path);
                s.consents = [];
                return { json: { consents_deleted: 2, refresh_tokens_deleted: 1 } };
            }
        },
    });
    const page = await context.newPage();
    return { page, s };
}

/** @param {any} page */
async function openPrivacy(page) {
    await page.goto(`${ORIGIN}${ACCOUNT_PATH}`);
    await page.getByRole('button', { name: 'Privacy', exact: true }).click();
    await page.getByText('My App', { exact: true }).waitFor();
}

test('account: a single revoke warns about the logout and sends the user to the login', async () => {
    const { page, s } = await open({
        documentPath: ACCOUNT_PATH,
        html: accountHtml,
        session: session(USER_ID, ''),
        consents: CONSENTS,
    });
    try {
        await openPrivacy(page);
        await page.getByRole('button', { name: 'Revoke Access', exact: true }).first().click();
        const dialog = page.getByRole('dialog');
        await dialog.getByText(LOGOUT_WARNING).waitFor();
        await dialog.getByRole('button', { name: 'Revoke Access', exact: true }).click();
        await page.waitForURL((/** @type {URL} */ url) => url.pathname === AUTHORIZE_PATH);

        assert.deepEqual(s.deletes, [`/auth/v1/users/${USER_ID}/consents?client_id=app`]);
        assert.equal(s.login?.searchParams.get('state'), 'account');
    } finally {
        await s.context.close();
    }
});

test('account: revoke all asks first, then sends the user to the login', async () => {
    const { page, s } = await open({
        documentPath: ACCOUNT_PATH,
        html: accountHtml,
        session: session(USER_ID, ''),
        consents: CONSENTS,
    });
    try {
        await openPrivacy(page);
        await page.getByRole('button', { name: 'Revoke All Access', exact: true }).click();
        const dialog = page.getByRole('dialog');
        await dialog.getByText(LOGOUT_WARNING).waitFor();

        await dialog.getByRole('button', { name: 'Cancel', exact: true }).click();
        assert.deepEqual(s.deletes, [], 'cancel must not revoke anything');

        await page.getByRole('button', { name: 'Revoke All Access', exact: true }).click();
        await dialog.getByRole('button', { name: 'Revoke All Access', exact: true }).click();
        await page.waitForURL((/** @type {URL} */ url) => url.pathname === AUTHORIZE_PATH);

        assert.deepEqual(s.deletes, [`/auth/v1/users/${USER_ID}/consents/all`]);
        assert.equal(s.login?.searchParams.get('state'), 'account');
    } finally {
        await s.context.close();
    }
});

test('account: no revoke all without a consent', async () => {
    const { page, s } = await open({
        documentPath: ACCOUNT_PATH,
        html: accountHtml,
        session: session(USER_ID, ''),
        consents: [],
    });
    try {
        await page.goto(`${ORIGIN}${ACCOUNT_PATH}`);
        await page.getByRole('button', { name: 'Privacy', exact: true }).click();
        await page.getByText('No application has been granted access yet.').waitFor();
        assert.equal(
            await page.getByRole('button', { name: 'Revoke All Access', exact: true }).count(),
            0,
        );
    } finally {
        await s.context.close();
    }
});

test('admin: lists the consents of a user and revokes all of them', async () => {
    const { page, s } = await open({
        documentPath: ADMIN_PATH,
        html: adminHtml,
        session: session(ADMIN_ID, 'rauthy_admin'),
        consents: CONSENTS,
    });
    try {
        await page.goto(`${ORIGIN}${ADMIN_PATH}?uid=${USER_ID}&userTab=consents`);
        await page.getByText('My App', { exact: true }).waitFor();
        // the client id is shown next to the name, a nameless client is titled by its id
        await page.getByText('app', { exact: true }).waitFor();
        await page.getByText('https://api.example.com', { exact: true }).waitFor();
        await page.getByText('Mozilla/5.0 Test', { exact: true }).waitFor();
        const times = await page
            .locator('time')
            .evaluateAll((/** @type {any[]} */ els) => els.map(el => el.getAttribute('datetime')));
        assert.deepEqual(times, [
            '2023-11-14T22:13:20.000Z',
            '2023-11-15T22:13:20.000Z',
            '2023-11-14T22:13:20.000Z',
            '2023-11-14T22:13:20.000Z',
        ]);

        await page.getByRole('button', { name: 'Revoke All Access', exact: true }).click();
        const dialog = page.getByRole('dialog');
        await dialog.getByText(SESSIONS_WARNING).waitFor();
        await dialog.getByRole('button', { name: 'Revoke All Access', exact: true }).click();
        await page.getByText('This user has not given consent to any application.').waitFor();

        assert.deepEqual(s.deletes, [`/auth/v1/users/${USER_ID}/consents/all`]);
        assert.equal(s.login, undefined, 'revoking for another user keeps the admin logged in');
    } finally {
        await s.context.close();
    }
});
