// Helpers for the admin list/detail sections (clients, scopes, attributes). Everything works on
// roles and accessible names. Where the current UI does not expose a proper role yet, a helper
// prefers the correct role first and falls back to the closest non-CSS alternative, so the same
// test keeps working once the markup is fixed.
import { expect } from '@playwright/test';
import { showEntryList } from './ui.mjs';
import { api } from './api.mjs';

/** Escapes a string for use inside a `RegExp`. */
export const reEscape = (/** @type {string} */ s) => s.replace(/[.*+?^${}()|[\]\\]/g, '\\$&');

/** The secondary navigation inside `main` that holds Add New, order, search and the entries. */
export function subNav(/** @type {import('@playwright/test').Page} */ page) {
    return page.getByRole('main').getByRole('navigation').first();
}

/**
 * Makes sure the secondary navigation is expanded. At phone size it is collapsed behind a toggle
 * button that has no accessible name (FINDING), so the fallback is the button that controls it.
 * @param {import('@playwright/test').Page} page
 */
export async function openSubNav(page) {
    await showEntryList(page);
    await expect(subNav(page).getByRole('button', { name: 'Add New' })).toBeVisible();
}

/** All entry buttons of the list in the secondary navigation. */
export function entries(/** @type {import('@playwright/test').Page} */ page) {
    const nav = subNav(page);
    return nav
        .getByRole('listitem')
        .getByRole('button')
        .or(nav.getByRole('listbox').getByRole('option'));
}

/**
 * The list entry whose accessible name starts with `key` (clients show "id name", scopes
 * "name default").
 * @param {import('@playwright/test').Page} page
 * @param {string} key
 */
export function entry(page, key) {
    return entries(page).filter({ hasText: new RegExp(`(^|\\s)${reEscape(key)}(\\s|$)`) });
}

/**
 * Asserts that a list entry is the selected one. The selection is announced with
 * `aria-current="page"` on the entry or on its list item.
 * @param {import('@playwright/test').Locator} locator
 */
export async function expectEntryCurrent(locator) {
    await expect
        .poll(() =>
            locator.evaluate(
                el => el.closest('[aria-current]')?.getAttribute('aria-current') ?? null,
            ),
        )
        .toBe('page');
}

/**
 * The key (ID or name) of every visible entry, in display order. Entries show a title and, below
 * it, the key; entries without a separate title show only the key.
 */
export async function entryKeys(/** @type {import('@playwright/test').Page} */ page) {
    const texts = await entries(page).allInnerTexts();
    return texts.map(t => {
        const lines = t
            .split('\n')
            .map(l => l.trim())
            .filter(Boolean);
        return (lines.at(-1) || '').split(/\s+/)[0];
    });
}

/**
 * A tab of the detail view. The current UI renders tabs as plain buttons with
 * `aria-current="step"` (FINDING), so the fallback is a button with that visible text. The text
 * filter keeps the icon-only "Delete" button of the search field out.
 * @param {import('@playwright/test').Page} page
 * @param {string} name
 */
export function tab(page, name) {
    const main = page.getByRole('main');
    return main
        .getByRole('tab', { name, exact: true })
        .or(main.getByRole('button', { name, exact: true }).filter({ hasText: name }))
        .first();
}

/**
 * The selected state of a tab: `aria-selected` for real tabs, `aria-current` for the current UI.
 * @param {import('@playwright/test').Locator} t
 */
export async function expectTabSelected(t) {
    await expect
        .poll(async () => {
            const sel = await t.getAttribute('aria-selected');
            const cur = await t.getAttribute('aria-current');
            return sel === 'true' || (!!cur && cur !== 'false');
        })
        .toBe(true);
}

