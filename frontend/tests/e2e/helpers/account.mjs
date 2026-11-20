// Helpers for the account dashboard and the public flows: dedicated test users that are created
// and removed through the admin API, logins as such a user, WebAuthn with a virtual authenticator
// and the OAuth device flow from the device's point of view.
import { createHash, generateKeyPairSync, randomBytes } from 'node:crypto';
import { expect, request as playwrightRequest } from '@playwright/test';
import { api } from './api.mjs';
import { AUTH_STATE, unique } from './env.mjs';
import { firstRauthyLink, waitForMail } from './mail.mjs';

/** A password that satisfies the password policy of the dev backend (min 14, a-z, A-Z, 0-9). */
export const TEST_PASSWORD = 'E2e-Test-Pwd-1234';

/** A browser context without any session, used for the public pages and the test user. */
export const NO_SESSION = { cookies: [], origins: [] };

/**
 * Requests the account dashboard sends for every user and that fail by design. They belong to
 * `EXPECTED_HTTP_ERRORS` in `fixtures.mjs` (see the report); until then specs allow them.
 */
// One regex on purpose: `test.use()` reads a 2-element array as `[value, fixtureOptions]`.
export const ACCOUNT_EXPECTED_ERRORS = [
    // 404: a user without a PAM account (the dashboard probes for it to decide on the PAM tab)
    // 406: the dashboard asks whether the user may delete the own account; admins may not
    /^http (404 GET \/auth\/v1\/pam\/users\/self|406 GET \/auth\/v1\/users\/[^/]+\/self\/delete)$/,
];

/** `now` in unix seconds, minus a second of clock tolerance for Mailcrab timestamps. */
export const nowSecs = () => Math.floor(Date.now() / 1000) - 1;

/**
 * Creates a user through the admin API and gives it a password, so it can log in right away.
 * `preferredUsername: null` leaves the required preferred username empty on purpose.
 * @param {import('@playwright/test').Page} adminPage a page with the logged-in admin session
 * @param {{ password?: string | null, preferredUsername?: string | null, givenName?: string,
 *   familyName?: string, prefix?: string, roles?: string[] }} [opts]
 */
export async function createTestUser(adminPage, opts = {}) {
    const id = unique(opts.prefix || 'e2e-acc').toLowerCase();
    const email = `${id}@localhost.test`;
    const preferredUsername =
        opts.preferredUsername === undefined
            ? `u${id.replace(/[^a-z0-9]/g, '')}`.slice(0, 40)
            : opts.preferredUsername;
    const givenName = opts.givenName || 'Testy';
    const familyName = opts.familyName || 'Tester';
    const since = nowSecs();
    /** @type {any} */
    const user = await api(adminPage, 'POST', 'users', {
        email,
        given_name: givenName,
        family_name: familyName,
        language: 'en',
        roles: opts.roles || [],
        ...(preferredUsername ? { preferred_username: preferredUsername } : {}),
    });
    const password = opts.password === undefined ? TEST_PASSWORD : opts.password;
    if (password) {
        // `PUT /users/{id}` cannot set it: it always rejects the request while the preferred
        // username is configured as required (see the report). The magic link of the
        // "New Password" mail sets it instead, with plain HTTP.
        const request = await playwrightRequest.newContext({
            baseURL: new URL(adminPage.url()).origin,
        });
        try {
            await setPasswordFromNewUserMail(request, user.id, email, since, password);
        } finally {
            await request.dispose();
        }
    }
    return {
        id: /** @type {string} */ (user.id),
        email,
        password: /** @type {string} */ (password || ''),
        preferredUsername,
        givenName,
        familyName,
    };
}

/**
 * Reads the value the backend renders into a `<template id="...">` of a server-rendered page.
 * @param {string} html
 * @param {string} id
 */
export function readTemplate(html, id) {
    const m = html.match(new RegExp(`<template id="${id}">([\\s\\S]*?)</template>`));
    if (!m) throw new Error(`template ${id} not found`);
    return m[1].replaceAll('&quot;', '"').replaceAll('&amp;', '&');
}

/**
 * Sets the first password through the magic link of the "New Password" mail.
 * @param {import('@playwright/test').APIRequestContext} request
 * @param {string} userId
 * @param {string} email
 * @param {number} since
 * @param {string} password
 */
export async function setPasswordFromNewUserMail(request, userId, email, since, password) {
    const mail = await waitForMail(request, { to: email, subject: /New Password/, since });
    const html = await (await request.get(firstRauthyLink(mail))).text();
    const tpl = JSON.parse(readTemplate(html, 'tpl_password_reset'));
    const res = await request.put(`/auth/v1/users/${userId}/reset`, {
        headers: { 'content-type': 'application/json', 'x-pwd-csrf-token': tpl.csrf_token },
        data: JSON.stringify({ password, magic_link_id: tpl.magic_link_id }),
    });
    if (!res.ok()) throw new Error(`password for ${email}: ${res.status()} ${await res.text()}`);
}

