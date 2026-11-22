// Built-UI contract tests for the password prompt, credential errors and session recovery.
import assert from 'node:assert/strict';
import { after, before, test } from 'node:test';
import { injectTemplates, launchChromium, readBuiltHtml, serveUi } from './helpers/browser.mjs';

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

const consentContent = {
    source: 'local',
    content_hash: 'a'.repeat(64),
    name: null,
    scopes: [
        { name: 'email', title: null, description: null, warning: false },
        { name: 'groups', title: null, description: null, warning: false },
        { name: 'openid', title: null, description: null, warning: false },
    ],
    intro: null,
    terms_url: null,
    default_scopes: ['openid', 'email'],
    granted_scopes: [],
};

test('login: the consent block locks the defaults and blocks the login until confirmed', async t => {
    const context = await browser.newContext({ serviceWorkers: 'block', locale: 'en-US' });
    t.after(() => context.close());
    const page = await context.newPage();
    /** @type {string[]} */
    const requests = [];
    /** @type {any[]} */
    const logins = [];
    const changed = { ...consentContent, content_hash: 'b'.repeat(64) };
    await serveUi({
        context,
        documentPath,
        html: injectTemplates(html, { tpl_consent_content: JSON.stringify(consentContent) }),
        requests,
        handle: (/** @type {any} */ _route, /** @type {any} */ request, /** @type {any} */ url) => {
            if (url.pathname === documentPath && request.method() === 'POST') {
                logins.push(request.postDataJSON());
                // the first login finds the content changed meanwhile
                return logins.length === 1
                    ? {
                          status: 400,
                          json: { error: 'consent_required', message: JSON.stringify(changed) },
                      }
                    : { status: 400, json: { error: 'x', message: 'done' } };
            }
        },
    });
    await page.goto(
        `http://localhost${documentPath}?client_id=test&redirect_uri=http%3A%2F%2Flocalhost%2Fcb` +
            '&scope=openid%20groups',
    );
    await page.getByText('Consent', { exact: true }).waitFor();
    await page.locator('input[name=email]').fill('user@localhost');
    const submit = page.locator('form button[type=submit]');
    assert.ok(await submit.isDisabled(), 'the login waits for the confirmation');

    assert.ok(await page.getByLabel('openid', { exact: true }).isDisabled());
    assert.ok(await page.getByLabel('openid', { exact: true }).isChecked());
    assert.ok(!(await page.getByLabel('groups', { exact: true }).isChecked()));
    await page.getByText('Details').first().click();
    await page.getByText('Your E-Mail address').waitFor();

    await page.getByLabel('groups', { exact: true }).click();
    await page.getByLabel('I agree').click();
    await submit.click();
    await page.getByText('The consent content has changed, please check it again.').waitFor();
    assert.deepEqual(logins[0].consent, {
        content_hash: 'a'.repeat(64),
        accepted_scopes: ['email', 'groups', 'openid'],
    });

    // the changed content needs a new confirmation and is sent with its own hash
    assert.ok(await submit.isDisabled());
    await page.getByLabel('I agree').click();
    await submit.click();
    await page.getByText('done').waitFor();
    assert.deepEqual(logins[1].consent, {
        content_hash: 'b'.repeat(64),
        accepted_scopes: ['email', 'openid'],
    });
});

/**
 * Opens the login page with the given templates and query, `handle` answers the requests, and
 * every `POST /path` with its JSON body is recorded in `posts`.
 * @param {{ templates: Record<string, string>, query?: string, path?: string, handle?: (request: any, url: any) => any }} opts
 */
