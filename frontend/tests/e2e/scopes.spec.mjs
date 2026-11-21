import { openEntityDelete, entityDelete } from './helpers/entities.mjs';
// Characterization tests for the admin Scopes section: list, search, order, Add New (incl. scope
// names with dots, upstream issue #1311), attribute mappings into access / id tokens, immutable
// default scopes and Delete.
import { test, expect } from './fixtures.mjs';
import { api } from './helpers/api.mjs';
import { unique } from './helpers/env.mjs';
import { chooseOption } from './helpers/ui.mjs';
import {
    clickDeleteAndConfirm,
    countRequests,
    entries,
    entry,
    expectInvalid,
    openAddNew,
    openSubNav,
    saveAndWait,
    saveButton,
    subNav,
    expectEntryCurrent,
} from './helpers/clients.mjs';

const SCOPES = '/auth/v1/admin/scopes';
const DEFAULT_SCOPES = ['address', 'email', 'groups', 'openid', 'phone', 'profile'];

/** A unique scope name; dots are allowed since upstream issue #1311. */
const scopeName = (/** @type {string} */ prefix) => unique(prefix).replaceAll('-', '.');

/** @param {import('@playwright/test').Page} page */
async function allScopes(page) {
    return /** @type {{ id: string, name: string, attr_include_access?: string[], attr_include_id?: string[], claims_at_root: boolean }[]} */ (
        await api(page, 'GET', 'scopes')
    );
}

/**
 * @param {import('@playwright/test').Page} page
 * @param {string} name
 */
async function scopeByName(page, name) {
    return (await allScopes(page)).find(s => s.name === name);
}

/**
 * @param {import('@playwright/test').Page} page
 * @param {string} name
 */
async function deleteScopeQuietly(page, name) {
    const s = await scopeByName(page, name);
    if (s) await api(page, 'DELETE', `scopes/${s.id}`);
}

/**
 * @param {import('@playwright/test').Page} page
 * @param {string} id
 */
async function openScope(page, id) {
    await page.goto(`${SCOPES}?sid=${encodeURIComponent(id)}`);
    await expect(page.getByRole('textbox', { name: 'Scope Name' })).toBeVisible();
}

test.describe('scopes list', () => {
    test('loads all scopes and marks the default ones', async ({ page }) => {
        await page.goto(SCOPES);
        const all = await allScopes(page);
        await expect(entries(page)).toHaveCount(all.length);
        for (const name of DEFAULT_SCOPES) {
            await expect(entry(page, name)).toContainText('default');
        }
        // a custom bootstrap scope has no "default" marker
        const custom = all.find(s => !DEFAULT_SCOPES.includes(s.name));
        if (custom) await expect(entry(page, custom.name)).not.toContainText('default');
    });

    test('search by name and by ID', async ({ page }) => {
        await page.goto(SCOPES);
        const all = await allScopes(page);
        const search = subNav(page).getByRole('searchbox', { name: 'Search' });
        const options = subNav(page).getByRole('combobox', { name: 'Search Options' });
        await expect(options).toContainText('Name');

        await search.fill('OPEN');
        await expect(entries(page)).toHaveCount(1);
        await expect(entry(page, 'openid')).toBeVisible();

        // searching for the ID while searching by name finds nothing
        const email = /** @type {any} */ (all.find(s => s.name === 'email'));
        await search.fill(email.id);
        await expect(entries(page)).toHaveCount(0);

        await chooseOption(page, options, 'ID');
        await expect(options).toContainText('ID');
        await expect(entries(page)).toHaveCount(1);
        await expect(entry(page, 'email')).toBeVisible();

        await subNav(page)
            .getByRole('search')
            .getByRole('button', { name: /delete|clear/i })
            .click();
        await expect(entries(page)).toHaveCount(all.length);
    });

    test('orders by name and by ID, ascending and descending', async ({ page }) => {
        await page.goto(SCOPES);
        const all = await allScopes(page);
        await expect(entries(page)).toHaveCount(all.length);
        const names = () => entries(page).locator('.scope-name').allTextContents();
        const byName = all.map(s => s.name).sort((a, b) => a.localeCompare(b));
        await expect.poll(names).toEqual(byName);

        await subNav(page).getByRole('button', { name: 'Change sort to descending' }).click();
        await expect.poll(names).toEqual([...byName].reverse());

        // order by ID (still descending)
        const order = subNav(page).getByRole('combobox', { name: 'Order by ...' });
        await chooseOption(page, order, 'ID');
        const byId = [...all].sort((a, b) => a.id.localeCompare(b.id)).map(s => s.name);
        await expect.poll(names).toEqual([...byId].reverse());

        await subNav(page).getByRole('button', { name: 'Change sort to ascending' }).click();
        await expect.poll(names).toEqual(byId);
    });

    test('selecting a scope sets the deep link', async ({ page }) => {
        await page.goto(SCOPES);
        const email = /** @type {any} */ (await scopeByName(page, 'email'));
        await entry(page, 'email').click();
        await expect(page).toHaveURL(new RegExp(`[?&]sid=${email.id}(&|$)`));
        await expectEntryCurrent(entry(page, 'email'));
        await expect(page.getByRole('textbox', { name: 'Scope Name' })).toHaveValue('email');
        await expect(page.getByRole('main').getByText(email.id, { exact: true })).toBeVisible();

        await page.reload();
        await expect(page.getByRole('textbox', { name: 'Scope Name' })).toHaveValue('email');
    });

    test('default scopes are immutable and cannot be deleted', async ({ page }) => {
        await page.goto(SCOPES);
        const openid = /** @type {any} */ (await scopeByName(page, 'openid'));
        await openScope(page, openid.id);
        await expect(page.getByRole('textbox', { name: 'Scope Name' })).toBeDisabled();
        await expect(
            page.getByText('This is a default OIDC Scope. These are immutable.'),
        ).toBeVisible();
        await expect(
            page.getByRole('main').getByRole('button', { name: 'Save', exact: true }),
        ).toHaveCount(0);

        await expect(entityDelete(page)).toBeDisabled();
    });
});

