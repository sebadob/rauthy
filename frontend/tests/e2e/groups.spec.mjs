import { openEntityDelete } from './helpers/entities.mjs';
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

const PAGE = '/auth/v1/admin/groups';

/** @typedef {{ id: string, name: string, meta?: any }} Group */

/** @param {import('@playwright/test').Page} page */
async function groups(page) {
    return /** @type {Group[]} */ (await api(page, 'GET', 'groups'));
}

/** @param {import('@playwright/test').Page} page @param {string} id */
async function groupById(page, id) {
    return (await groups(page)).find(g => g.id === id);
}

/** @param {import('@playwright/test').Page} page */
const nameInput = page => page.getByRole('main').getByRole('textbox', { name: 'Group Name' });
/** @param {import('@playwright/test').Page} page */
const metaInput = page =>
    page.getByRole('main').getByRole('textbox', { name: 'Metadata as JSON value' });

test.describe('groups list', () => {
    test('loads every group of the backend', async ({ page }) => {
        await page.goto(PAGE);
        const all = await groups(page);
        await expect(entryList(page).getByRole('listitem')).toHaveCount(all.length);
        expect((await listedNames(page)).sort()).toEqual(all.map(g => g.name).sort());
    });

    test('search by name and by ID, then clear the search', async ({ page }) => {
        await page.goto(PAGE);
        const all = await groups(page);
        expect(all.length).toBeGreaterThan(0);
        const target = all.find(g => g.name === 'user') ?? all[0];
        await expect(entry(page, target.name)).toBeVisible();

        await searchBox(page).fill(target.name.toUpperCase());
        const byName = all
            .filter(g => g.name.toLowerCase().includes(target.name.toLowerCase()))
            .map(g => g.name);
        expect(byName.length).toBeGreaterThan(0);
        await expect(entryList(page).getByRole('listitem')).toHaveCount(byName.length);
        expect((await listedNames(page)).sort()).toEqual(byName.sort());

        await searchBox(page).fill('no-such-group-xyz');
        await expect(entryList(page).getByRole('listitem')).toHaveCount(0);

        await chooseOption(
            page,
            subNav(page).getByRole('combobox', { name: 'Search Options' }),
            'ID',
        );
        await searchBox(page).fill(target.id.slice(2, 14));
        await expect(entryList(page).getByRole('listitem')).toHaveCount(1);
        await expect(entry(page, target.name)).toBeVisible();

        await searchBox(page).fill('admin');
        await expect(entryList(page).getByRole('listitem')).toHaveCount(
            all.filter(g => g.id.toLowerCase().includes('admin')).length,
        );

        await searchClear(page).click();
        await expect(searchBox(page)).toHaveValue('');
        await expect(entryList(page).getByRole('listitem')).toHaveCount(all.length);
    });

    test('sort by name ascending / descending and by ID', async ({ page }) => {
        await page.goto(PAGE);
        const all = await groups(page);
        const names = all.map(g => g.name);
        await expect(entryList(page).getByRole('listitem')).toHaveCount(all.length);

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
            all.map(g => g.id),
        );
        const byId = ids.map(id => /** @type {Group} */ (all.find(g => g.id === id)).name);
        await expect.poll(() => listedNames(page)).toEqual(byId);
    });

    test('selecting a group shows its details and a deep link', async ({ page }) => {
        await page.goto(PAGE);
        const all = await groups(page);
        const user = /** @type {Group} */ (all.find(g => g.name === 'user'));

        await openEntry(page, 'user');
        await expect(page).toHaveURL(new RegExp(`[?&]gid=${user.id}`));
        await expectCurrent(page, 'user');
        await expect(nameInput(page)).toHaveValue('user');
        await expect(page.getByRole('main').getByText(user.id)).toBeVisible();

        await page.goto(`${PAGE}?gid=${user.id}`);
        await expect(nameInput(page)).toHaveValue('user');
        await expectCurrent(page, 'user');

        const other = /** @type {Group} */ (all.find(g => g.id !== user.id));
        expect(other).toBeDefined();
        await openEntry(page, other.name);
        await expect(nameInput(page)).toHaveValue(other.name);
    });
});

