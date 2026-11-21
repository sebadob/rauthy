import { openEntityDelete } from './helpers/entities.mjs';
import { enterTime } from './helpers/ui.mjs';
import { test, expect } from './fixtures.mjs';
import { api } from './helpers/api.mjs';
import { unique } from './helpers/env.mjs';
import {
    deleteAction,
    deleteQuietly,
    entry,
    entryList,
    expectCurrent,
    expectTabSelected,
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
    tab,
} from './helpers/entities.mjs';

const PAGE = '/auth/v1/admin/api_keys';

const GROUPS = [
    'Blacklist',
    'Clients',
    'Events',
    'Generic',
    'Groups',
    'Pam',
    'Roles',
    'Secrets',
    'Sessions',
    'Scopes',
    'UserAttributes',
    'Users',
    'AuthProviders',
    'ApiKeys',
];
const RIGHTS = ['create', 'read', 'update', 'delete'];

/** @typedef {{ group: string, access_rights: string[] }} Access */
/** @typedef {{ name: string, created: number, expires?: number, access: Access[] }} ApiKey */

/** API key names allow at most 24 characters. */
const keyName = (prefix = 'e2ek') => unique(prefix);

/** @param {import('@playwright/test').Page} page */
async function keys(page) {
    return /** @type {ApiKey[]} */ ((await api(page, 'GET', 'api_keys')).keys);
}

/** @param {import('@playwright/test').Page} page @param {string} name */
async function keyByName(page, name) {
    return (await keys(page)).find(k => k.name === name);
}

/** Access rights as a sorted `{ group: rights[] }` map, independent of the order. */
function accessMap(/** @type {Access[] | undefined} */ access) {
    /** @type {Record<string, string[]>} */
    const m = {};
    for (const a of access || []) m[a.group] = [...a.access_rights].sort();
    return m;
}

/** @param {import('@playwright/test').Locator | import('@playwright/test').Page} scope */
function matrixBox(scope, /** @type {string} */ group, /** @type {string} */ right) {
    return scope.getByRole('checkbox', { name: `${group}: ${right}`, exact: true });
}

/**
 * The date and time inputs of the expiry.
 * FINDING: upstream both inputs are called "Expiry" (`lib/form/InputDateTimeCombo.svelte`), so the
 * date is the first and the time the last one.
 */
function expiry(/** @type {import('@playwright/test').Locator} */ scope) {
    const all = scope.getByRole('textbox', { name: /^Expiry/ });
    return { date: all.first(), time: all.last() };
}

/** @param {import('@playwright/test').Page} page */
async function localTs(page, /** @type {string} */ date, /** @type {string} */ time) {
    return page.evaluate(([d, t]) => Date.parse(`${d}T${t}`) / 1000, [date, time]);
}

test.describe('API keys list', () => {
    /** @type {string[]} */
    let created = [];
    test.afterEach(async ({ page }) => {
        for (const n of created) await deleteQuietly(page, `api_keys/${n}`);
        created = [];
    });

    test('loads every key, sorts, searches and deep links', async ({ page }) => {
        await page.goto(PAGE);
        const a = keyName('e2ek-a');
        const b = keyName('e2ek-b');
        for (const n of [a, b]) {
            await api(page, 'POST', 'api_keys', {
                name: n,
                access: [{ group: 'Roles', access_rights: ['read'] }],
            });
            created.push(n);
        }
        await page.goto(PAGE);
        const all = await keys(page);
        await expect(entry(page, a)).toBeVisible();
        await expect(entryList(page).getByRole('listitem')).toHaveCount(all.length);

        const names = all.map(k => k.name);
        await expect.poll(() => listedNames(page)).toEqual(await sortedInBrowser(page, names));
        await subNav(page).getByRole('button', { name: 'Change sort to descending' }).click();
        await expect
            .poll(() => listedNames(page))
            .toEqual(await sortedInBrowser(page, names, true));
        await subNav(page).getByRole('button', { name: 'Change sort to ascending' }).click();
        await expect.poll(() => listedNames(page)).toEqual(await sortedInBrowser(page, names));

        await searchBox(page).fill(a);
        await expect(entryList(page).getByRole('listitem')).toHaveCount(1);
        await expect(entry(page, a)).toBeVisible();
        await searchBox(page).fill('e2ek-');
        await expect(entry(page, a)).toBeVisible();
        await expect(entry(page, b)).toBeVisible();
        await searchBox(page).fill('nothing-matches');
        await expect(entryList(page).getByRole('listitem')).toHaveCount(0);
        await searchClear(page).click();
        await expect(entryList(page).getByRole('listitem')).toHaveCount(all.length);

        await openEntry(page, b);
        await expect(page).toHaveURL(new RegExp(`[?&]kn=${b}`));
        await expectCurrent(page, b);
        await expectTabSelected(page, 'Config');
        await expect(page.getByRole('main').getByText(b, { exact: true }).last()).toBeVisible();
        await expect(matrixBox(page.getByRole('main'), 'Roles', 'read')).toBeChecked();

        await page.goto(`${PAGE}?kn=${a}`);
        await expectCurrent(page, a);
        await expect(matrixBox(page.getByRole('main'), 'Roles', 'read')).toBeChecked();
        await expect(matrixBox(page.getByRole('main'), 'Roles', 'create')).not.toBeChecked();
    });

    test('search finds keys with upper-case letters', async ({ page }) => {
        await page.goto(PAGE);
        const name = keyName('E2EK');
        await api(page, 'POST', 'api_keys', {
            name,
            access: [{ group: 'Roles', access_rights: ['read'] }],
        });
        created.push(name);
        await page.goto(PAGE);
        await expect(entry(page, name)).toBeVisible();
        // Wait for the debounced filter before checking whether this key reappears.
        await searchBox(page).fill('nothing-matches');
        await expect(entry(page, name)).toHaveCount(0);
        await searchBox(page).fill(name.toLowerCase());
        await expect(entry(page, name)).toBeVisible({ timeout: 3_000 });
    });
});

