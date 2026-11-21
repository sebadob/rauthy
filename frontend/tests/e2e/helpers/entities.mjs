// Helpers for the admin sections built as "sub navigation list + detail view with tabs"
// (roles, groups, providers, API keys). Everything is selected by role and accessible name.
// Where the upstream UI does not expose a proper role yet, the fallback is documented inline.
import { expect } from '@playwright/test';
import { showEntryList } from './ui.mjs';
import { api } from './api.mjs';

/** @typedef {import('@playwright/test').Page} Page */

/** The secondary navigation of the section (Add New, order, search and the entry list). */
export function subNav(/** @type {Page} */ page) {
    return page.getByRole('main').getByRole('navigation').first();
}

/** The list of entries inside the secondary navigation. */
export function entryList(/** @type {Page} */ page) {
    return subNav(page).getByRole('list').first();
}

/** A single entry of the list, by its exact name. */
export function entry(/** @type {Page} */ page, /** @type {string} */ name) {
    return entryList(page).getByRole('button', { name, exact: true });
}

/**
 * Returns to the section list when a phone-size detail is open.
 */
export async function showSubNav(/** @type {Page} */ page) {
    await showEntryList(page);
    await expect(subNav(page)).toBeVisible();
}

/** Opens an entry from the list (expanding the list first on phones). */
export async function openEntry(/** @type {Page} */ page, /** @type {string} */ name) {
    await showSubNav(page);
    await entry(page, name).click();
}

/** Opens the "Add New" dialog of the section and returns it. */
export async function openAddNew(/** @type {Page} */ page) {
    await showSubNav(page);
    await subNav(page).getByRole('button', { name: 'Add New' }).click();
    const dialog = page.getByRole('dialog');
    await expect(dialog).toBeVisible();
    return dialog;
}

/** The names of all entries in the list, in display order. */
export async function listedNames(/** @type {Page} */ page) {
    const texts = await entryList(page).getByRole('listitem').allInnerTexts();
    return texts.map(t => t.trim());
}

/**
 * Asserts that the entry is marked as the current one. Upstream sets `aria-current="page"` on the
 * `<li>`; a redesign may set it on the button / link itself, both are accepted.
 */
export async function expectCurrent(/** @type {Page} */ page, /** @type {string} */ name) {
    const btn = entry(page, name);
    const item = entryList(page)
        .getByRole('listitem')
        .filter({ has: page.getByRole('button', { name, exact: true }) });
    await expect
        .poll(async () => {
            const a = await item.getAttribute('aria-current');
            const b = await btn.getAttribute('aria-current');
            return [a, b].some(v => v && v !== 'false');
        })
        .toBe(true);
}

/**
 * A tab of the detail view. Prefers `role=tab`.
 * FINDING: upstream (`lib/tabs/Tabs.svelte`) renders tabs as plain buttons without a tablist,
 * and the search field's icon-only clear button is also called "Delete". The fallback therefore
 * takes the first button of `main` with that name that also shows it as visible text.
 */
export function tab(/** @type {Page} */ page, /** @type {string} */ name) {
    const main = page.getByRole('main');
    return main
        .getByRole('tab', { name, exact: true })
        .or(main.getByRole('button', { name, exact: true }).filter({ hasText: name }))
        .first();
}

/** Asserts that a tab is the selected one (`aria-selected` or upstream `aria-current="step"`). */
export async function expectTabSelected(/** @type {Page} */ page, /** @type {string} */ name) {
    const t = tab(page, name);
    await expect
        .poll(async () => {
            const sel = await t.getAttribute('aria-selected');
            const cur = await t.getAttribute('aria-current');
            return sel === 'true' || (!!cur && cur !== 'false');
        })
        .toBe(true);
}

/** The action next to the entity name. */
export function entityDelete(/** @type {Page} */ page) {
    return page
        .locator('.admin-entity-header')
        .getByRole('button', { name: 'Delete', exact: true });
}

/** Opens the entity's delete confirmation and returns the dialog. */
export async function openEntityDelete(/** @type {Page} */ page) {
    await entityDelete(page).click();
    const dialog = page.getByRole('dialog', { name: 'Delete', exact: true });
    await expect(dialog).toBeVisible();
    return dialog;
}

/** Confirms deletion in an open dialog. */
export function deleteAction(/** @type {Page} */ page) {
    return page
        .getByRole('dialog', { name: 'Delete', exact: true })
        .getByRole('button', { name: 'Delete', exact: true });
}

/** The "Save" button of the detail view (outside of dialogs). */
export function saveButton(/** @type {Page} */ page) {
    return page.getByRole('main').getByRole('button', { name: 'Save', exact: true });
}

/** The search box of the secondary navigation and its clear button. */
export function searchBox(/** @type {Page} */ page) {
    return subNav(page).getByRole('searchbox', { name: 'Search' });
}

/**
 * FINDING: the clear button of the search field (`lib/search_bar/SearchBar.svelte`) is called
 * "Delete", which collides with the Delete tab. A name like "Clear search" is accepted as well.
 */
export function searchClear(/** @type {Page} */ page) {
    return subNav(page)
        .getByRole('search')
        .getByRole('button', { name: /^(delete|clear|clear search|reset)$/i });
}

/** Collects every request whose method + path match, to assert that a save was blocked. */
export function recordRequests(
    /** @type {Page} */ page,
    /** @type {string} */ method,
    /** @type {RegExp} */ path,
) {
    /** @type {string[]} */
    const seen = [];
    page.on('request', req => {
        if (req.method() === method && path.test(new URL(req.url()).pathname)) {
            seen.push(req.url());
        }
    });
    return seen;
}

/** Sorts like the UI does (`String.localeCompare` in the browser). */
export async function sortedInBrowser(
    /** @type {Page} */ page,
    /** @type {string[]} */ values,
    /** @type {boolean} */ desc = false,
) {
    return page.evaluate(
        ([v, d]) =>
            [.../** @type {string[]} */ (v)].sort((a, b) =>
                d ? b.localeCompare(a) : a.localeCompare(b),
            ),
        /** @type {[string[], boolean]} */ ([values, desc]),
    );
}

/** Creates a user via the REST API with the given roles / groups and returns it. */
export async function createUser(
    /** @type {Page} */ page,
    /** @type {{ email: string, roles?: string[], groups?: string[] }} */ opts,
) {
    return api(page, 'POST', 'users', {
        email: opts.email,
        given_name: 'E2E',
        family_name: 'Entity',
        language: 'en',
        roles: opts.roles || [],
        groups: opts.groups || [],
    });
}

/** Deletes something via the API and ignores a 404 (already gone). */
export async function deleteQuietly(/** @type {Page} */ page, /** @type {string} */ path) {
    try {
        await api(page, 'DELETE', path);
    } catch (e) {
        if (!/-> 404/.test(String(e))) throw e;
    }
}

/** Asserts that a textarea / input holds JSON equal to `expected` (independent of key order). */
export async function expectJsonValue(
    /** @type {import('@playwright/test').Locator} */ input,
    /** @type {any} */ expected,
) {
    await expect
        .poll(async () => {
            try {
                return JSON.parse(await input.inputValue());
            } catch {
                return await input.inputValue();
            }
        })
        .toEqual(expected);
}