test.describe('create group', () => {
    /** @type {string[]} */
    let cleanup = [];
    test.afterEach(async ({ page }) => {
        const all = await groups(page);
        for (const name of cleanup) {
            const g = all.find(x => x.name === name);
            if (g) await deleteQuietly(page, `groups/${g.id}`);
        }
        cleanup = [];
    });

    test('dialog validates the name and can be closed without saving', async ({ page }) => {
        await page.goto(PAGE);
        const duplicateName = unique('e2e-group');
        cleanup.push(duplicateName);
        await api(page, 'POST', 'groups', { group: duplicateName });
        await page.reload();
        const posts = recordRequests(page, 'POST', /\/auth\/v1\/groups$/);

        let dialog = await openAddNew(page);
        const name = dialog.getByRole('textbox', { name: 'Group Name' });
        await expect(name).toBeFocused();

        await dialog.getByRole('button', { name: 'Save' }).click();
        await expect(name).toHaveAttribute('aria-invalid', 'true');

        await name.fill('bad group!');
        await expect(dialog.getByText('Invalid Input')).toBeVisible();
        await expect(name).toHaveAttribute('aria-invalid', 'true');
        await dialog.getByRole('button', { name: 'Save' }).click();

        await name.fill('a');
        await expect(dialog.getByText('Invalid Input')).toBeVisible();
        await dialog.getByRole('button', { name: 'Save' }).click();

        await name.fill(duplicateName);
        await expect(dialog.getByText('Invalid Input')).toBeHidden();
        await dialog.getByRole('button', { name: 'Save' }).click();
        await expect(dialog.getByText('Name exists already')).toBeVisible();
        expect(posts).toEqual([]);

        await dialog.getByRole('button', { name: 'Close' }).click();
        await expect(dialog).toBeHidden();

        dialog = await openAddNew(page);
        await page.keyboard.press('Escape');
        await expect(dialog).toBeHidden();
        expect(posts).toEqual([]);
    });

    test('creates a group with spaces in its name and JSON metadata', async ({ page }) => {
        // unlike roles, group names may contain spaces
        const name = unique('e2e group');
        cleanup.push(name);
        await page.goto(PAGE);

        const dialog = await openAddNew(page);
        await dialog.getByRole('textbox', { name: 'Group Name' }).fill(name);
        await dialog
            .getByRole('textbox', { name: 'Metadata as JSON value' })
            .fill('{"department": "qa", "size": 7}');
        await dialog.getByRole('button', { name: 'Save' }).click();
        await expect(dialog).toBeHidden();

        const created = /** @type {Group} */ ((await groups(page)).find(g => g.name === name));
        expect(created.meta).toEqual({ department: 'qa', size: 7 });

        await expect(page).toHaveURL(new RegExp(`[?&]gid=${created.id}`));
        await showSubNav(page);
        await expect(entry(page, name)).toBeVisible();
        await expectCurrent(page, name);
        await expect(nameInput(page)).toHaveValue(name);
        await expectJsonValue(metaInput(page), { department: 'qa', size: 7 });

        await page.reload();
        await expect(nameInput(page)).toHaveValue(name);
    });
});