test.describe('create API key', () => {
    /** @type {string[]} */
    let created = [];
    test.afterEach(async ({ page }) => {
        for (const n of created) await deleteQuietly(page, `api_keys/${n}`);
        created = [];
    });

    test('dialog validates name and permissions and can be closed', async ({ page }) => {
        await page.goto(PAGE);
        await expect(entry(page, 'bootstrap')).toBeVisible();
        const posts = recordRequests(page, 'POST', /\/auth\/v1\/api_keys$/);

        const dialog = await openAddNew(page);
        const name = dialog.getByRole('textbox', { name: 'Key Name' });
        await expect(dialog.getByRole('checkbox', { name: 'Limited Validity' })).not.toBeChecked();
        await expect(expiry(dialog).date).toHaveCount(0);
        for (const g of GROUPS) {
            for (const r of RIGHTS) await expect(matrixBox(dialog, g, r)).not.toBeChecked();
        }

        // required
        await dialog.getByRole('button', { name: 'Save' }).click();
        await expect(name).toHaveAttribute('aria-invalid', 'true');

        // invalid characters
        await name.fill('bad key!');
        await expect(dialog.getByText('Invalid Input')).toBeVisible();
        await dialog.getByRole('button', { name: 'Save' }).click();

        // too long (max 24)
        await name.fill('a'.repeat(25));
        await expect(dialog.getByText('Invalid Input')).toBeVisible();

        // no permission at all
        await name.fill(keyName());
        await expect(dialog.getByText('Invalid Input')).toBeHidden();
        await dialog.getByRole('button', { name: 'Save' }).click();
        await expect(
            dialog.getByText('Grant the new API Key at least one permission'),
        ).toBeVisible();

        // duplicate name
        await name.fill('bootstrap');
        await matrixBox(dialog, 'Roles', 'read').check();
        await dialog.getByRole('button', { name: 'Save' }).click();
        await expect(dialog.getByText('Name already exists')).toBeVisible();
        expect(posts).toEqual([]);

        await dialog.getByRole('button', { name: 'Close' }).click();
        await expect(dialog).toBeHidden();
    });

    test('creates a key without expiry using the matrix toggles', async ({ page }) => {
        const name = keyName();
        created.push(name);
        await page.goto(PAGE);
        await expect(entry(page, 'bootstrap')).toBeVisible();

        const dialog = await openAddNew(page);
        await dialog.getByRole('textbox', { name: 'Key Name' }).fill(name);
        // single boxes
        await matrixBox(dialog, 'Roles', 'read').check();
        await matrixBox(dialog, 'Groups', 'read').check();
        await matrixBox(dialog, 'Groups', 'update').check();
        // a whole row
        await dialog.getByRole('button', { name: 'Toggle: Clients' }).click();
        for (const r of RIGHTS) await expect(matrixBox(dialog, 'Clients', r)).toBeChecked();
        // a row toggles off again when it is complete
        await dialog.getByRole('button', { name: 'Toggle: Clients' }).click();
        for (const r of RIGHTS) await expect(matrixBox(dialog, 'Clients', r)).not.toBeChecked();
        await dialog.getByRole('button', { name: 'Toggle: Scopes' }).click();
        // a whole column, twice: on for every group, then off for every group
        await dialog.getByRole('button', { name: 'Toggle: create' }).click();
        for (const g of GROUPS) await expect(matrixBox(dialog, g, 'create')).toBeChecked();
        await dialog.getByRole('button', { name: 'Toggle: create' }).click();
        for (const g of GROUPS) await expect(matrixBox(dialog, g, 'create')).not.toBeChecked();

        await dialog.getByRole('button', { name: 'Save' }).click();
        await expect(dialog).toBeHidden();
        await showSubNav(page);
        await expect(entry(page, name)).toBeVisible();

        const key = /** @type {ApiKey} */ (await keyByName(page, name));
        // the API returns `null` for a key without expiry
        expect(key.expires ?? null).toBeNull();
        expect(accessMap(key.access)).toEqual({
            Roles: ['read'],
            Groups: ['read', 'update'],
            Scopes: ['read', 'update', 'delete'].sort(),
        });

        await openEntry(page, name);
        await expect(
            page.getByRole('main').getByRole('checkbox', { name: 'Limited Validity' }),
        ).not.toBeChecked();
        await expect(matrixBox(page.getByRole('main'), 'Groups', 'update')).toBeChecked();
        await expect(matrixBox(page.getByRole('main'), 'Scopes', 'create')).not.toBeChecked();
    });

    test('creates a key with an expiry', async ({ page }) => {
        const name = keyName();
        created.push(name);
        await page.goto(PAGE);
        await expect(entry(page, 'bootstrap')).toBeVisible();

        const dialog = await openAddNew(page);
        await dialog.getByRole('textbox', { name: 'Key Name' }).fill(name);
        await dialog.getByRole('checkbox', { name: 'Limited Validity' }).check();
        const { date, time } = expiry(dialog);
        await expect(date).toBeVisible();
        await date.fill('2031-03-04');
        await enterTime(time, '08:15');
        await expect(time).toHaveValue('08:15');
        await matrixBox(dialog, 'Events', 'read').check();
        await dialog.getByRole('button', { name: 'Save' }).click();
        await expect(dialog).toBeHidden();

        const key = /** @type {ApiKey} */ (await keyByName(page, name));
        expect(key.expires).toBe(await localTs(page, '2031-03-04', '08:15'));
        expect(accessMap(key.access)).toEqual({ Events: ['read'] });

        await page.goto(`${PAGE}?kn=${name}`);
        const main = page.getByRole('main');
        await expect(main.getByRole('checkbox', { name: 'Limited Validity' })).toBeChecked();
        await expect(expiry(main).date).toHaveValue('2031-03-04');
        await expect(expiry(main).time).toHaveValue('08:15');
    });
});

