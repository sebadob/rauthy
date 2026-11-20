import { openEntityDelete, entityDelete } from './helpers/entities.mjs';
import { test, expect } from './fixtures.mjs';
import { api } from './helpers/api.mjs';
import { unique } from './helpers/env.mjs';
import { chooseOption } from './helpers/ui.mjs';
import {
    createUser,
    deleteAction,
    deleteQuietly,
    expectJsonValue,
    entry,
    entryList,
    expectCurrent,
    listedNames,
    openAddNew,
    openEntry,
    recordRequests,
    saveButton,
    searchBox,
    searchClear,
    showSubNav,
    sortedInBrowser,
    subNav,
} from './helpers/entities.mjs';

const PAGE = '/auth/v1/admin/roles';

/** @typedef {{ id: string, name: string, meta?: any }} Role */

/** @param {import('@playwright/test').Page} page */
async function roles(page) {
    return /** @type {Role[]} */ (await api(page, 'GET', 'roles'));
}

/** @param {import('@playwright/test').Page} page @param {string} id */
async function roleById(page, id) {
    return (await roles(page)).find(r => r.id === id);
}

/** @param {import('@playwright/test').Page} page */
const nameInput = page => page.getByRole('main').getByRole('textbox', { name: 'Role Name' });
/** @param {import('@playwright/test').Page} page */
const metaInput = page =>
    page.getByRole('main').getByRole('textbox', { name: 'Metadata as JSON value' });

test.describe('roles list', () => {
    test('loads every role of the backend', async ({ page }) => {
        await page.goto(PAGE);
        const all = await roles(page);
        await expect(entry(page, 'rauthy_admin')).toBeVisible();
        await expect(entryList(page).getByRole('listitem')).toHaveCount(all.length);
        expect((await listedNames(page)).sort()).toEqual(all.map(r => r.name).sort());
    });

    test('search by name and by ID, then clear the search', async ({ page }) => {
        await page.goto(PAGE);
        const all = await roles(page);
        await expect(entry(page, 'rauthy_admin')).toBeVisible();

        // by name, case-insensitive substring
        await searchBox(page).fill('ADMIN');
        const byName = all.filter(r => r.name.toLowerCase().includes('admin')).map(r => r.name);
        await expect(entryList(page).getByRole('listitem')).toHaveCount(byName.length);
        expect((await listedNames(page)).sort()).toEqual(byName.sort());

        await searchBox(page).fill('no-such-role-xyz');
        await expect(entryList(page).getByRole('listitem')).toHaveCount(0);

        // by ID
        const target = /** @type {Role} */ (all.find(r => r.name === 'user'));
        await chooseOption(
            page,
            subNav(page).getByRole('combobox', { name: 'Search Options' }),
            'ID',
        );
        await searchBox(page).fill(target.id.slice(2, 14));
        await expect(entryList(page).getByRole('listitem')).toHaveCount(1);
        await expect(entry(page, 'user')).toBeVisible();

        // a name does not match while searching by ID
        await searchBox(page).fill('rauthy_admin');
        await expect(entryList(page).getByRole('listitem')).toHaveCount(0);

        await searchClear(page).click();
        await expect(searchBox(page)).toHaveValue('');
        await expect(entryList(page).getByRole('listitem')).toHaveCount(all.length);
    });

    test('sort by name ascending / descending and by ID', async ({ page }) => {
        await page.goto(PAGE);
        const all = await roles(page);
        await expect(entry(page, 'rauthy_admin')).toBeVisible();
        const names = all.map(r => r.name);

        await expect.poll(() => listedNames(page)).toEqual(await sortedInBrowser(page, names));

        await subNav(page).getByRole('button', { name: 'Change sort to descending' }).click();
        await expect
            .poll(() => listedNames(page))
            .toEqual(await sortedInBrowser(page, names, true));

        await subNav(page).getByRole('button', { name: 'Change sort to ascending' }).click();
        await expect.poll(() => listedNames(page)).toEqual(await sortedInBrowser(page, names));

        await chooseOption(
            page,
            subNav(page).getByRole('combobox', { name: 'Order by ...' }),
            'ID',
        );
        const ids = await sortedInBrowser(
            page,
            all.map(r => r.id),
        );
        const byId = ids.map(id => /** @type {Role} */ (all.find(r => r.id === id)).name);
        await expect.poll(() => listedNames(page)).toEqual(byId);
    });

    test('selecting a role shows its details and a deep link', async ({ page }) => {
        await page.goto(PAGE);
        const user = /** @type {Role} */ ((await roles(page)).find(r => r.name === 'user'));

        await openEntry(page, 'user');
        await expect(page).toHaveURL(new RegExp(`[?&]rid=${user.id}`));
        await expectCurrent(page, 'user');
        await expect(nameInput(page)).toHaveValue('user');
        await expect(page.getByRole('main').getByText(user.id)).toBeVisible();

        // deep link opens the same role directly
        await page.goto(`${PAGE}?rid=${user.id}`);
        await expect(nameInput(page)).toHaveValue('user');
        await expectCurrent(page, 'user');

        // switching to another role replaces the details
        await openEntry(page, 'admin');
        await expect(nameInput(page)).toHaveValue('admin');
    });

    test('rauthy_admin is immutable', async ({ page }) => {
        await page.goto(PAGE);
        await openEntry(page, 'rauthy_admin');
        await expect(nameInput(page)).toBeDisabled();
        await expect(
            page
                .getByRole('main')
                .getByRole('paragraph')
                .filter({ hasText: /rauthy_admin\s*role is immutable/ }),
        ).toBeVisible();
        await expect(saveButton(page)).toHaveCount(0);
        await expect(metaInput(page)).toHaveCount(0);

        await expect(entityDelete(page)).toBeDisabled();
    });
});

