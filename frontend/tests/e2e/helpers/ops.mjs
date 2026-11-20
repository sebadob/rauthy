// Helpers for the operational admin sections: sessions, events, blacklist, KV store and PAM.
// Everything works on roles and accessible names first and only falls back to text or document
// order where the original UI does not expose a usable name yet (see the FINDING comments).
import { expect } from '@playwright/test';
import { api } from './api.mjs';
import { unique } from './env.mjs';
import { loginWithPassword } from './login.mjs';

/** A password that satisfies the default password policy of the dev backend. */
export const OPS_PASSWORD = 'E2e-Ops-Pwd-12345';

/** Storage state without any session, for contexts that act as a different user. */
export const NO_SESSION = { cookies: [], origins: [] };

/**
 * Creates a Rauthy user with a password through the admin API, ready to log in.
 * @param {import('@playwright/test').Page} adminPage
 * @param {string} [prefix]
 */
export async function createUser(adminPage, prefix = 'e2e-ops') {
    const id = unique(prefix).toLowerCase();
    const email = `${id}@localhost.test`;
    const preferredUsername = `u${id.replace(/[^a-z0-9]/g, '')}`.slice(0, 40);
    /** @type {any} */
    const user = await api(adminPage, 'POST', 'users', {
        email,
        given_name: 'Ops',
        family_name: 'Tester',
        roles: [],
        language: 'en',
        preferred_username: preferredUsername,
    });
    // The dev backend does not store the preferred username given on creation and its PUT
    // validation always demands one, so it is set explicitly and the password is patched in.
    await api(adminPage, 'PUT', `users/${user.id}/self/preferred_username`, {
        preferred_username: preferredUsername,
        force_overwrite: true,
    });
    await api(adminPage, 'PATCH', `users/${user.id}`, {
        put: [
            { key: 'password', value: OPS_PASSWORD },
            { key: 'email_verified', value: true },
        ],
        del: [],
    });
    return { id: /** @type {string} */ (user.id), email, password: OPS_PASSWORD };
}

/** @param {import('@playwright/test').Page} adminPage @param {string} userId */
export async function deleteUser(adminPage, userId) {
    try {
        await api(adminPage, 'DELETE', `users/${userId}`);
    } catch (e) {
        if (!/-> 404/.test(String(e))) throw e;
    }
}

/**
 * Logs `user` into the account dashboard inside a fresh browser context without the admin
 * session and returns that page, its session info and a close function.
 * @param {import('@playwright/test').Browser} browser
 * @param {string | undefined} baseURL
 * @param {{ email: string, password: string }} user
 */
export async function loginInNewContext(browser, baseURL, user) {
    const ctx = await browser.newContext({ baseURL, storageState: NO_SESSION, locale: 'en-US' });
    const page = await ctx.newPage();
    await loginWithPassword(page, { ...user, startPath: '/auth/v1/account' });
    await expect(page).toHaveURL(/\/auth\/v1\/account/, { timeout: 30_000 });
    const res = await page.request.get('/auth/v1/oidc/sessioninfo');
    expect(res.status()).toBe(200);
    /** @type {{ id: string, user_id: string }} */
    const info = await res.json();
    return { page, ctx, info, close: () => ctx.close() };
}

/**
 * The search box of a list. The original UI names it "Search"; the clear button next to it is
 * named "Delete", which collides with real delete buttons (FINDING).
 * @param {import('@playwright/test').Page | import('@playwright/test').Locator} scope
 */
export function searchBox(scope) {
    return scope.getByRole('searchbox', { name: /search/i }).first();
}

/**
 * A tab of a tab bar. The original UI renders tabs as plain buttons with `aria-current="step"`
 * (FINDING: should be `tablist` / `tab` with `aria-selected`), so both are accepted.
 * @param {import('@playwright/test').Page | import('@playwright/test').Locator} scope
 * @param {string} name
 */
export function tab(scope, name) {
    return scope
        .getByRole('tab', { name, exact: true })
        .or(scope.getByRole('button', { name, exact: true }))
        .first();
}