test.describe('edit API key', () => {
    /** @type {string} */
    let name;
    test.beforeEach(async ({ page }) => {
        await page.goto(PAGE);
        name = keyName();
        await api(page, 'POST', 'api_keys', {
            name,
            access: [{ group: 'Roles', access_rights: ['read'] }],
        });
    });
    test.afterEach(async ({ page }) => {
        await deleteQuietly(page, `api_keys/${name}`);
    });

    test('permission matrix: toggle, save, reload, persisted', async ({ page }) => {
        await page.goto(`${PAGE}?kn=${name}`);
        const main = page.getByRole('main');
        await expect(matrixBox(main, 'Roles', 'read')).toBeChecked();

        await matrixBox(main, 'Groups', 'read').check();
        await matrixBox(main, 'Groups', 'update').check();
        await main.getByRole('button', { name: 'Toggle: Users' }).click();
        await main.getByRole('button', { name: 'Toggle: delete' }).click();
        for (const g of GROUPS) await expect(matrixBox(main, g, 'delete')).toBeChecked();
        await matrixBox(main, 'Roles', 'read').uncheck();
        await main.getByRole('button', { name: 'Toggle: delete' }).click();
        for (const g of GROUPS) await expect(matrixBox(main, g, 'delete')).not.toBeChecked();

        await saveButton(page).click();
        await expect
            .poll(async () => accessMap((await keyByName(page, name))?.access))
            .toEqual({
                Groups: ['read', 'update'],
                Users: ['create', 'read', 'update'],
            });

        await page.reload();
        await expect(matrixBox(main, 'Groups', 'read')).toBeChecked();
        await expect(matrixBox(main, 'Groups', 'update')).toBeChecked();
        await expect(matrixBox(main, 'Groups', 'create')).not.toBeChecked();
        for (const r of ['create', 'read', 'update']) {
            await expect(matrixBox(main, 'Users', r)).toBeChecked();
        }
        await expect(matrixBox(main, 'Users', 'delete')).not.toBeChecked();
        await expect(matrixBox(main, 'Roles', 'read')).not.toBeChecked();
    });

    test('saving without any permission shows an error', async ({ page }) => {
        await page.goto(`${PAGE}?kn=${name}`);
        const main = page.getByRole('main');
        const puts = recordRequests(page, 'PUT', /\/auth\/v1\/api_keys\/[^/]+$/);
        await matrixBox(main, 'Roles', 'read').uncheck();
        await saveButton(page).click();
        await expect(main.getByText('Grant the API Key at least one permission')).toBeVisible();
        expect(puts).toEqual([]);
        expect(accessMap((await keyByName(page, name))?.access)).toEqual({ Roles: ['read'] });
    });

    test('add and remove an expiry', async ({ page }) => {
        await page.goto(`${PAGE}?kn=${name}`);
        const main = page.getByRole('main');
        const validity = main.getByRole('checkbox', { name: 'Limited Validity' });
        await expect(validity).not.toBeChecked();
        await expect(expiry(main).date).toHaveCount(0);

        await validity.check();
        await expiry(main).date.fill('2032-07-01');
        await enterTime(expiry(main).time, '17:30');
        await saveButton(page).click();
        const ts = await localTs(page, '2032-07-01', '17:30');
        await expect.poll(async () => (await keyByName(page, name))?.expires).toBe(ts);

        await page.reload();
        await expect(validity).toBeChecked();
        await expect(expiry(main).date).toHaveValue('2032-07-01');
        await expect(expiry(main).time).toHaveValue('17:30');

        await validity.uncheck();
        await expect(expiry(main).date).toHaveCount(0);
        await saveButton(page).click();
        await expect.poll(async () => (await keyByName(page, name))?.expires ?? null).toBeNull();
        await page.reload();
        await expect(validity).not.toBeChecked();
        // permissions were not touched
        expect(accessMap((await keyByName(page, name))?.access)).toEqual({ Roles: ['read'] });
    });
});