/** @param {import('@playwright/test').Page} adminPage @param {string} userId */
export async function deleteTestUser(adminPage, userId) {
    try {
        await api(adminPage, 'DELETE', `users/${userId}`);
    } catch (e) {
        // already gone, e.g. after an account self-deletion
        if (!/-> 404/.test(String(e))) throw e;
    }
}

/** Finds a user by e-mail through the admin API. */
export async function findUserByEmail(
    /** @type {import('@playwright/test').Page} */ adminPage,
    /** @type {string} */ email,
) {
    /** @type {any[]} */
    const users = await api(adminPage, 'GET', 'users');
    return users.find(u => u.email === email);
}

/**
 * Opens a page with the admin session in its own context, to arrange and verify data while the
 * test page itself is logged in as the test user or nobody. It never runs UI assertions.
 * @param {import('@playwright/test').Browser} browser
 * @param {string} baseURL
 */
export async function openAdminPage(browser, baseURL) {
    const ctx = await browser.newContext({ baseURL, storageState: AUTH_STATE });
    const page = await ctx.newPage();
    await page.goto('/auth/v1/account');
    return { page, close: () => ctx.close() };
}

/** The e-mail field of the login page. */
export const emailField = (/** @type {import('@playwright/test').Page} */ page) =>
    page.getByRole('textbox', { name: 'E-Mail' });

/**
 * The primary "Login" button. The Passkey button on the login page has the same accessible name
 * "Login" (see the report), so the one without the "Passkey" text is the submit button.
 */
export const loginButton = (/** @type {import('@playwright/test').Page} */ page) =>
    // the Passkey button carries the same name: the submit button of the login form is the one
    page.locator('form').getByRole('button', { name: 'Login', exact: true });

/** The "Login" button that starts the Passkey (discoverable credential) login. */
export const passkeyLoginButton = (/** @type {import('@playwright/test').Page} */ page) =>
    page.getByRole('button', { name: 'Login', exact: true }).filter({ hasText: 'Passkey' });

/**
 * Types the e-mail once the page is interactive. The login button is enabled by the component
 * state, so it only turns enabled once the page has hydrated and the value is bound.
 * @param {import('@playwright/test').Page} page
 * @param {string} email
 */
export async function fillLoginEmail(page, email) {
    await expect(async () => {
        await emailField(page).fill(email);
        await expect(loginButton(page)).toBeEnabled({ timeout: 1_000 });
    }).toPass({ timeout: 20_000 });
}

/**
 * Logs in on the login page the browser is currently showing (or `startPath`).
 * @param {import('@playwright/test').Page} page
 * @param {string} email
 * @param {string} password
 * @param {string} [startPath]
 */
export async function loginOnPage(page, email, password, startPath) {
    if (startPath) await page.goto(startPath);
    await fillLoginEmail(page, email);
    await emailField(page).press('Enter');
    const pwd = page.getByLabel('Password', { exact: true });
    await expect(pwd).toBeVisible({ timeout: 20_000 });
    await pwd.fill(password);
    await pwd.press('Enter');
}

/**
 * Logs the test user into the account dashboard and waits until it rendered.
 * @param {import('@playwright/test').Page} page
 * @param {{ email: string, password: string }} user
 */
export async function loginToAccount(page, user) {
    await loginOnPage(page, user.email, user.password, '/auth/v1/account');
    await expect(page).toHaveURL(/\/auth\/v1\/account/, { timeout: 30_000 });
    await expect(page.getByText(user.email, { exact: true }).first()).toBeVisible();
}

/**
 * A tab of the account dashboard. The tabs are plain buttons that mark the active one with
 * `aria-current` instead of `role=tab` / `aria-selected` (see the report).
 * @param {import('@playwright/test').Page} page
 * @param {string} name
 */
export const accountTab = (page, name) => page.getByRole('button', { name, exact: true }).first();

/**
 * Enables WebAuthn for the page and adds a CTAP2 platform authenticator with user verification,
 * which answers every ceremony automatically.
 * @param {import('@playwright/test').Page} page
 */
export async function addVirtualAuthenticator(page) {
    const cdp = await page.context().newCDPSession(page);
    await cdp.send('WebAuthn.enable');
    const { authenticatorId } = await cdp.send('WebAuthn.addVirtualAuthenticator', {
        options: {
            protocol: 'ctap2',
            transport: 'internal',
            hasResidentKey: true,
            hasUserVerification: true,
            isUserVerified: true,
            automaticPresenceSimulation: true,
        },
    });
    return {
        /** @returns {Promise<any[]>} */
        credentials: async () =>
            (await cdp.send('WebAuthn.getCredentials', { authenticatorId })).credentials,
    };
}

/**
 * The origin the backend accepts WebAuthn ceremonies from (`webauthn.rp_origin`). The dev
 * `config.toml` sets it to the Vite dev server `http://localhost:5173`, while the E2E backend
 * serves the UI on `E2E_BASE_URL`. A browser signs the page origin into every ceremony, so the
 * passkey tests open the pages under the RP origin and forward each request to the backend.
 */
