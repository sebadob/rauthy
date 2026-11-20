import { openEntityDelete, entityDelete } from './helpers/entities.mjs';
// Characterization tests for the admin Attributes section: list, search, order, Add New,
// description / default value / user editable, rename, Delete, and that a new attribute shows up
// in a user's Attributes tab.
import { test, expect } from './fixtures.mjs';
import { api } from './helpers/api.mjs';
import { unique } from './helpers/env.mjs';
import {
    clickDeleteAndConfirm,
    countRequests,
    entries,
    entry,
    entryKeys,
    expectInvalid,
    openAddNew,
    openSubNav,
    saveAndWait,
    saveButton,
    subNav,
    tab,
    expectEntryCurrent,
} from './helpers/clients.mjs';

const ATTRS = '/auth/v1/admin/attributes';
/** Attribute names allow `[a-zA-Z0-9-_/]{2,32}`. */
const attrName = (/** @type {string} */ prefix) => unique(prefix).replaceAll('-', '_');

/** @param {import('@playwright/test').Page} page */
async function allAttrs(page) {
    return /** @type {{ name: string, desc?: string, default_value?: any, user_editable: boolean }[]} */ (
        (await api(page, 'GET', 'users/attr')).values
    );
}

/**
 * @param {import('@playwright/test').Page} page
 * @param {string} name
 */
async function attrByName(page, name) {
    return (await allAttrs(page)).find(a => a.name === name);
}

/**
 * @param {import('@playwright/test').Page} page
 * @param {string} name
 */
async function deleteAttrQuietly(page, name) {
    try {
        await api(page, 'DELETE', `users/attr/${name}`);
    } catch (e) {
        if (!/-> 404/.test(String(e))) throw e;
    }
}

/**
 * @param {import('@playwright/test').Page} page
 * @param {string} name
 */
async function openAttr(page, name) {
    await page.goto(`${ATTRS}?an=${encodeURIComponent(name)}`);
    await expect(page.getByRole('textbox', { name: 'Attribute Name' })).toHaveValue(name);
}

test.describe('attributes list', () => {
    test('loads all attributes with the count of the API', async ({ page }) => {
        await page.goto(ATTRS);
        const all = await allAttrs(page);
        await expect(entries(page)).toHaveCount(all.length);
        for (const a of all) await expect(entry(page, a.name)).toBeVisible();
    });

    test('search by name, sort ascending / descending', async ({ page }) => {
        const extra = attrName('e2e_zz');
        await page.goto(ATTRS);
        await api(page, 'POST', 'users/attr', { name: extra });
        try {
            await page.reload();
            const names = (await allAttrs(page)).map(a => a.name);
            await expect(entries(page)).toHaveCount(names.length);
            await expect(
                subNav(page).getByRole('combobox', { name: 'Search Options' }),
            ).toContainText('Name');
            const asc = [...names].sort((a, b) => a.localeCompare(b));
            await expect.poll(() => entryKeys(page)).toEqual(asc);

            await subNav(page).getByRole('button', { name: 'Change sort to descending' }).click();
            await expect.poll(() => entryKeys(page)).toEqual([...asc].reverse());
            await subNav(page).getByRole('button', { name: 'Change sort to ascending' }).click();
            await expect.poll(() => entryKeys(page)).toEqual(asc);

            const search = subNav(page).getByRole('searchbox', { name: 'Search' });
            await search.fill(extra.toUpperCase());
            await expect(entries(page)).toHaveCount(1);
            await expect(entry(page, extra)).toBeVisible();
            await search.fill('nothing-matches-this');
            await expect(entries(page)).toHaveCount(0);
            await search.fill('');
            await expect(entries(page)).toHaveCount(names.length);
        } finally {
            await deleteAttrQuietly(page, extra);
        }
    });

    test('selecting an attribute shows its details and sets the deep link', async ({ page }) => {
        await page.goto(ATTRS);
        const first = (await allAttrs(page))[0];
        await entry(page, first.name).click();
        await expect(page).toHaveURL(new RegExp(`[?&]an=${first.name}(&|$)`));
        await expectEntryCurrent(entry(page, first.name));
        await expect(page.getByRole('textbox', { name: 'Attribute Name' })).toHaveValue(first.name);
        await expect(page.getByRole('textbox', { name: 'Description' })).toHaveValue(
            first.desc || '',
        );
        await expect(entityDelete(page)).toBeVisible();

        await page.reload();
        await expect(page.getByRole('textbox', { name: 'Attribute Name' })).toHaveValue(first.name);
    });
});