test.describe('create role', () => {
    /** @type {string[]} */
    let cleanup = [];
    test.afterEach(async ({ page }) => {
        const all = await roles(page);
        for (const name of cleanup) {
            const r = all.find(x => x.name === name);
            if (r) await deleteQuietly(page, `roles/${r.id}`);
        }
        cleanup = [];
    });

    test('dialog validates the name and can be closed without saving', async ({ page }) => {
        await page.goto(PAGE);
        await expect(entry(page, 'rauthy_admin')).toBeVisible();
        const posts = recordRequests(page, 'POST', /\/auth\/v1\/roles$/);

        let dialog = await openAddNew(page);
        const name = dialog.getByRole('textbox', { name: 'Role Name' });
        await expect(name).toBeFocused();
        await expect(dialog.getByRole('textbox', { name: 'Metadata as JSON value' })).toBeEmpty();

        // empty name is required
        await dialog.getByRole('button', { name: 'Save' }).click();
        await expect(name).toHaveAttribute('aria-invalid', 'true');

        // spaces and special chars are not allowed for roles
        await name.fill('bad role!');
        await expect(dialog.getByText('Invalid Input')).toBeVisible();
        await expect(name).toHaveAttribute('aria-invalid', 'true');
        await dialog.getByRole('button', { name: 'Save' }).click();

        // too short
        await name.fill('a');
        await expect(dialog.getByText('Invalid Input')).toBeVisible();
        await dialog.getByRole('button', { name: 'Save' }).click();

        // existing name
        await name.fill('admin');
        await expect(dialog.getByText('Invalid Input')).toBeHidden();
        await dialog.getByRole('button', { name: 'Save' }).click();
        await expect(dialog.getByText('Name exists already')).toBeVisible();
        expect(posts).toEqual([]);

        // close button
        await dialog.getByRole('button', { name: 'Close' }).click();
        await expect(dialog).toBeHidden();

        // Escape closes as well, and a fresh dialog starts empty
        dialog = await openAddNew(page);
        await page.keyboard.press('Escape');
        await expect(dialog).toBeHidden();
        expect(posts).toEqual([]);
    });

    test('creates a role with JSON metadata and selects it', async ({ page }) => {
        const name = unique('e2e-role');
        cleanup.push(name);
        await page.goto(PAGE);
        await expect(entry(page, 'rauthy_admin')).toBeVisible();

        const dialog = await openAddNew(page);
        await dialog.getByRole('textbox', { name: 'Role Name' }).fill(name);
        await dialog
            .getByRole('textbox', { name: 'Metadata as JSON value' })
            .fill('{"team": "e2e", "level": 3, "tags": ["a", "b"]}');
        await dialog.getByRole('button', { name: 'Save' }).click();
        await expect(dialog).toBeHidden();

        const created = /** @type {Role} */ ((await roles(page)).find(r => r.name === name));
        expect(created.meta).toEqual({ team: 'e2e', level: 3, tags: ['a', 'b'] });

        await expect(page).toHaveURL(new RegExp(`[?&]rid=${created.id}`));
        await showSubNav(page);
        await expect(entry(page, name)).toBeVisible();
        await expectCurrent(page, name);
        await expect(nameInput(page)).toHaveValue(name);
        await expectJsonValue(metaInput(page), { team: 'e2e', level: 3, tags: ['a', 'b'] });

        // survives a reload
        await page.reload();
        await expect(nameInput(page)).toHaveValue(name);
    });

    test('creates a role without metadata', async ({ page }) => {
        const name = unique('e2e-role');
        cleanup.push(name);
        await page.goto(PAGE);
        await expect(entry(page, 'rauthy_admin')).toBeVisible();
        const dialog = await openAddNew(page);
        await dialog.getByRole('textbox', { name: 'Role Name' }).fill(name);
        await dialog.getByRole('textbox', { name: 'Role Name' }).press('Enter');
        await expect(dialog).toBeHidden();
        await expect(nameInput(page)).toHaveValue(name);
        await expect(metaInput(page)).toHaveValue('');
        const created = (await roles(page)).find(r => r.name === name);
        expect(created).toBeTruthy();
        expect(created?.meta).toBeUndefined();
    });
});