async function openConsentPage({ templates, query = '', path = documentPath, handle }) {
    const context = await browser.newContext({ serviceWorkers: 'block', locale: 'en-US' });
    // the PoW worker needs the backend, a stub answers it at once
    await context.addInitScript(() => {
        // @ts-ignore
        window.Worker = class {
            /** @type {any} */
            onmessage = null;
            postMessage() {
                setTimeout(() => this.onmessage?.({ data: 'pow' }), 0);
            }
            terminate() {}
        };
    });
    const page = await context.newPage();
    /** @type {string[]} */
    const requests = [];
    /** @type {{ path: string, body: any }[]} */
    const posts = [];
    await serveUi({
        context,
        documentPath: path,
        html: injectTemplates(
            await readBuiltHtml(`${path.slice('/auth/v1/'.length)}.html`),
            templates,
        ),
        requests,
        handle: (/** @type {any} */ _route, /** @type {any} */ request, /** @type {any} */ url) => {
            if (request.method() === 'POST') {
                posts.push({ path: url.pathname, body: request.postDataJSON() });
            }
            if (request.method() === 'POST' && url.pathname === '/auth/v1/pow') {
                return { status: 200, contentType: 'text/plain', body: 'challenge' };
            }
            return handle?.(request, url);
        },
    });
    await page.goto(
        `http://localhost${path}?client_id=test&redirect_uri=http%3A%2F%2Flocalhost%2Fcb${query}`,
    );
    /** @param {string} p */
    const postsTo = p => posts.filter(r => r.path === p);
    return { context, page, requests, posts, postsTo };
}

const consentRequired = (/** @type {any} */ content) => ({
    status: 400,
    json: { error: 'consent_required', message: JSON.stringify(content) },
});

const refreshPath = '/auth/v1/oidc/authorize/refresh';

test('login: a consent_required on the automatic refresh shows the consent block once', async t => {
    const { context, page, postsTo } = await openConsentPage({
        templates: {
            tpl_login_action: 'Refresh',
            tpl_consent_content: JSON.stringify(consentContent),
        },
        query: '&scope=openid%20groups',
        handle: (/** @type {any} */ request, /** @type {any} */ url) => {
            if (url.pathname === refreshPath) {
                return postsTo(refreshPath).length === 1
                    ? consentRequired({ ...consentContent, granted_scopes: ['groups'] })
                    : { status: 400, json: { error: 'x', message: 'refreshed' } };
            }
        },
    });
    t.after(() => context.close());

    const cont = page.getByRole('button', { name: 'Continue' });
    await cont.waitFor();
    await page.waitForTimeout(500);
    assert.equal(postsTo(refreshPath).length, 1, 'the refresh is sent once');
    assert.equal(await page.locator('input[name=email]').count(), 0, 'no credential form');
    assert.ok(await page.getByLabel('groups', { exact: true }).isChecked(), 'granted is checked');
    assert.ok(await cont.isDisabled());

    await page.getByLabel('I agree').click();
    await cont.click();
    await page.getByText('refreshed').waitFor();
    assert.equal(postsTo(refreshPath).length, 2);
    assert.deepEqual(postsTo(refreshPath)[1].body.consent, {
        content_hash: consentContent.content_hash,
        accepted_scopes: ['email', 'groups', 'openid'],
    });
});

test('login: a failed consent refresh falls back to the login form', async t => {
    const { context, page } = await openConsentPage({
        templates: {
            tpl_login_action: 'RefreshConsent',
            tpl_consent_content: JSON.stringify(consentContent),
            tpl_auth_providers: JSON.stringify([{ id: 'p1', name: 'Upstream', updated: 0 }]),
        },
        handle: (/** @type {any} */ request, /** @type {any} */ url) => {
            if (url.pathname === refreshPath) {
                return { status: 403, json: { error: 'Forbidden', message: 'session gone' } };
            }
        },
    });
    t.after(() => context.close());

    const cont = page.getByRole('button', { name: 'Continue' });
    await cont.waitFor();
    assert.equal(await page.locator('input[name=email]').count(), 0);
    await page.getByLabel('I agree').click();
    await cont.click();
    await page.getByText('session gone').waitFor();
    await page.locator('input[name=email]').waitFor();
    await page.getByRole('button', { name: 'Login: Upstream' }).waitFor();
});