/** Confirms an entity deletion and returns the DELETE status. */
export async function clickDeleteAndConfirm(
    /** @type {import('@playwright/test').Page} */ page,
    /** @type {string} */ pathPart,
) {
    await page.locator('.admin-entity-header').getByRole('button', { name: 'Delete' }).click();
    const del = page.waitForResponse(
        r => r.request().method() === 'DELETE' && r.url().includes(pathPart),
    );
    await page
        .getByRole('dialog', { name: 'Delete' })
        .getByRole('button', { name: 'Delete' })
        .click();
    return (await del).status();
}

/**
 * Opens the "Add New" dialog of the current section and returns it.
 * @param {import('@playwright/test').Page} page
 */
export async function openAddNew(page) {
    await openSubNav(page);
    await subNav(page).getByRole('button', { name: 'Add New' }).click();
    const dialog = page.getByRole('dialog');
    await expect(dialog).toBeVisible();
    return dialog;
}

/**
 * The "Save" button of the visible detail form (not the one inside a dialog).
 * @param {import('@playwright/test').Page} page
 */
export function saveButton(page) {
    return page.getByRole('main').getByRole('button', { name: 'Save', exact: true }).last();
}

/**
 * Clicks a Save button and waits for the request it triggers.
 * @param {import('@playwright/test').Page} page
 * @param {import('@playwright/test').Locator} button
 * @param {'PUT'|'POST'} method
 * @param {string} pathPart
 */
export async function saveAndWait(page, button, method, pathPart) {
    const res = page.waitForResponse(
        r => r.request().method() === method && r.url().includes(pathPart),
    );
    await button.click();
    const r = await res;
    expect(r.status(), `${method} ${pathPart}`).toBeLessThan(400);
    return r;
}

/**
 * Counts requests matching `method` and `pathPart` while `fn` runs, plus a short settle time,
 * to prove that a click did NOT send anything (validation blocked the save).
 * @param {import('@playwright/test').Page} page
 * @param {string} method
 * @param {string} pathPart
 * @param {() => Promise<void>} fn
 */
export async function countRequests(page, method, pathPart, fn) {
    let n = 0;
    /** @param {import('@playwright/test').Request} r */
    const on = r => {
        if (r.method() === method && r.url().includes(pathPart)) n += 1;
    };
    page.on('request', on);
    try {
        await fn();
        // nothing to wait for when the save is blocked: give a request a moment to show up
        await page.waitForTimeout(500);
    } finally {
        page.off('request', on);
    }
    return n;
}

/**
 * Asserts that an input reports itself as invalid, either through ARIA or native validity.
 * @param {import('@playwright/test').Locator} input
 */
export async function expectInvalid(input) {
    await expect
        .poll(async () => {
            const aria = await input.getAttribute('aria-invalid');
            const valid = await input.evaluate(
                el => /** @type {HTMLInputElement} */ (el).validity?.valid ?? true,
            );
            return aria === 'true' || !valid;
        })
        .toBe(true);
}

/**
 * The tag input for a multi value field (redirect URIs, contacts, …): the textbox to add values
 * and the remove buttons of existing values.
 * @param {import('@playwright/test').Page | import('@playwright/test').Locator} scope
 * @param {string} label
 */
export function tagInput(scope, label) {
    const input = scope.getByRole('textbox', { name: label, exact: true });
    // The values and the input share one list. `has` is resolved inside each list, so it needs a
    // locator that starts at the page, not at `scope`.
    const root = 'page' in scope ? scope.page() : scope;
    const list = scope
        .getByRole('list')
        .filter({ has: root.getByRole('textbox', { name: label, exact: true }) });
    return {
        input,
        list,
        /** @param {string} value */
        item: value => list.getByRole('listitem').filter({ hasText: value }),
        /** @param {string} value */
        remove: async value => {
            const item = list.getByRole('listitem').filter({ hasText: value });
            const named = item.getByRole('button', { name: new RegExp(`remove|delete`, 'i') });
            await ((await named.count()) ? named : item.getByRole('button')).first().click();
        },
        /** @param {string} value */
        add: async value => {
            await input.fill(value);
            await input.press('Enter');
        },
    };
}