/**
 * Asserts that `tabLocator` is the selected tab (either `aria-selected` or `aria-current`).
 * @param {import('@playwright/test').Locator} tabLocator
 */
export async function expectTabSelected(tabLocator) {
    await expect
        .poll(async () => {
            const sel = await tabLocator.getAttribute('aria-selected');
            const cur = await tabLocator.getAttribute('aria-current');
            return sel === 'true' || (!!cur && cur !== 'false');
        })
        .toBe(true);
}

/**
 * Entry of a navigation list (e.g. KV namespaces, PAM users) that is selected by clicking it.
 * Accepts links, options and buttons, so a redesign may use any of them.
 * @param {import('@playwright/test').Page | import('@playwright/test').Locator} scope
 * @param {string | RegExp} name
 */
export function navEntry(scope, name) {
    return scope
        .getByRole('link', { name })
        .or(scope.getByRole('option', { name }))
        .or(scope.getByRole('button', { name }))
        .first();
}

/**
 * The delete control that belongs to the list entry identified by `entry` (the entry's own
 * button / text). Prefers a delete button whose accessible name mentions `label`; the original UI
 * only offers an unnamed icon button directly after the entry (FINDING), which is the fallback.
 * @param {import('@playwright/test').Page} page
 * @param {import('@playwright/test').Locator} entry
 * @param {string} label
 */
export async function rowDeleteButton(page, entry, label) {
    const named = page.getByRole('button', {
        name: new RegExp(`(delete|remove).*${escapeRe(label)}`, 'i'),
    });
    if (await named.count()) return named.first();
    return entry.locator('xpath=following::button[1]');
}

/** Opens a combobox and picks `value` (exact). Supports options and listbox buttons. */
export async function pick(
    /** @type {import('@playwright/test').Page} */ page,
    /** @type {import('@playwright/test').Locator} */ combobox,
    /** @type {string} */ value,
) {
    await combobox.click();
    const listbox = page.getByRole('listbox').filter({ visible: true }).last();
    await expect(listbox).toBeVisible();
    const option = listbox.getByRole('option', { name: value, exact: true });
    if (await option.count()) {
        await option.first().click();
    } else {
        await listbox.getByRole('button', { name: value, exact: true }).first().click();
    }
    await expect(combobox).toContainText(value);
}

/** @param {string} s */
export function escapeRe(s) {
    return s.replace(/[.*+?^${}()|[\]\\]/g, '\\$&');
}

/**
 * Formats a unix timestamp (seconds) the way the admin UI shows it: `YYYY/MM/DD HH:mm:ss` in
 * the browser time zone of the Playwright config.
 * @param {number} ts
 * @param {string} [timeZone]
 */
export function fmtTs(ts, timeZone = 'Europe/Berlin') {
    const parts = Object.fromEntries(
        new Intl.DateTimeFormat('en-GB', {
            timeZone,
            year: 'numeric',
            month: '2-digit',
            day: '2-digit',
            hour: '2-digit',
            minute: '2-digit',
            second: '2-digit',
            hourCycle: 'h23',
        })
            .formatToParts(new Date(ts * 1000))
            .map(p => [p.type, p.value]),
    );
    return `${parts.year}/${parts.month}/${parts.day} ${parts.hour}:${parts.minute}:${parts.second}`;
}

/**
 * Vertical position of the first visible match, used to assert the visual order of list
 * entries without depending on the markup.
 * @param {import('@playwright/test').Locator} loc
 */
export async function topOf(loc) {
    const box = await loc.first().boundingBox();
    if (!box) throw new Error('element has no bounding box');
    return box.y;
}

/**
 * Reads the clipboard of the page (the context needs clipboard permissions).
 * @param {import('@playwright/test').Page} page
 */
export async function readClipboard(page) {
    return page.evaluate(() => navigator.clipboard.readText());
}

/**
 * Grants clipboard access for the app origin.
 * @param {import('@playwright/test').Page} page
 * @param {string | undefined} baseURL
 */
export async function allowClipboard(page, baseURL) {
    await page
        .context()
        .grantPermissions(['clipboard-read', 'clipboard-write'], { origin: baseURL });
}
