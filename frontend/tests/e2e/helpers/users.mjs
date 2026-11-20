// Helpers for the users admin page. Locators work on roles and accessible names. Where the
// original UI does not expose a usable name, a documented fallback is used (see users.spec.mjs).
import { expect } from '@playwright/test';
import { showEntryList } from './ui.mjs';
import { api } from './api.mjs';
import { AUTH_STATE, MAILCRAB_URL, unique } from './env.mjs';

/** A 1x1 PNG, enough for the picture upload. */
export const PNG_1PX = Buffer.from(
    'iVBORw0KGgoAAAANSUhEUgAAAAEAAAABCAYAAAAfFcSJAAAADUlEQVR42mNkYPhfDwAChwGA60e6kgAAAABJRU5ErkJggg==',
    'base64',
);

/** A password that satisfies the dev password policy (min 14, lower, upper, digit). */
export function strongPassword() {
    return `E2e-Pw-${Math.random().toString(36).slice(2, 8)}-X9${Date.now().toString(36)}`;
}

/**
 * Opens a page with the admin session outside the test-scoped `page`, for `beforeAll` /
 * `afterAll` arrangement through the REST API.
 * @param {import('@playwright/test').Browser} browser
 * @param {import('@playwright/test').TestInfo} testInfo
 */
export async function adminApiPage(browser, testInfo) {
    const ctx = await browser.newContext({
        baseURL: testInfo.project.use.baseURL,
        storageState: AUTH_STATE,
    });
    const page = await ctx.newPage();
    await page.goto('/auth/v1/account');
    return page;
}

/**
 * Creates a user through the API with a unique e-mail.
 * @param {import('@playwright/test').Page} page
 * @param {Record<string, any>} [data]
 */
export async function createUser(page, data = {}) {
    return api(page, 'POST', 'users', {
        email: `${unique('e2e-user')}@example.com`,
        given_name: 'E2e',
        family_name: 'User',
        language: 'en',
        roles: [],
        ...data,
    });
}

/** Deletes a user and ignores a user that is already gone. */
export async function deleteUser(
    /** @type {import('@playwright/test').Page} */ page,
    /** @type {string} */ id,
) {
    try {
        await api(page, 'DELETE', `users/${id}`);
    } catch (e) {
        if (!/-> 404/.test(String(e))) throw e;
    }
}

/** Returns the user or `undefined` when it does not exist. */
export async function findUser(
    /** @type {import('@playwright/test').Page} */ page,
    /** @type {string} */ id,
) {
    try {
        return await api(page, 'GET', `users/${id}`);
    } catch (e) {
        if (/-> 404/.test(String(e))) return undefined;
        throw e;
    }
}

/** Entries of the user list: one button per user, named by avatar initials, e-mail and name. */
export function userEntries(/** @type {import('@playwright/test').Page} */ page) {
    return page.getByRole('main').getByRole('button', { name: /@/ });
}

/** The user list entry for `email`. */
export function userEntry(
    /** @type {import('@playwright/test').Page} */ page,
    /** @type {string} */ email,
) {
    return userEntries(page).filter({ hasText: email });
}

/** E-mails of the visible list entries, in display order. */
export async function visibleEmails(/** @type {import('@playwright/test').Page} */ page) {
    const names = await userEntries(page).allInnerTexts();
    return names.map(n => {
        const m = n.match(/[^\s]+@[^\s]+/);
        return m ? m[0] : n;
    });
}

/**
 * On narrow screens the user list sits in a collapsible sub navigation. Opens it when the search
 * box is not visible yet.
 * FINDING: the toggle button has no accessible name (`aria-expanded` sits on the inner svg), so
 * the fallback is the first button inside `main`.
 * @param {import('@playwright/test').Page} page
 */
export async function openUserList(page) {
    await showEntryList(page);
    await expect(page.getByRole('searchbox', { name: 'Search' })).toBeVisible();
}

/**
 * Selects a detail tab. The original renders tabs as buttons with `aria-current`, a redesign
 * may use `role=tab`. Several icon buttons share the names "Delete" and "Logout", so a button
 * fallback must also carry the name as visible text.
 * @param {import('@playwright/test').Page} page
 * @param {string} name
 */
export function detailTab(page, name) {
    const main = page.getByRole('main');
    return main
        .getByRole('tab', { name, exact: true })
        .or(
            main
                .getByRole('button', { name, exact: true })
                .filter({ hasText: new RegExp(`^\\s*${name}\\s*$`) }),
        )
        .first();
}

/**
 * Opens the roles / groups chooser of a `SelectList` and returns its dialog.
 * FINDING: both edit buttons are icon buttons named only "Edit" (from a `title`), they do not
 * say which list they edit. The fallback picks them by position among the "Edit" titles.
 * @param {import('@playwright/test').Page | import('@playwright/test').Locator} scope
 * @param {'Roles' | 'Groups'} list
 */
export async function openSelectList(scope, list) {
    const named = scope.getByRole('button', { name: `Edit ${list}` });
    const btn = (await named.count())
        ? named.first()
        : scope.getByTitle('Edit', { exact: true }).nth(list === 'Roles' ? 0 : 1);
    await btn.click();
    const page = 'page' in scope ? scope.page() : scope;
    const dialog = page
        .getByRole('dialog')
        .filter({ has: page.getByRole('heading', { name: list }) });
    await expect(dialog).toBeVisible();
    return dialog;
}

/** Closes a `SelectList` dialog with its text "Close" button. */
export async function closeSelectList(/** @type {import('@playwright/test').Locator} */ dialog) {
    await dialog.getByRole('button', { name: 'Close', exact: true }).last().click();
    await expect(dialog).toBeHidden();
}

/** IDs of all mails Mailcrab holds for `to`. */
export async function mailIdsTo(
    /** @type {import('@playwright/test').APIRequestContext} */ request,
    /** @type {string} */ to,
) {
    const list = await (await request.get(`${MAILCRAB_URL}/api/messages`)).json();
    return list
        .filter((/** @type {any} */ m) => m.to.some((/** @type {any} */ t) => t.email === to))
        .map((/** @type {any} */ m) => m.id);
}

/**
 * Waits for a mail to `to` that is not in `known` and returns it with its body.
 * @param {import('@playwright/test').APIRequestContext} request
 * @param {{ to: string, known: string[], subject?: RegExp }} opts
 */
export async function waitForNewMail(request, { to, known, subject }) {
    /** @type {any} */
    let found;
    await expect
        .poll(
            async () => {
                const list = await (await request.get(`${MAILCRAB_URL}/api/messages`)).json();
                found = list.find(
                    (/** @type {any} */ m) =>
                        !known.includes(m.id) &&
                        m.to.some((/** @type {any} */ t) => t.email === to) &&
                        (!subject || subject.test(m.subject)),
                );
                return !!found;
            },
            { timeout: 30_000, message: `new mail to ${to}` },
        )
        .toBe(true);
    return (await request.get(`${MAILCRAB_URL}/api/message/${found.id}`)).json();
}

/**
 * Collects the requests a page sends to `pathRe` with `method`, to assert that a blocked form
 * never reached the backend.
 * @param {import('@playwright/test').Page} page
 * @param {string} method
 * @param {RegExp} pathRe
 */
export function trackRequests(page, method, pathRe) {
    /** @type {string[]} */
    const seen = [];
    page.on('request', req => {
        if (req.method() === method && pathRe.test(new URL(req.url()).pathname)) {
            seen.push(req.url());
        }
    });
    return seen;
}