test.describe('edit role', () => {
    /** @type {Role} */
    let role;
    test.beforeEach(async ({ page }) => {
        await page.goto(PAGE);
        role = await api(page, 'POST', 'roles', { role: unique('e2e-role'), meta: { v: 1 } });
    });
    test.afterEach(async ({ page }) => {
        await deleteQuietly(page, `roles/${role.id}`);
    });

    test('edit metadata, save, reload: persisted', async ({ page }) => {
        await page.goto(`${PAGE}?rid=${role.id}`);
        await expectJsonValue(metaInput(page), { v: 1 });

        await metaInput(page).fill('{"v": 2, "note": "edited"}');
        await saveButton(page).click();
        await expect
            .poll(async () => (await roleById(page, role.id))?.meta)
            .toEqual({
                v: 2,
                note: 'edited',
            });

        await page.reload();
        await expectJsonValue(metaInput(page), { v: 2, note: 'edited' });

        // plain values are stored as JSON scalars, plain text as a JSON string
        await metaInput(page).fill('[1, "two"]');
        await saveButton(page).click();
        await expect.poll(async () => (await roleById(page, role.id))?.meta).toEqual([1, 'two']);

        await metaInput(page).fill('just some text');
        await saveButton(page).click();
        await expect.poll(async () => (await roleById(page, role.id))?.meta).toBe('just some text');
        await page.reload();
        await expect(metaInput(page)).toHaveValue('just some text');

        // clearing removes the metadata
        await metaInput(page).fill('');
        await saveButton(page).click();
        await expect.poll(async () => (await roleById(page, role.id))?.meta).toBeUndefined();
        await page.reload();
        await expect(metaInput(page)).toHaveValue('');
    });

    test('rename keeps the role on users holding it (#1861)', async ({ page }) => {
        const user = await createUser(page, {
            email: `${unique('e2e-role-user')}@example.com`,
            roles: [role.name],
        });
        try {
            const newName = unique('e2e-renamed');
            await page.goto(`${PAGE}?rid=${role.id}`);
            await expect(nameInput(page)).toHaveValue(role.name);
            await nameInput(page).fill(newName);
            await saveButton(page).click();

            await expect.poll(async () => (await roleById(page, role.id))?.name).toBe(newName);
            await showSubNav(page);
            await expect(entry(page, newName)).toBeVisible();
            await expect(entry(page, role.name)).toHaveCount(0);
            // the id stays the same, the deep link still works
            await page.reload();
            await expect(nameInput(page)).toHaveValue(newName);

            const u = await api(page, 'GET', `users/${user.id}`);
            expect(u.roles).toContain(newName);
            expect(u.roles).not.toContain(role.name);
        } finally {
            await deleteQuietly(page, `users/${user.id}`);
        }
    });

    test('invalid or duplicate name shows an error and blocks the save', async ({ page }) => {
        await page.goto(`${PAGE}?rid=${role.id}`);
        await expect(nameInput(page)).toHaveValue(role.name);
        const puts = recordRequests(page, 'PUT', /\/auth\/v1\/roles\//);

        await nameInput(page).fill('has space');
        await expect(page.getByRole('main').getByText('Invalid Input')).toBeVisible();
        await expect(nameInput(page)).toHaveAttribute('aria-invalid', 'true');
        await saveButton(page).click();

        await nameInput(page).fill('');
        await saveButton(page).click();
        await expect(nameInput(page)).toHaveAttribute('aria-invalid', 'true');

        await nameInput(page).fill('admin');
        await saveButton(page).click();
        await expect(page.getByRole('main').getByText('Name exists already')).toBeVisible();

        expect(puts).toEqual([]);
        expect((await roleById(page, role.id))?.name).toBe(role.name);
    });

    // UPSTREAM BUG: `stringifyJsonValue()` in `src/utils/jsonValue.ts` returns '' for every
    // falsy value, so metadata `false` / `0` is shown as empty and wiped by the next save.
    test.fail('falsy JSON metadata is shown after a reload', async ({ page }) => {
        await page.goto(`${PAGE}?rid=${role.id}`);
        await metaInput(page).fill('false');
        await saveButton(page).click();
        await expect.poll(async () => (await roleById(page, role.id))?.meta).toBe(false);
        await page.reload();
        await expect(metaInput(page)).toHaveValue('false', { timeout: 3_000 });
    });

    // UPSTREAM BUG: `parseJsonValue()` in `src/utils/jsonValue.ts` uses `Number.parseInt` first,
    // so any text that starts with a digit ("12 monkeys", "2024-01-01") is silently stored as a
    // number and the rest of the input is lost.
    test.fail('text metadata starting with a digit is stored unchanged', async ({ page }) => {
        await page.goto(`${PAGE}?rid=${role.id}`);
        await metaInput(page).fill('12 monkeys');
        await saveButton(page).click();
        await expect
            .poll(async () => (await roleById(page, role.id))?.meta, { timeout: 3_000 })
            .toBe('12 monkeys');
    });
});

test.describe('delete role', () => {
    test('header action asks first, cancelling keeps the entry, confirming deletes it', async ({
        page,
    }) => {
        await page.goto(PAGE);
        const role = await api(page, 'POST', 'roles', { role: unique('e2e-role') });
        const user = await createUser(page, {
            email: `${unique('e2e-role-del')}@example.com`,
            roles: [role.name],
        });
        try {
            await page.goto(`${PAGE}?rid=${role.id}`);
            await expect(nameInput(page)).toHaveValue(role.name);

            await openEntityDelete(page);
            await expect(
                page.getByRole('main').getByText('Are you sure you want to delete this role?'),
            ).toBeVisible();

            // Cancel closes the dialog and keeps the entity.
            await page
                .getByRole('dialog', { name: 'Delete' })
                .getByRole('button', { name: 'Cancel' })
                .click();
            await expect(nameInput(page)).toHaveValue(role.name);
            expect(await roleById(page, role.id)).toBeTruthy();

            await openEntityDelete(page);
            await deleteAction(page).click();
            await expect.poll(async () => await roleById(page, role.id)).toBeUndefined();
            await expect(entry(page, role.name)).toHaveCount(0);
            await expect(nameInput(page)).toHaveCount(0);

            // the role is removed from users as well
            const u = await api(page, 'GET', `users/${user.id}`);
            expect(u.roles).not.toContain(role.name);
        } finally {
            await deleteQuietly(page, `users/${user.id}`);
            await deleteQuietly(page, `roles/${role.id}`);
        }
    });
});

test.describe('roles on a phone', () => {
    test('create, edit and delete a role @mobile', async ({ page }) => {
        const name = unique('e2e-role-m');
        await page.goto(PAGE);
        try {
            const dialog = await openAddNew(page);
            await dialog.getByRole('textbox', { name: 'Role Name' }).fill(name);
            await dialog.getByRole('button', { name: 'Save' }).click();
            await expect(dialog).toBeHidden();
            await expect(nameInput(page)).toHaveValue(name);

            await metaInput(page).fill('{"mobile": true}');
            await saveButton(page).click();
            await expect
                .poll(async () => (await roles(page)).find(r => r.name === name)?.meta)
                .toEqual({ mobile: true });

            await page.goto(PAGE);
            await openEntry(page, name);
            await expectJsonValue(metaInput(page), { mobile: true });
            await openEntityDelete(page);
            await deleteAction(page).click();
            await expect
                .poll(async () => (await roles(page)).some(r => r.name === name))
                .toBe(false);
        } finally {
            const r = (await roles(page)).find(x => x.name === name);
            if (r) await deleteQuietly(page, `roles/${r.id}`);
        }
    });
});