test('login: with a consent block, the idp_hint login waits for Continue', async t => {
    const { context, page, postsTo } = await openConsentPage({
        templates: {
            tpl_consent_content: JSON.stringify(consentContent),
            tpl_auth_providers: JSON.stringify([{ id: 'p1', name: 'Upstream', updated: 0 }]),
        },
        query: '&scope=openid%20groups&idp_hint=p1',
        handle: (/** @type {any} */ request, /** @type {any} */ url) => {
            if (url.pathname === '/auth/v1/providers/login') {
                return { status: 400, json: { error: 'x', message: 'provider login' } };
            }
        },
    });
    t.after(() => context.close());

    const cont = page.getByRole('button', { name: 'Continue' });
    await cont.waitFor();
    await page.getByLabel('I agree').click();
    await page.waitForTimeout(500);
    assert.equal(postsTo('/auth/v1/providers/login').length, 0, 'no login on the checkbox');

    await cont.click();
    await page.getByText('provider login').waitFor();
    const logins = postsTo('/auth/v1/providers/login');
    assert.equal(logins.length, 1);
    assert.equal(logins[0].body.provider_id, 'p1');
    assert.deepEqual(logins[0].body.consent, {
        content_hash: consentContent.content_hash,
        accepted_scopes: ['email', 'openid'],
    });
});

test('login: a consent_required provider login shows the content to confirm', async t => {
    const changed = { ...consentContent, content_hash: 'b'.repeat(64) };
    const { context, page, postsTo } = await openConsentPage({
        templates: {
            tpl_auth_providers: JSON.stringify([{ id: 'p1', name: 'Upstream', updated: 0 }]),
        },
        handle: (/** @type {any} */ request, /** @type {any} */ url) => {
            if (url.pathname === '/auth/v1/providers/login') {
                return consentRequired(changed);
            }
        },
    });
    t.after(() => context.close());

    await page.getByRole('button', { name: 'Login: Upstream' }).click();
    await page.getByText('The consent content has changed, please check it again.').waitFor();
    assert.ok(await page.getByRole('button', { name: 'Login: Upstream' }).isDisabled());
    assert.equal(postsTo('/auth/v1/providers/login').length, 1);
});

test('login: a remote scope description shows as plain text only', async t => {
    const remote = {
        ...consentContent,
        source: 'remote',
        name: 'Calendar API',
        scopes: [
            {
                name: 'groups',
                title: 'Groups',
                description:
                    '<div style="position:fixed;inset:0;background:red">overlay</div>' +
                    '<a href="https://evil.example">click</a><img src=x onerror="window.pwned=1">',
                warning: false,
            },
            { name: 'openid', title: null, description: null, warning: false },
        ],
        default_scopes: ['openid'],
    };
    const { context, page } = await openConsentPage({
        templates: {
            tpl_client_name: 'My App',
            // escaped like the backend does it for the `<template>`
            tpl_consent_content: JSON.stringify(remote)
                .replaceAll('<', '\\u003c')
                .replaceAll('>', '\\u003e')
                .replaceAll('&', '\\u0026'),
        },
        query: '&scope=openid%20groups',
    });
    t.after(() => context.close());

    await page.getByText('My App requests access to Calendar API:', { exact: true }).waitFor();
    await page.getByText('Details').click();
    const details = page.getByText('overlayclick', { exact: true });
    await details.waitFor();
    assert.equal(await details.locator('*').count(), 0, 'no element from the description');
    assert.equal(await page.locator('[style*="fixed"]').count(), 0);
    assert.equal(await page.evaluate(() => /** @type {any} */ (window).pwned), undefined);
});

