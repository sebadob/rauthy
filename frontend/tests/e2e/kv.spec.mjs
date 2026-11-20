import { openEntityDelete, deleteAction } from './helpers/entities.mjs';
// KV store: namespaces, values, access keys, editing and deletion.
import { test, expect } from './fixtures.mjs';
import { api } from './helpers/api.mjs';
import { unique } from './helpers/env.mjs';
import { showEntryList } from './helpers/ui.mjs';

/** @type {string[]} */
let namespaces = [];

test.afterEach(async ({ page }) => {
    for (const ns of namespaces) {
        try {
            await api(page, 'DELETE', `kv/ns/${ns}`);
        } catch (e) {
            if (!/-> 404/.test(String(e))) throw e;
        }
    }
    namespaces = [];
});

/** Reaches the KV store the way users do, from the sidebar (see the deep link test below). */
async function openKv(/** @type {import('@playwright/test').Page} */ page) {
    await page.goto('/auth/v1/admin/users');
    await expect(page.getByRole('heading', { name: 'Users', level: 1 })).toBeVisible();
    const link = page.getByRole('link', { name: 'KV Store' });
    if (!(await link.isVisible())) {
        // at phone size the main navigation is a drawer behind the "Administration" button
        await page.getByRole('button', { name: 'Administration' }).click();
    }
    await link.click();
    await expect(page.getByRole('heading', { name: 'KV Store', level: 1 })).toBeVisible();
}

function nsEntry(/** @type {import('@playwright/test').Page} */ page, /** @type {string} */ ns) {
    return page
        .getByRole('main')
        .getByRole('navigation')
        .first()
        .getByRole('button', { name: new RegExp(`^${ns}( \\(public\\))?$`) });
}

/** A detail tab: the first button of `main` with that name shown as text. */
function tab(/** @type {import('@playwright/test').Page} */ page, /** @type {string} */ name) {
    return page
        .getByRole('main')
        .getByRole('button', { name, exact: true })
        .filter({ hasText: name })
        .first();
}

async function newNamespace(/** @type {import('@playwright/test').Page} */ page, data = {}) {
    await page.goto('/auth/v1/admin/users');
    const name = unique('e2e-kv');
    await api(page, 'POST', 'kv/ns', { name, public: false, ...data });
    namespaces.push(name);
    return name;
}

test('the default namespace is selected on open', async ({ page }) => {
    await openKv(page);
    await expect(page).toHaveURL(/kv\?ns=default/);
    await expect(page.getByRole('main').getByRole('heading', { name: 'default' })).toBeVisible();
});

test('a direct link to the KV store opens the KV store', async ({ page }) => {
    await page.goto('/auth/v1/admin/kv');
    await expect(page).toHaveTitle(/KV/, { timeout: 5_000 });
});

test('creates a public namespace from the dialog', async ({ page }) => {
    await openKv(page);
    const name = unique('e2e-kv');
    await page.getByRole('button', { name: 'Add New' }).click();
    const dialog = page.getByRole('dialog');
    await expect(dialog.getByRole('heading', { name: 'New Namespace' })).toBeVisible();
    await dialog.getByRole('textbox', { name: 'Name' }).fill(name);
    await dialog.getByRole('checkbox', { name: 'Public Access' }).check();
    await dialog.getByRole('button', { name: 'Save' }).click();
    namespaces.push(name);

    await expect(dialog).toBeHidden();
    await expect(nsEntry(page, name)).toHaveAccessibleName(`${name} (public)`);
    const all = await api(page, 'GET', 'kv/ns');
    expect(all).toContainEqual(expect.objectContaining({ name, public: true }));
});