test.describe('add new scope', () => {
    /** @type {string[]} */
    let created = [];
    test.afterEach(async ({ page }) => {
        for (const name of created) await deleteScopeQuietly(page, name);
        created = [];
    });

    test('validates the name and can be cancelled', async ({ page }) => {
        await page.goto(SCOPES);
        const before = (await allScopes(page)).length;
        const dialog = await openAddNew(page);
        const name = dialog.getByRole('textbox', { name: 'Scope Name' });
        const save = dialog.getByRole('button', { name: 'Save' });
        await expect(name).toBeFocused();

        expect(await countRequests(page, 'POST', '/auth/v1/scopes', () => save.click())).toBe(0);
        await expectInvalid(name);

        for (const bad of ['a', 'with space', 'bad$char', 'x'.repeat(65)]) {
            await name.fill(bad);
            expect(
                await countRequests(page, 'POST', '/auth/v1/scopes', () => save.click()),
                bad,
            ).toBe(0);
            await expectInvalid(name);
        }

        await name.fill('openid');
        expect(await countRequests(page, 'POST', '/auth/v1/scopes', () => save.click())).toBe(0);
        await expect(dialog.getByText('Name exists already')).toBeVisible();

        await dialog.getByRole('button', { name: 'Close' }).first().click();
        await expect(dialog).toBeHidden();
        expect((await allScopes(page)).length).toBe(before);
    });

    test('creates a scope with dots in its name (#1311)', async ({ page }) => {
        const name = scopeName('e2e.scope');
        created.push(name);
        await page.goto(SCOPES);
        const dialog = await openAddNew(page);
        await dialog.getByRole('textbox', { name: 'Scope Name' }).fill(name);
        await saveAndWait(page, dialog.getByRole('button', { name: 'Save' }), 'POST', '/scopes');
        await expect(dialog).toBeHidden();

        const s = /** @type {any} */ (await scopeByName(page, name));
        expect(s).toBeTruthy();
        expect(s.attr_include_access ?? null).toBeNull();
        await expect(page).toHaveURL(new RegExp(`[?&]sid=${s.id}(&|$)`));
        await expectEntryCurrent(entry(page, name));
        await expect(entry(page, name)).not.toContainText('default');
        await expect(page.getByRole('textbox', { name: 'Scope Name' })).toHaveValue(name);
        await expect(page.getByRole('textbox', { name: 'Scope Name' })).toBeEnabled();

        // the new scope can be allowed for a client right away
        await page.goto('/auth/v1/admin/clients?cid=init_client');
        await page.getByRole('button', { name: 'Tokens', exact: true }).click();
        await page
            .getByRole('main')
            .getByRole('button', { name: 'Edit', exact: true })
            .first()
            .click();
        await expect(
            page.getByRole('dialog').getByRole('checkbox', { name, exact: true }),
        ).toBeVisible();
    });
});