test.describe('edit group', () => {
    /** @type {Group} */
    let group;
    test.beforeEach(async ({ page }) => {
        await page.goto(PAGE);
        group = await api(page, 'POST', 'groups', { group: unique('e2e-group'), meta: 'initial' });
    });
    test.afterEach(async ({ page }) => {
        await deleteQuietly(page, `groups/${group.id}`);
    });

    test('edit metadata, save, reload: persisted', async ({ page }) => {
        await page.goto(`${PAGE}?gid=${group.id}`);
        await expect(nameInput(page)).toHaveValue(group.name);
        await expect(metaInput(page)).toHaveValue('initial');

        await metaInput(page).fill('{"owner": "e2e", "active": true}');
        await saveButton(page).click();
        await expect
            .poll(async () => (await groupById(page, group.id))?.meta)
            .toEqual({ owner: 'e2e', active: true });
        await page.reload();
        await expectJsonValue(metaInput(page), { owner: 'e2e', active: true });

        await metaInput(page).fill('');
        await saveButton(page).click();
        await expect.poll(async () => (await groupById(page, group.id))?.meta).toBeUndefined();
        await page.reload();
        await expect(metaInput(page)).toHaveValue('');
    });

    test('rename keeps the group on users in it (#1861)', async ({ page }) => {
        const user = await createUser(page, {
            email: `${unique('e2e-group-user')}@example.com`,
            groups: [group.name],
        });
        try {
            const newName = unique('e2e renamed');
            await page.goto(`${PAGE}?gid=${group.id}`);
            await expect(nameInput(page)).toHaveValue(group.name);
            await nameInput(page).fill(newName);
            await saveButton(page).click();

            await expect.poll(async () => (await groupById(page, group.id))?.name).toBe(newName);
            await showSubNav(page);
            await expect(entry(page, newName)).toBeVisible();
            await expect(entry(page, group.name)).toHaveCount(0);
            await page.reload();
            await expect(nameInput(page)).toHaveValue(newName);

            const u = await api(page, 'GET', `users/${user.id}`);
            expect(u.groups).toContain(newName);
            expect(u.groups).not.toContain(group.name);
        } finally {
            await deleteQuietly(page, `users/${user.id}`);
        }
    });

    test('invalid or duplicate name shows an error and blocks the save', async ({ page }) => {
        await page.goto(`${PAGE}?gid=${group.id}`);
        await expect(nameInput(page)).toHaveValue(group.name);
        const puts = recordRequests(page, 'PUT', /\/auth\/v1\/groups\//);

        await nameInput(page).fill('not.allowed');
        await expect(page.getByRole('main').getByText('Invalid Input')).toBeVisible();
        await expect(nameInput(page)).toHaveAttribute('aria-invalid', 'true');
        await saveButton(page).click();

        await nameInput(page).fill('');
        await saveButton(page).click();
        await expect(nameInput(page)).toHaveAttribute('aria-invalid', 'true');

        await nameInput(page).fill('user');
        await saveButton(page).click();
        await expect(page.getByRole('main').getByText('Name exists already')).toBeVisible();

        expect(puts).toEqual([]);
        expect((await groupById(page, group.id))?.name).toBe(group.name);
    });
});

test.describe('delete group', () => {
    test('header action asks first, cancelling keeps the entry, confirming deletes it', async ({
        page,
    }) => {
        await page.goto(PAGE);
        const group = await api(page, 'POST', 'groups', { group: unique('e2e-group') });
        const user = await createUser(page, {
            email: `${unique('e2e-group-del')}@example.com`,
            groups: [group.name],
        });
        try {
            await page.goto(`${PAGE}?gid=${group.id}`);
            await expect(nameInput(page)).toHaveValue(group.name);

            await openEntityDelete(page);
            await expect(
                page.getByRole('main').getByText('Are you sure you want to delete this group?'),
            ).toBeVisible();

            await page
                .getByRole('dialog', { name: 'Delete' })
                .getByRole('button', { name: 'Cancel' })
                .click();
            await expect(nameInput(page)).toHaveValue(group.name);
            expect(await groupById(page, group.id)).toBeTruthy();

            await openEntityDelete(page);
            await deleteAction(page).click();
            await expect.poll(async () => await groupById(page, group.id)).toBeUndefined();
            await expect(entry(page, group.name)).toHaveCount(0);
            await expect(nameInput(page)).toHaveCount(0);

            const u = await api(page, 'GET', `users/${user.id}`);
            expect(u.groups || []).not.toContain(group.name);
        } finally {
            await deleteQuietly(page, `users/${user.id}`);
            await deleteQuietly(page, `groups/${group.id}`);
        }
    });
});

test.describe('groups on a phone', () => {
    test('create, edit and delete a group @mobile', async ({ page }) => {
        const name = unique('e2e-group-m');
        await page.goto(PAGE);
        try {
            const dialog = await openAddNew(page);
            await dialog.getByRole('textbox', { name: 'Group Name' }).fill(name);
            await dialog.getByRole('button', { name: 'Save' }).click();
            await expect(dialog).toBeHidden();
            await expect(nameInput(page)).toHaveValue(name);

            await metaInput(page).fill('{"mobile": true}');
            await saveButton(page).click();
            await expect
                .poll(async () => (await groups(page)).find(g => g.name === name)?.meta)
                .toEqual({ mobile: true });

            await page.goto(PAGE);
            await openEntry(page, name);
            await expectJsonValue(metaInput(page), { mobile: true });
            await openEntityDelete(page);
            await deleteAction(page).click();
            await expect
                .poll(async () => (await groups(page)).some(g => g.name === name))
                .toBe(false);
        } finally {
            const g = (await groups(page)).find(x => x.name === name);
            if (g) await deleteQuietly(page, `groups/${g.id}`);
        }
    });
});