test('login: the consent choices survive the password reset view', async t => {
    const { context, page } = await openConsentPage({
        templates: {
            tpl_consent_content: JSON.stringify(consentContent),
            tpl_password_reset_link_always: 'true',
        },
        query: '&scope=openid%20groups',
        handle: (/** @type {any} */ request, /** @type {any} */ url) => {
            if (url.pathname === '/auth/v1/users/request_reset') {
                return { status: 200, json: {} };
            }
        },
    });
    t.after(() => context.close());

    await page.getByLabel('groups', { exact: true }).click();
    await page.getByLabel('I agree').click();
    await page.locator('input[name=email]').fill('user@localhost');
    await page.getByText('Password forgotten?').click();
    assert.ok(!(await page.getByLabel('I agree').isVisible()), 'hidden during the reset');
    await page.getByRole('button', { name: 'Request', exact: true }).click();
    // the reset view closes 3 s after the email was sent
    const submit = page.locator('form button[type=submit]');
    await submit.waitFor({ timeout: 6000 });
    assert.ok(await page.getByLabel('groups', { exact: true }).isChecked());
    assert.ok(await page.getByLabel('I agree').isChecked());
    assert.ok(!(await submit.isDisabled()));
});

test('provider callback: a consent_required returns to the page the login started from', async t => {
    const loginPage = `http://localhost${documentPath}?client_id=test&redirect_uri=x`;
    const { context, page } = await openConsentPage({
        templates: {},
        path: '/auth/v1/providers/callback',
        query: '&state=s&code=c',
        handle: (/** @type {any} */ request, /** @type {any} */ url) => {
            if (url.pathname === '/auth/v1/providers/callback') {
                return consentRequired(consentContent);
            }
            if (url.pathname === documentPath) {
                return { status: 200, contentType: 'text/html', body: '<p>login page</p>' };
            }
        },
    });
    t.after(() => context.close());
    await page.evaluate(
        (/** @type {string} */ p) => sessionStorage.setItem('provider_login_page', p),
        loginPage,
    );
    await page.reload();
    await page.waitForURL(loginPage);
    assert.equal(await page.evaluate(() => sessionStorage.getItem('provider_login_page')), null);
});

test('provider callback: a consent_required without a login page shows a message', async t => {
    const { context, page } = await openConsentPage({
        templates: {},
        path: '/auth/v1/providers/callback',
        query: '&state=s&code=c',
        handle: (/** @type {any} */ request, /** @type {any} */ url) => {
            if (url.pathname === '/auth/v1/providers/callback') {
                return consentRequired(consentContent);
            }
        },
    });
    t.after(() => context.close());
    await page.getByText('The consent content has changed, please check it again.').waitFor();
    assert.equal(await page.getByText('consent_required').count(), 0);
});

test('register: a provider sign-up collects the consent it is asked for', async t => {
    const { context, page, postsTo } = await openConsentPage({
        templates: {
            tpl_auth_providers: JSON.stringify([{ id: 'p1', name: 'Upstream', updated: 0 }]),
        },
        path: '/auth/v1/users/register',
        handle: (/** @type {any} */ request, /** @type {any} */ url) => {
            if (url.pathname === '/auth/v1/providers/login') {
                return postsTo(url.pathname).length === 1
                    ? consentRequired(consentContent)
                    : { status: 400, json: { error: 'x', message: 'signed up' } };
            }
        },
    });
    t.after(() => context.close());

    const provider = page.getByRole('button', { name: /Upstream$/ });
    await provider.click();
    await page.getByText('Consent', { exact: true }).waitFor();
    assert.equal(postsTo('/auth/v1/providers/login')[0].body.consent, undefined);
    assert.ok(await provider.isDisabled(), 'the sign-up waits for the confirmation');

    await page.getByLabel('I agree').click();
    await provider.click();
    await page.getByText('signed up').waitFor();
    assert.deepEqual(postsTo('/auth/v1/providers/login')[1].body.consent, {
        content_hash: consentContent.content_hash,
        accepted_scopes: ['email', 'openid'],
    });
});