test.describe('scope config', () => {
    /** @type {string} */
    let name;
    /** @type {string} */
    let sid;
    /** @type {string[]} */
    let attrs;
    test.beforeEach(async ({ page }) => {
        name = scopeName('e2e.cfg');
        attrs = [unique('e2e_sa').replaceAll('-', '_'), unique('e2e_sb').replaceAll('-', '_')];
        await page.goto(SCOPES);
        for (const a of attrs) await api(page, 'POST', 'users/attr', { name: a });
        sid = (await api(page, 'POST', 'scopes', { scope: name })).id;
        await openScope(page, sid);
    });
    test.afterEach(async ({ page }) => {
        try {
            await api(page, 'DELETE', `scopes/${sid}`);
        } catch {}
        for (const a of attrs) {
            try {
                await api(page, 'DELETE', `users/attr/${a}`);
            } catch {}
        }
    });

    test('maps attributes into access and id tokens', async ({ page }) => {
        const main = page.getByRole('main');
        await expect(main.getByText('You can map custom scopes to attributes.')).toBeVisible();
        await expect(main.getByText('Access Token Mappings')).toBeVisible();
        await expect(main.getByText('Id Token Mappings')).toBeVisible();
        // The access token editor comes before the ID token editor.
        const edit = main.getByRole('button', { name: 'Edit', exact: true });
        await expect(edit).toHaveCount(2);
        const accessChips = main.locator('.admin-selection-field').first().getByRole('listitem');

        await edit.nth(0).click();
        let dialog = page.getByRole('dialog');
        await expect(dialog.getByRole('heading', { name: 'Access Token Mappings' })).toBeVisible();
        await dialog.getByRole('checkbox', { name: attrs[0], exact: true }).check();
        await dialog.getByRole('checkbox', { name: attrs[1], exact: true }).check();
        await dialog.getByRole('button', { name: 'Close' }).last().click();
        await expect(dialog).toBeHidden();
        await expect(accessChips).toHaveText(attrs);

        await edit.nth(1).click();
        dialog = page.getByRole('dialog');
        await expect(dialog.getByRole('heading', { name: 'Id Token Mappings' })).toBeVisible();
        await dialog.getByRole('checkbox', { name: attrs[1], exact: true }).check();
        await page.keyboard.press('Escape');
        await expect(dialog).toBeHidden();

        // claims at root shows a warning
        const rootWarn = main.getByText(
            /This scope's mapped attributes are written at the token root/,
        );
        await expect(rootWarn).toHaveCount(0);
        await main.getByRole('checkbox', { name: 'Emit claims at token root' }).check();
        await expect(rootWarn).toBeVisible();

        await saveAndWait(page, saveButton(page), 'PUT', `/scopes/${sid}`);
        let s = /** @type {any} */ (await scopeByName(page, name));
        expect([...s.attr_include_access].sort()).toEqual([...attrs].sort());
        expect(s.attr_include_id).toEqual([attrs[1]]);
        expect(s.claims_at_root).toBe(true);

        await page.reload();
        await expect(accessChips).toHaveText(attrs);
        await expect(
            main.getByRole('checkbox', { name: 'Emit claims at token root' }),
        ).toBeChecked();

        // remove all mappings again
        await main.getByRole('button', { name: 'Edit', exact: true }).nth(0).click();
        dialog = page.getByRole('dialog');
        await dialog.getByRole('checkbox', { name: attrs[0], exact: true }).uncheck();
        await dialog.getByRole('checkbox', { name: attrs[1], exact: true }).uncheck();
        await dialog.getByRole('button', { name: 'Close' }).last().click();
        await main.getByRole('button', { name: 'Edit', exact: true }).nth(1).click();
        dialog = page.getByRole('dialog');
        await dialog.getByRole('checkbox', { name: attrs[1], exact: true }).uncheck();
        await dialog.getByRole('button', { name: 'Close' }).last().click();
        await main.getByRole('checkbox', { name: 'Emit claims at token root' }).uncheck();
        await saveAndWait(page, saveButton(page), 'PUT', `/scopes/${sid}`);
        s = await scopeByName(page, name);
        expect(s.attr_include_access ?? null).toBeNull();
        expect(s.attr_include_id ?? null).toBeNull();
        expect(s.claims_at_root).toBe(false);
    });

    test('renames the scope and validates the new name', async ({ page }) => {
        const input = page.getByRole('textbox', { name: 'Scope Name' });
        await expect(input).toHaveValue(name);

        await input.fill('a');
        await expectInvalid(input);
        expect(
            await countRequests(page, 'PUT', `/scopes/${sid}`, () => saveButton(page).click()),
        ).toBe(0);

        // a default scope name or an existing name is rejected before any request
        await input.fill('profile');
        expect(
            await countRequests(page, 'PUT', `/scopes/${sid}`, () => saveButton(page).click()),
        ).toBe(0);
        await expect(page.getByText('Name exists already')).toBeVisible();

        const renamed = `${name}.renamed`;
        await input.fill(renamed);
        await saveAndWait(page, saveButton(page), 'PUT', `/scopes/${sid}`);
        await expect(entry(page, renamed)).toBeVisible();
        await expect(entry(page, name)).toHaveCount(0);
        expect(
            (await api(page, 'GET', 'scopes')).find((/** @type {any} */ s) => s.id === sid).name,
        ).toBe(renamed);

        await page.reload();
        await expect(input).toHaveValue(renamed);
    });

    test('cancelling keeps the scope, confirming deletes it', async ({ page }) => {
        await openEntityDelete(page);
        await expect(page.getByText('Are you sure you want to delete this scope?')).toBeVisible();
        await page
            .getByRole('dialog', { name: 'Delete' })
            .getByRole('button', { name: 'Cancel' })
            .click();
        await expect(page.getByRole('textbox', { name: 'Scope Name' })).toBeVisible();
        expect(await scopeByName(page, name)).toBeTruthy();

        expect(await clickDeleteAndConfirm(page, `/scopes/${sid}`)).toBeLessThan(400);
        await expect(entry(page, name)).toHaveCount(0);
        await expect(page.getByRole('textbox', { name: 'Scope Name' })).toHaveCount(0);
        expect(await scopeByName(page, name)).toBeUndefined();
    });
});