export const RP_ORIGIN = process.env.E2E_WEBAUTHN_ORIGIN || 'http://localhost:5173';

/**
 * Serves `RP_ORIGIN` from the backend at `baseURL` for this context, so a page opened under the
 * RP origin is the very same production UI and backend. Cookies are shared, because cookies do
 * not depend on the port; the CSRF token lives in `localStorage` and is copied over.
 * @param {import('@playwright/test').BrowserContext} context
 * @param {string} baseURL
 */
export async function serveUnderRpOrigin(context, baseURL) {
    const backend = new URL(baseURL).origin;
    if (RP_ORIGIN === backend) return (/** @type {string} */ path) => path;
    await context.route(`${RP_ORIGIN}/**`, async route => {
        const req = route.request();
        const headers = { ...req.headers() };
        if (headers.origin) headers.origin = backend;
        if (headers.referer) headers.referer = headers.referer.replace(RP_ORIGIN, backend);
        const res = await route.fetch({
            url: req.url().replace(RP_ORIGIN, backend),
            headers,
            maxRedirects: 0,
        });
        // Absolute `location` headers keep pointing to the backend on purpose: after a login the
        // OIDC callback must run on the origin that holds the PKCE verifier.
        await route.fulfill({ response: res });
    });
    return (/** @type {string} */ path) => `${RP_ORIGIN}${path}`;
}

/**
 * Copies the CSRF token of the logged-in backend origin into the RP origin.
 * @param {import('@playwright/test').Page} page currently on the backend origin, logged in
 */
export async function copyCsrfTokenToRpOrigin(page) {
    const sct = await page.evaluate(() => localStorage.getItem('sct'));
    await page.context().addInitScript(
        ([origin, value]) => {
            if (location.origin === origin && value) localStorage.setItem('sct', value);
        },
        [RP_ORIGIN, sct],
    );
}

/**
 * Waits until the credential stuffing detection forgot the previous failed login of this IP.
 * `cred_stuff_detection.scan_window = '5s'` and `blacklist_threshold = 3` in the dev config.
 */
export async function coolDownCredStuffWindow(/** @type {import('@playwright/test').Page} */ page) {
    await page.waitForTimeout(6_000);
}

/** A fresh, unique `ssh-ed25519` public key in OpenSSH format. */
export function newSshPublicKey(/** @type {string} */ comment) {
    const { publicKey } = generateKeyPairSync('ed25519');
    const raw = publicKey.export({ format: 'der', type: 'spki' }).subarray(-32);
    const str = (/** @type {Buffer} */ b) => {
        const len = Buffer.alloc(4);
        len.writeUInt32BE(b.length);
        return Buffer.concat([len, b]);
    };
    const blob = Buffer.concat([str(Buffer.from('ssh-ed25519')), str(raw)]);
    return `ssh-ed25519 ${blob.toString('base64')} ${comment}`;
}

/**
 * Creates a public client through the admin API with the given extra properties, e.g. the
 * enabled flows. The client JSON from the API is a valid update request.
 * @param {import('@playwright/test').Page} adminPage
 * @param {Record<string, any>} changes
 */
export async function createTestClient(adminPage, changes) {
    const id = unique('e2e-pub').toLowerCase();
    await api(adminPage, 'POST', 'clients', {
        id,
        name: 'E2E Public Flows',
        confidential: false,
        redirect_uris: ['http://localhost:8080/auth/v1/e2e-callback'],
    });
    const client = await api(adminPage, 'GET', `clients/${id}`);
    await api(adminPage, 'PUT', `clients/${id}`, { ...client, ...changes });
    return id;
}

export const DEVICE_GRANT = 'urn:ietf:params:oauth:grant-type:device_code';

/**
 * Starts the device authorization grant like a device would.
 * `device_grant.rate_limit = '3s'` limits this per IP, so it retries after a 429.
 * @param {import('@playwright/test').APIRequestContext} request
 * @param {string} clientId
 */
export async function startDeviceAuthorization(request, clientId) {
    /** @type {any} */
    let body;
    await expect(async () => {
        const res = await request.post('/auth/v1/oidc/device', { form: { client_id: clientId } });
        expect(res.status(), await res.text()).toBe(200);
        body = await res.json();
    }).toPass({ intervals: [1_000, 2_000, 3_000], timeout: 20_000 });
    return body;
}

/**
 * Polls the token endpoint once with the device code, like the device does.
 * @param {import('@playwright/test').APIRequestContext} request
 * @param {string} clientId
 * @param {string} deviceCode
 */
export async function pollDeviceToken(request, clientId, deviceCode) {
    const res = await request.post('/auth/v1/oidc/token', {
        form: { grant_type: DEVICE_GRANT, client_id: clientId, device_code: deviceCode },
    });
    return { status: res.status(), body: await res.json() };
}

/** A PKCE verifier and S256 challenge. */
export function pkce() {
    const verifier = randomBytes(48).toString('base64url');
    const challenge = createHash('sha256').update(verifier).digest('base64url');
    return { verifier, challenge };
}