test.describe('API key secret', () => {
    test.use({ permissions: ['clipboard-read', 'clipboard-write'] });

    test('generate a secret once, copy it and use it against the API', async ({
        page,
        playwright,
        baseURL,
    }) => {
        await page.goto(PAGE);
        const name = keyName();
        await api(page, 'POST', 'api_keys', {
            name,
            access: [{ group: 'Roles', access_rights: ['read'] }],
        });
        const anon = await playwright.request.newContext({
            baseURL,
            // no admin session: only the API key may authorize these requests
            storageState: { cookies: [], origins: [] },
        });
        try {
            await page.goto(`${PAGE}?kn=${name}`);
            const main = page.getByRole('main');
            await tab(page, 'Secret').click();
            await expectTabSelected(page, 'Secret');
            await expect(main.getByText('You will only see this secret once')).toBeVisible();

            const secretBox = main.getByRole('textbox', { name: 'API Key', exact: true });
            await expect(secretBox).toHaveCount(0);

            await main.getByRole('button', { name: 'Generate' }).click();
            await expect(secretBox).toBeVisible();
            await expect(secretBox).toHaveValue(/^•+$/);
            const reveal = main.getByRole('button', { name: 'Show', exact: true }).first();
            const [control, field] = await Promise.all([
                reveal.boundingBox(),
                secretBox.boundingBox(),
            ]);
            expect(!!control && !!field && control.y + control.height <= field.y).toBe(true);
            await reveal.click();
            const secret = await secretBox.inputValue();
            expect(secret).toMatch(new RegExp(`^${name}\\$.{20,}$`));
            await expect(main.getByText('API-Key <api_key>')).toBeVisible();

            // copy to clipboard
            await main.getByRole('button', { name: 'Copy value to clipboard' }).first().click();
            expect(await page.evaluate(() => navigator.clipboard.readText())).toBe(secret);

            // the curl examples contain the secret and the test URL
            const curl = main.getByRole('textbox', { name: 'curl text command simple' });
            await expect(curl).toBeVisible();
            await main.getByRole('button', { name: 'Copy value to clipboard' }).last().click();
            const curlCmd = await page.evaluate(() => navigator.clipboard.readText());
            expect(curlCmd).toContain(`Authorization: API-Key ${secret}`);
            expect(curlCmd).toContain(`/auth/v1/api_keys/${name}/test`);
            await expect(
                main.getByRole('textbox', { name: 'curl text command with jq' }),
            ).toBeVisible();

            // use the key: the test endpoint and an allowed endpoint work ...
            const auth = { Authorization: `API-Key ${secret}` };
            const testRes = await anon.get(`/auth/v1/api_keys/${name}/test`, { headers: auth });
            expect(testRes.status()).toBe(200);
            expect((await testRes.json()).name).toBe(name);
            const rolesRes = await anon.get('/auth/v1/roles', { headers: auth });
            expect(rolesRes.status()).toBe(200);
            expect(Array.isArray(await rolesRes.json())).toBe(true);
            // ... a forbidden one does not
            const usersRes = await anon.get('/auth/v1/users', { headers: auth });
            expect([401, 403]).toContain(usersRes.status());
            const createRes = await anon.post('/auth/v1/roles', {
                headers: auth,
                data: { role: unique('e2e-forbidden') },
            });
            expect([401, 403]).toContain(createRes.status());

            // the secret is shown only once
            await tab(page, 'Config').click();
            await tab(page, 'Secret').click();
            await expect(secretBox).toHaveCount(0);

            // generating a new one invalidates the old secret
            await main.getByRole('button', { name: 'Generate' }).click();
            await expect(secretBox).toBeVisible();
            await reveal.click();
            const secret2 = await secretBox.inputValue();
            expect(secret2).not.toBe(secret);
            const oldRes = await anon.get(`/auth/v1/api_keys/${name}/test`, { headers: auth });
            expect(oldRes.status()).toBe(401);
            const newRes = await anon.get(`/auth/v1/api_keys/${name}/test`, {
                headers: { Authorization: `API-Key ${secret2}` },
            });
            expect(newRes.status()).toBe(200);
        } finally {
            await anon.dispose();
            await deleteQuietly(page, `api_keys/${name}`);
        }
    });
});