test.describe('scopes at phone size', () => {
    test('create, map and delete a scope @mobile', async ({ page }) => {
        const name = scopeName('e2e.mob');
        await page.goto(SCOPES);
        try {
            const dialog = await openAddNew(page);
            await dialog.getByRole('textbox', { name: 'Scope Name' }).fill(name);
            await saveAndWait(
                page,
                dialog.getByRole('button', { name: 'Save' }),
                'POST',
                '/scopes',
            );
            const s = /** @type {any} */ (await scopeByName(page, name));
            await expect(page).toHaveURL(new RegExp(`[?&]sid=${s.id}(&|$)`));
            await expect(page.getByRole('textbox', { name: 'Scope Name' })).toHaveValue(name);

            await page
                .getByRole('main')
                .getByRole('checkbox', { name: 'Emit claims at token root' })
                .check();
            await saveAndWait(page, saveButton(page), 'PUT', `/scopes/${s.id}`);
            expect(/** @type {any} */ (await scopeByName(page, name)).claims_at_root).toBe(true);

            expect(await clickDeleteAndConfirm(page, `/scopes/${s.id}`)).toBeLessThan(400);
            expect(await scopeByName(page, name)).toBeUndefined();
        } finally {
            await deleteScopeQuietly(page, name);
        }
    });

    test('list and deep link @mobile', async ({ page }) => {
        await page.goto(SCOPES);
        await openSubNav(page);
        await entry(page, 'profile').click();
        await expect(page).toHaveURL(/[?&]sid=/);
        await expect(page.getByRole('textbox', { name: 'Scope Name' })).toHaveValue('profile');
    });
});