test.describe('add new attribute', () => {
    /** @type {string[]} */
    let created = [];
    test.afterEach(async ({ page }) => {
        for (const name of created) await deleteAttrQuietly(page, name);
        created = [];
    });

    test('validates name and description and can be cancelled', async ({ page }) => {
        await page.goto(ATTRS);
        const before = (await allAttrs(page)).length;
        const dialog = await openAddNew(page);
        const name = dialog.getByRole('textbox', { name: 'Attribute Name' });
        const desc = dialog.getByRole('textbox', { name: 'Description' });
        const save = dialog.getByRole('button', { name: 'Save' });
        await expect(name).toBeFocused();

        expect(await countRequests(page, 'POST', '/users/attr', () => save.click())).toBe(0);
        await expectInvalid(name);

        for (const bad of ['a', 'with space', 'dot.ted', 'x'.repeat(33)]) {
            await name.fill(bad);
            expect(await countRequests(page, 'POST', '/users/attr', () => save.click()), bad).toBe(
                0,
            );
            await expectInvalid(name);
        }

        await name.fill(attrName('e2e_val'));
        await desc.fill('not allowed: punctuation!');
        expect(await countRequests(page, 'POST', '/users/attr', () => save.click())).toBe(0);
        await expectInvalid(desc);

        const existing = (await allAttrs(page))[0].name;
        await desc.fill('');
        await name.fill(existing);
        expect(await countRequests(page, 'POST', '/users/attr', () => save.click())).toBe(0);
        await expect(dialog.getByText('Name exists already')).toBeVisible();

        await dialog.getByRole('button', { name: 'Close' }).first().click();
        await expect(dialog).toBeHidden();
        expect((await allAttrs(page)).length).toBe(before);
    });

    test('creates an attribute with description', async ({ page }) => {
        const name = attrName('e2e_new');
        created.push(name);
        await page.goto(ATTRS);
        const dialog = await openAddNew(page);
        await dialog.getByRole('textbox', { name: 'Attribute Name' }).fill(name);
        await dialog.getByRole('textbox', { name: 'Description' }).fill('E2E description');
        await saveAndWait(
            page,
            dialog.getByRole('button', { name: 'Save' }),
            'POST',
            '/users/attr',
        );
        await expect(dialog).toBeHidden();

        await expect(page).toHaveURL(new RegExp(`[?&]an=${name}(&|$)`));
        await expectEntryCurrent(entry(page, name));
        await expect(page.getByRole('textbox', { name: 'Attribute Name' })).toHaveValue(name);
        await expect(page.getByRole('textbox', { name: 'Description' })).toHaveValue(
            'E2E description',
        );
        expect(await attrByName(page, name)).toMatchObject({
            name,
            desc: 'E2E description',
            user_editable: false,
        });
    });
});