test.describe('delete API key', () => {
    test('header action asks first, cancelling keeps the entry, confirming deletes it', async ({
        page,
        playwright,
        baseURL,
    }) => {
        await page.goto(PAGE);
        const name = keyName();
        await api(page, 'POST', 'api_keys', {
            name,
            access: [{ group: 'Roles', access_rights: ['read'] }],
        });
        const secret = await api(page, 'PUT', `api_keys/${name}/secret`);
        const anon = await playwright.request.newContext({
            baseURL,
            // no admin session: only the API key may authorize these requests
            storageState: { cookies: [], origins: [] },
        });
        try {
            await page.goto(`${PAGE}?kn=${name}`);
            await openEntityDelete(page);
            await expect(
                page
                    .getByRole('main')
                    .getByText('Are you sure, that you want to delete this API Key?'),
            ).toBeVisible();

            await page
                .getByRole('dialog', { name: 'Delete' })
                .getByRole('button', { name: 'Cancel' })
                .click();
            await expect(matrixBox(page.getByRole('main'), 'Roles', 'read')).toBeChecked();
            expect(await keyByName(page, name)).toBeTruthy();

            await openEntityDelete(page);
            await deleteAction(page).click();
            await expect.poll(async () => await keyByName(page, name)).toBeUndefined();
            await expect(entry(page, name)).toHaveCount(0);

            const res = await anon.get(`/auth/v1/api_keys/${name}/test`, {
                headers: { Authorization: `API-Key ${secret}` },
            });
            expect(res.status()).toBe(401);
        } finally {
            await anon.dispose();
            await deleteQuietly(page, `api_keys/${name}`);
        }
    });
});

test.describe('API keys on a phone', () => {
    test('create, open and delete a key @mobile', async ({ page }) => {
        const name = keyName('e2ekm');
        await page.goto(PAGE);
        try {
            const dialog = await openAddNew(page);
            await dialog.getByRole('textbox', { name: 'Key Name' }).fill(name);
            await dialog.getByRole('button', { name: 'Toggle: Roles' }).click();
            await dialog.getByRole('button', { name: 'Save' }).click();
            await expect(dialog).toBeHidden();
            await expect
                .poll(async () => accessMap((await keyByName(page, name))?.access))
                .toEqual({ Roles: [...RIGHTS].sort() });

            await openEntry(page, name);
            await expect(matrixBox(page.getByRole('main'), 'Roles', 'delete')).toBeChecked();
            await openEntityDelete(page);
            await deleteAction(page).click();
            await expect.poll(async () => await keyByName(page, name)).toBeUndefined();
        } finally {
            await deleteQuietly(page, `api_keys/${name}`);
        }
    });
});