/**
 * Creates a client through the API (for tests where creation is not the subject).
 * @param {import('@playwright/test').Page} page
 * @param {string} id
 * @param {Record<string, any>} [extra] fields for a follow-up PUT
 */
export async function createClient(page, id, extra) {
    await api(page, 'POST', 'clients', {
        id,
        name: 'E2E Client',
        confidential: true,
        redirect_uris: ['http://localhost:9999/callback'],
    });
    if (extra) {
        const c = await api(page, 'GET', `clients/${id}`);
        await api(page, 'PUT', `clients/${id}`, { ...toUpdate(c), ...extra });
    }
    return api(page, 'GET', `clients/${id}`);
}

/** Converts a `ClientResponse` into an `UpdateClientRequest`. */
export function toUpdate(/** @type {any} */ c) {
    return {
        name: c.name,
        enabled: c.enabled,
        confidential: c.confidential,
        redirect_uris: c.redirect_uris,
        post_logout_redirect_uris: c.post_logout_redirect_uris,
        allowed_origins: c.allowed_origins,
        flows_enabled: c.flows_enabled,
        access_token_alg: c.access_token_alg,
        id_token_alg: c.id_token_alg,
        access_token_lifetime: c.access_token_lifetime,
        auth_code_lifetime: c.auth_code_lifetime,
        scopes: c.scopes,
        default_scopes: c.default_scopes,
        challenges: c.challenges,
        force_mfa: c.force_mfa,
        client_uri: c.client_uri,
        contacts: c.contacts,
        backchannel_logout_uri: c.backchannel_logout_uri,
        restrict_group_prefix: c.restrict_group_prefix,
        claims: c.claims,
        claims_at_root: c.claims_at_root,
        allowed_resources: c.allowed_resources,
        default_aud: c.default_aud,
        scim: c.scim,
    };
}

/** Deletes a client and ignores a 404 (the test may have deleted it already). */
export async function deleteClientQuietly(
    /** @type {import('@playwright/test').Page} */ page,
    /** @type {string} */ id,
) {
    try {
        await api(page, 'DELETE', `clients/${id}`);
    } catch (e) {
        if (!/-> 404/.test(String(e))) throw e;
    }
}

/**
 * A valid 8x8 PNG (solid red) built on the fly, so no binary fixture has to live in the repo.
 * @returns {Promise<Buffer>}
 */
export async function makePng() {
    const zlib = await import('node:zlib');
    // the backend rejects logos below 84px (clients) or 20px (providers)
    const w = 128;
    const h = 128;
    const raw = Buffer.alloc((w * 3 + 1) * h);
    for (let y = 0; y < h; y++) {
        const row = y * (w * 3 + 1);
        raw[row] = 0;
        for (let x = 0; x < w; x++) {
            raw[row + 1 + x * 3] = 255;
        }
    }
    const chunk = (/** @type {string} */ type, /** @type {Buffer} */ data) => {
        const len = Buffer.alloc(4);
        len.writeUInt32BE(data.length);
        const td = Buffer.concat([Buffer.from(type, 'ascii'), data]);
        const crc = Buffer.alloc(4);
        crc.writeUInt32BE(zlib.crc32(td) >>> 0);
        return Buffer.concat([len, td, crc]);
    };
    const ihdr = Buffer.alloc(13);
    ihdr.writeUInt32BE(w, 0);
    ihdr.writeUInt32BE(h, 4);
    ihdr[8] = 8; // bit depth
    ihdr[9] = 2; // RGB
    return Buffer.concat([
        Buffer.from([0x89, 0x50, 0x4e, 0x47, 0x0d, 0x0a, 0x1a, 0x0a]),
        chunk('IHDR', ihdr),
        chunk('IDAT', zlib.deflateSync(raw)),
        chunk('IEND', Buffer.alloc(0)),
    ]);
}