test.describe('attribute config', () => {
    /** @type {string} */
    let name;
    test.beforeEach(async ({ page }) => {
        name = attrName('e2e_cfg');
        await page.goto(ATTRS);
        await api(page, 'POST', 'users/attr', { name, desc: 'Initial' });
        await openAttr(page, name);
    });
    test.afterEach(async ({ page }) => {
        await deleteAttrQuietly(page, name);
        await deleteAttrQuietly(page, `${name}_r`);
    });

    test('edits description and default value', async ({ page }) => {
        const desc = page.getByRole('textbox', { name: 'Description' });
        const def = page.getByRole('textbox', { name: 'Default Value' });
        await expect(def).toHaveAccessibleDescription('String / JSON');
        await expect(desc).toHaveValue('Initial');
        await expect(def).toHaveValue('');

        await desc.fill('bad!');
        await expectInvalid(desc);
        expect(
            await countRequests(page, 'PUT', `/users/attr/${name}`, () => saveButton(page).click()),
        ).toBe(0);

        await desc.fill('Changed description');
        await def.fill('{"level": 1}');
        await saveAndWait(page, saveButton(page), 'PUT', `/users/attr/${name}`);
        expect(await attrByName(page, name)).toMatchObject({
            desc: 'Changed description',
            default_value: { level: 1 },
        });

        await page.reload();
        await expect(desc).toHaveValue('Changed description');
        await expect(def).toHaveValue(/"level": 1/);

        // a plain string default value
        await def.fill('fallback');
        await desc.fill('');
        await saveAndWait(page, saveButton(page), 'PUT', `/users/attr/${name}`);
        const a = await attrByName(page, name);
        expect(a?.default_value).toBe('fallback');
        expect(a?.desc ?? '').toBe('');
    });

    test('makes the attribute user editable after a warning', async ({ page }) => {
        const main = page.getByRole('main');
        // FINDING: the editable state is only an unnamed check icon next to "User Editable"
        await expect(main.getByText('User Editable')).toBeVisible();
        const toggle = main.getByRole('button', { name: 'Make Editable' });
        await expect(toggle).toHaveCount(1);

        await toggle.click();
        await expect(
            main.getByText(
                'You can convert this attribute and make it editable by users themselves.',
            ),
        ).toBeVisible();
        await expect(main.getByText(/This can never be changed back!/)).toBeVisible();
        const buttons = main.getByRole('button', { name: 'Make Editable' });
        await expect(buttons).toHaveCount(2);

        // clicking the toggle again hides the warning ("cancel")
        await buttons.first().click();
        await expect(main.getByText(/This can never be changed back!/)).toHaveCount(0);
        expect((await attrByName(page, name))?.user_editable).toBe(false);

        await toggle.click();
        await saveAndWait(page, buttons.last(), 'PUT', `/users/attr/${name}`);
        expect((await attrByName(page, name))?.user_editable).toBe(true);
        await expect(main.getByRole('button', { name: 'Make Editable' })).toHaveCount(0);

        await page.reload();
        await expect(page.getByRole('textbox', { name: 'Attribute Name' })).toHaveValue(name);
        await expect(main.getByRole('button', { name: 'Make Editable' })).toHaveCount(0);
    });

    test('renames the attribute and validates the name', async ({ page }) => {
        const input = page.getByRole('textbox', { name: 'Attribute Name' });
        await input.fill('a');
        await expectInvalid(input);
        expect(
            await countRequests(page, 'PUT', `/users/attr/${name}`, () => saveButton(page).click()),
        ).toBe(0);

        const other = (await allAttrs(page)).find(a => a.name !== name);
        await input.fill(/** @type {any} */ (other).name);
        expect(
            await countRequests(page, 'PUT', `/users/attr/${name}`, () => saveButton(page).click()),
        ).toBe(0);
        await expect(page.getByText('Name exists already')).toBeVisible();

        const renamed = `${name}_r`;
        await input.fill(renamed);
        await saveAndWait(page, saveButton(page), 'PUT', `/users/attr/${name}`);
        await expect(entry(page, renamed)).toBeVisible();
        await expect(entry(page, name)).toHaveCount(0);
        expect(await attrByName(page, renamed)).toBeTruthy();
        expect(await attrByName(page, name)).toBeUndefined();
    });

    test('cancelling keeps the attribute, confirming deletes it', async ({ page }) => {
        await openEntityDelete(page);
        await expect(
            page.getByText('Are you sure you want to delete this attribute?'),
        ).toBeVisible();
        await page
            .getByRole('dialog', { name: 'Delete' })
            .getByRole('button', { name: 'Cancel' })
            .click();
        await expect(page.getByRole('textbox', { name: 'Attribute Name' })).toHaveValue(name);
        expect(await attrByName(page, name)).toBeTruthy();

        expect(await clickDeleteAndConfirm(page, `/users/attr/${name}`)).toBeLessThan(400);
        await expect(entry(page, name)).toHaveCount(0);
        await expect(page.getByRole('textbox', { name: 'Attribute Name' })).toHaveCount(0);
        expect(await attrByName(page, name)).toBeUndefined();
    });
});