test('adds, edits and deletes a value', async ({ page }) => {
    const ns = await newNamespace(page);
    await openKv(page);
    await nsEntry(page, ns).click();
    await expect(page).toHaveURL(new RegExp(`ns=${ns}`));
    const main = page.getByRole('main');
    const limit = main.getByRole('spinbutton', { name: 'Limit' });
    const loadAll = main.getByRole('button', { name: 'Load all values' });
    const newValue = main.getByRole('button', { name: 'New Value' });
    await expect(limit).toBeVisible();
    const bottom = (/** @type {import('@playwright/test').Locator} */ control) =>
        control.evaluate(element => element.getBoundingClientRect().bottom);
    await expect.poll(async () => (await bottom(limit)) - (await bottom(loadAll))).toBe(0);
    await expect.poll(async () => (await bottom(limit)) - (await bottom(newValue))).toBe(0);

    await newValue.click();
    const dialog = page.getByRole('dialog');
    await dialog.getByRole('textbox', { name: 'Key' }).fill('greeting');
    await dialog.getByRole('textbox', { name: 'JSON Value' }).fill('"hello"');
    await dialog.getByRole('button', { name: 'Save' }).click();
    await expect(dialog).toBeHidden();
    await expect(main.getByText('greeting', { exact: true })).toBeVisible();
    await expect
        .poll(async () => JSON.stringify(await api(page, 'GET', `kv/ns/${ns}/values`)))
        .toContain('hello');

    await main.getByRole('button', { name: 'Edit', exact: true }).last().click();
    const edit = page.getByRole('dialog');
    await edit.getByRole('textbox', { name: 'JSON Value' }).fill('{"changed": true}');
    await edit.getByRole('button', { name: 'Save' }).click();
    await expect(edit).toBeHidden();
    await expect
        .poll(async () => JSON.stringify(await api(page, 'GET', `kv/ns/${ns}/values`)))
        .toContain('changed');

    await main.getByRole('button', { name: 'Delete', exact: true }).last().click();
    const confirm = page.getByRole('dialog');
    if (await confirm.isVisible()) {
        await confirm.getByRole('button', { name: 'Delete' }).click();
    }
    await expect(main.getByText('greeting', { exact: true })).toHaveCount(0);
});

test('a value that is not valid JSON is stored as text', async ({ page }) => {
    // `parseJsonValue` falls back to a plain string on purpose
    const ns = await newNamespace(page);
    await openKv(page);
    await nsEntry(page, ns).click();
    await page.getByRole('main').getByRole('button', { name: 'New Value' }).click();
    const dialog = page.getByRole('dialog');
    await dialog.getByRole('textbox', { name: 'Key' }).fill('text');
    await dialog.getByRole('textbox', { name: 'JSON Value' }).fill('{not json');
    await dialog.getByRole('button', { name: 'Save' }).click();
    await expect(dialog).toBeHidden();
    const values = await api(page, 'GET', `kv/ns/${ns}/values`);
    expect(values).toContainEqual(expect.objectContaining({ key: 'text', value: '{not json' }));
});

test('creates an access key for a namespace', async ({ page }) => {
    const ns = await newNamespace(page);
    await openKv(page);
    await nsEntry(page, ns).click();
    await tab(page, 'Access').click();
    await page.getByRole('main').getByRole('button', { name: 'New Access Key' }).click();
    const dialog = page.getByRole('dialog');
    await expect(dialog).toBeVisible();
    await dialog.getByRole('textbox').first().fill('e2e-access');
    await dialog.getByRole('button', { name: 'Save' }).click();
    await expect
        .poll(async () => JSON.stringify(await api(page, 'GET', `kv/ns/${ns}/access`)))
        .toContain('e2e-access');
    await expect(page.getByRole('main').getByText('e2e-access')).toBeVisible();
});

test('edits namespace visibility and deletes the namespace', async ({ page }) => {
    const ns = await newNamespace(page);
    await openKv(page);
    await nsEntry(page, ns).click();
    const main = page.getByRole('main');

    await tab(page, 'Edit').click();
    await main.getByRole('checkbox', { name: 'Public Access' }).check();
    await main.getByRole('button', { name: 'Save', exact: true }).click();
    await expect
        .poll(
            async () =>
                (await api(page, 'GET', 'kv/ns')).find(
                    (/** @type {{ name: string, public: boolean }} */ n) => n.name === ns,
                )?.public,
        )
        .toBe(true);

    await openEntityDelete(page);
    await expect(
        main.getByText('Do you really want to delete this namespace including all existing data?'),
    ).toBeVisible();
    await deleteAction(page).click();
    await expect(nsEntry(page, ns)).toHaveCount(0);
    await expect
        .poll(async () =>
            (await api(page, 'GET', 'kv/ns')).some(
                (/** @type {{ name: string }} */ n) => n.name === ns,
            ),
        )
        .toBe(false);
});

test('namespaces and values are usable at phone size @mobile', async ({ page }) => {
    const ns = await newNamespace(page);
    await openKv(page);
    await showEntryList(page);
    await nsEntry(page, ns).click();
    await expect(page.getByRole('main').getByRole('button', { name: 'New Value' })).toBeVisible();
    const overflow = await page.evaluate(
        () => document.documentElement.scrollWidth - window.innerWidth,
    );
    expect(overflow).toBeLessThanOrEqual(1);
});