test.describe('attributes and users', () => {
    test("a new attribute appears in a user's Attributes tab and stores values", async ({
        page,
    }) => {
        const name = attrName('e2e_usr');
        const email = `${unique('e2e-attr')}@example.com`;
        await page.goto(ATTRS);
        await api(page, 'POST', 'users/attr', { name, desc: 'For users' });
        const user = await api(page, 'POST', 'users', {
            email,
            given_name: 'Attr',
            family_name: 'Tester',
            language: 'en',
            roles: [],
        });
        try {
            await page.goto(`/auth/v1/admin/users?uid=${user.id}`);
            await tab(page, 'Attributes').click();
            const input = page.getByRole('textbox', { name, exact: true });
            await expect(input).toBeVisible();
            // the description is used as placeholder
            await expect(input).toHaveAttribute('placeholder', 'For users');

            await input.fill('"e2e value"');
            await saveAndWait(
                page,
                page.getByRole('main').getByRole('button', { name: 'Save', exact: true }),
                'PUT',
                `/users/${user.id}/attr`,
            );
            const values = (await api(page, 'GET', `users/${user.id}/attr`)).values;
            expect(values).toContainEqual({ key: name, value: 'e2e value' });

            // deleting the attribute removes it from the user as well
            await api(page, 'DELETE', `users/attr/${name}`);
            await page.reload();
            await tab(page, 'Attributes').click();
            await expect(page.getByRole('textbox', { name, exact: true })).toHaveCount(0);
        } finally {
            await deleteAttrQuietly(page, name);
            await api(page, 'DELETE', `users/${user.id}`);
        }
    });
});

test.describe('attributes at phone size', () => {
    test('create, edit and delete an attribute @mobile', async ({ page }) => {
        const name = attrName('e2e_mob');
        await page.goto(ATTRS);
        try {
            const dialog = await openAddNew(page);
            await dialog.getByRole('textbox', { name: 'Attribute Name' }).fill(name);
            await saveAndWait(
                page,
                dialog.getByRole('button', { name: 'Save' }),
                'POST',
                '/users/attr',
            );
            await expect(page).toHaveURL(new RegExp(`[?&]an=${name}(&|$)`));

            await page.getByRole('textbox', { name: 'Description' }).fill('Mobile');
            await saveAndWait(page, saveButton(page), 'PUT', `/users/attr/${name}`);
            expect((await attrByName(page, name))?.desc).toBe('Mobile');

            expect(await clickDeleteAndConfirm(page, `/users/attr/${name}`)).toBeLessThan(400);
            expect(await attrByName(page, name)).toBeUndefined();
        } finally {
            await deleteAttrQuietly(page, name);
        }
    });

    test('list and deep link @mobile', async ({ page }) => {
        await page.goto(ATTRS);
        await openSubNav(page);
        const first = (await allAttrs(page))[0];
        await entry(page, first.name).click();
        await expect(page).toHaveURL(new RegExp(`[?&]an=${first.name}(&|$)`));
        await expect(page.getByRole('textbox', { name: 'Attribute Name' })).toHaveValue(first.name);
    });
});
