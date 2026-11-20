// Characterization tests for the admin Clients section: list, search, order, Add New, every
// field of the settings tabs, the Login tab (incl. a real client_credentials login with the shown
// secret), Branding (colors, preview, logo / favicon upload) and Delete.
import { test, expect } from './fixtures.mjs';
import { api } from './helpers/api.mjs';
import { unique } from './helpers/env.mjs';
import { chooseOption } from './helpers/ui.mjs';
import {
    countRequests,
    createClient,
    deleteClientQuietly,
    entries,
    entry,
    entryKeys,
    expectInvalid,
    expectTabSelected,
    makePng,
    openAddNew,
    openSubNav,
    saveAndWait,
    saveButton,
    subNav,
    tab,
    tagInput,
    expectEntryCurrent,
} from './helpers/clients.mjs';

const CLIENTS = '/auth/v1/admin/clients';
const detailUrl = (/** @type {string} */ id) => `${CLIENTS}?cid=${encodeURIComponent(id)}`;

/**
 * Opens a client by deep link and waits until its detail view is rendered.
 * @param {import('@playwright/test').Page} page
 * @param {string} id
 */
async function openClient(page, id) {
    await page.goto(detailUrl(id));
    await expect(page.getByRole('textbox', { name: 'Client Name', exact: true })).toBeVisible();
    await expect(page.getByRole('main').getByText(id, { exact: true }).first()).toBeVisible();
}

/** The client checkbox with the given accessible name. */
const checkbox = (
    /** @type {import('@playwright/test').Page} */ page,
    /** @type {string} */ name,
) => page.getByRole('main').getByRole('checkbox', { name, exact: true });

/**
 * Requests a client_credentials token directly from the token endpoint.
 * @param {import('@playwright/test').APIRequestContext} request
 * @param {string} id
 * @param {string} secret
 */
async function clientCredentials(request, id, secret) {
    return request.post('/auth/v1/oidc/token', {
        form: { grant_type: 'client_credentials', client_id: id, client_secret: secret },
    });
}

test.describe('clients list', () => {
    test('loads all clients with the count of the API', async ({ page }) => {
        await page.goto(CLIENTS);
        const all = await api(page, 'GET', 'clients');
        await expect(entries(page)).toHaveCount(all.length);
        for (const id of ['rauthy', 'init_client']) {
            await expect(entry(page, id)).toBeVisible();
        }
        // the "rauthy" entry shows its name as a secondary line
        await expect(entry(page, 'rauthy')).toContainText('Rauthy');
        // nothing is selected yet, so there is no detail view
        await expect(tab(page, 'General')).toHaveCount(0);
    });

    test('search filters by ID and the clear button resets it', async ({ page }) => {
        const id = unique('e2e-search');
        await page.goto(CLIENTS);
        await createClient(page, id);
        try {
            await page.reload();
            const total = (await api(page, 'GET', 'clients')).length;
            await expect(entries(page)).toHaveCount(total);
            await expect(
                subNav(page).getByRole('combobox', { name: 'Search Options' }),
            ).toContainText('ID');

            const search = subNav(page).getByRole('searchbox', { name: 'Search' });
            // case-insensitive substring match on the ID
            await search.fill(id.slice(4).toUpperCase());
            await expect(entries(page)).toHaveCount(1);
            await expect(entry(page, id)).toBeVisible();

            await search.fill('no-client-has-this-id-xyz');
            await expect(entries(page)).toHaveCount(0);

            // the icon-only clear button of the search field is labelled "Delete" (FINDING:
            // "Clear search" would describe it better)
            await subNav(page)
                .getByRole('search')
                .getByRole('button', { name: /delete|clear/i })
                .click();
            await expect(search).toHaveValue('');
            await expect(entries(page)).toHaveCount(total);
        } finally {
            await deleteClientQuietly(page, id);
        }
    });

    test('orders by ID ascending by default and toggles descending / ascending', async ({
        page,
    }) => {
        await page.goto(CLIENTS);
        await expect(entry(page, 'rauthy')).toBeVisible();
        await expect(subNav(page).getByRole('combobox', { name: 'Order by ...' })).toContainText(
            'ID',
        );
        const asc = await entryKeys(page);
        expect(asc.length).toBeGreaterThan(1);
        expect(asc).toEqual([...asc].sort((a, b) => a.localeCompare(b)));

        await subNav(page).getByRole('button', { name: 'Change sort to descending' }).click();
        await expect
            .poll(() => entryKeys(page))
            .toEqual([...asc].sort((a, b) => b.localeCompare(a)));

        await subNav(page).getByRole('button', { name: 'Change sort to ascending' }).click();
        await expect.poll(() => entryKeys(page)).toEqual(asc);
    });

    test('the list has no pagination: every client is rendered at once', async ({ page }) => {
        await page.goto(CLIENTS);
        const all = await api(page, 'GET', 'clients');
        await expect(entries(page)).toHaveCount(all.length);
        await expect(
            page.getByRole('main').getByRole('button', { name: /next page/i }),
        ).toHaveCount(0);
    });

    test('selecting an entry opens the details and sets the deep link', async ({ page }) => {
        await page.goto(CLIENTS);
        await entry(page, 'init_client').click();
        await expect(page).toHaveURL(/[?&]cid=init_client(&|$)/);
        await expectEntryCurrent(entry(page, 'init_client'));
        await expect(page.getByRole('textbox', { name: 'Client Name', exact: true })).toHaveValue(
            'Init Client',
        );
        for (const name of ['General', 'Login', 'Tokens', 'Integrations', 'Branding']) {
            await expect(tab(page, name)).toBeVisible();
        }
        await expectTabSelected(tab(page, 'General'));

        // switching to another entry updates URL and details
        await entry(page, 'rauthy').click();
        await expect(page).toHaveURL(/[?&]cid=rauthy(&|$)/);
        await expect(page.getByRole('textbox', { name: 'Client Name', exact: true })).toHaveValue(
            'Rauthy',
        );

        // deep link straight into a client
        await page.goto(detailUrl('init_client'));
        await expect(page.getByRole('textbox', { name: 'Client Name', exact: true })).toHaveValue(
            'Init Client',
        );
        // browser history navigates between selections
        await page.goBack();
        await expect(page).toHaveURL(/[?&]cid=rauthy(&|$)/);
    });

    test('a deep link to an unknown client shows no details', async ({ page }) => {
        await page.goto(detailUrl('does-not-exist-e2e'));
        await expect(entry(page, 'rauthy')).toBeVisible();
        await expect(page.getByRole('textbox', { name: 'Client Name', exact: true })).toHaveCount(
            0,
        );
    });

    test('the rauthy client protects its backchannel logout and SCIM settings', async ({
        page,
    }) => {
        await openClient(page, 'rauthy');
        await tab(page, 'Integrations').click();
        await expect(
            page.getByRole('textbox', { name: 'Backchannel Logout URI', exact: true }),
        ).toBeDisabled();
        await expect(checkbox(page, 'Enable SCIMv2')).toBeDisabled();
    });
});

test.describe('add new client', () => {
    /** @type {string[]} */
    let created = [];
    test.afterEach(async ({ page }) => {
        for (const id of created) await deleteClientQuietly(page, id);
        created = [];
    });

    test('dialog opens with focus on the ID and can be cancelled', async ({ page }) => {
        await page.goto(CLIENTS);
        const before = (await api(page, 'GET', 'clients')).length;
        const dialog = await openAddNew(page);
        const idInput = dialog.getByRole('textbox', { name: 'Client ID' });
        await expect(idInput).toBeFocused();
        await expect(dialog.getByRole('textbox', { name: 'Client Name' })).toBeVisible();
        // a new client is confidential by default
        await expect(dialog.getByRole('checkbox', { name: 'Confidential' })).toBeChecked();
        await expect(
            dialog.getByRole('textbox', { name: 'Redirect URIs', exact: true }),
        ).toBeVisible();
        await expect(
            dialog.getByRole('textbox', { name: 'Post Logout Redirect URIs' }),
        ).toBeVisible();

        await idInput.fill(unique('e2e-cancel'));
        await dialog.getByRole('button', { name: 'Close' }).first().click();
        await expect(dialog).toBeHidden();
        expect((await api(page, 'GET', 'clients')).length).toBe(before);

        // Escape closes it as well
        await openAddNew(page);
        await page.keyboard.press('Escape');
        await expect(page.getByRole('dialog')).toBeHidden();
    });

    test('validates the ID and blocks saving invalid input', async ({ page }) => {
        await page.goto(CLIENTS);
        const dialog = await openAddNew(page);
        const idInput = dialog.getByRole('textbox', { name: 'Client ID' });
        const save = dialog.getByRole('button', { name: 'Save' });

        // empty (required)
        let sent = await countRequests(page, 'POST', '/auth/v1/clients', () => save.click());
        expect(sent).toBe(0);
        await expectInvalid(idInput);

        // spaces and URI characters are not allowed in a client ID
        for (const bad of ['a', 'has space', 'https://x.example']) {
            await idInput.fill(bad);
            sent = await countRequests(page, 'POST', '/auth/v1/clients', () => save.click());
            expect(sent, bad).toBe(0);
            await expectInvalid(idInput);
            await expect(dialog.getByText('Invalid Input')).toBeVisible();
        }

        // an existing ID is rejected by the UI before any request
        await idInput.fill('rauthy');
        sent = await countRequests(page, 'POST', '/auth/v1/clients', () => save.click());
        expect(sent).toBe(0);
        await expect(dialog.getByText('Name exists already')).toBeVisible();
    });

    test('validates redirect URIs and lets values be removed', async ({ page }) => {
        await page.goto(CLIENTS);
        const dialog = await openAddNew(page);
        const redirect = tagInput(dialog, 'Redirect URIs');

        // a fragment is not allowed in a redirect URI (RFC 6749 3.1.2): stays in the input
        await redirect.add('https://app.example.com/cb#frag');
        await expectInvalid(redirect.input);
        await expect(redirect.list.getByRole('listitem')).toHaveCount(1);

        // reserved query keys are rejected as well
        await redirect.add('https://app.example.com/cb?code=1');
        await expectInvalid(redirect.input);
        await expect(redirect.list.getByRole('listitem')).toHaveCount(1);

        // degenerate URIs do not match the URI pattern
        await redirect.add('javascript:alert(1)');
        await expectInvalid(redirect.input);
        await expect(redirect.list.getByRole('listitem')).toHaveCount(1);

        // valid values become tags (the input itself is the last list item)
        await redirect.add('https://app.example.com/cb');
        await expect(redirect.item('https://app.example.com/cb')).toBeVisible();
        await redirect.add('http://localhost:3000/*');
        await expect(redirect.list.getByRole('listitem')).toHaveCount(3);

        await redirect.remove('http://localhost:3000/*');
        await expect(redirect.item('http://localhost:3000/*')).toHaveCount(0);
        await expect(redirect.list.getByRole('listitem')).toHaveCount(2);

        // `state` is reserved for a post logout redirect URI
        const postLogout = tagInput(dialog, 'Post Logout Redirect URIs');
        await postLogout.add('https://app.example.com/logout?state=1');
        await expectInvalid(postLogout.input);
        await expect(postLogout.list.getByRole('listitem')).toHaveCount(1);
    });

    test('creates a confidential client that can log in with client_credentials', async ({
        page,
        request,
    }) => {
        const id = unique('e2e-conf');
        created.push(id);
        await page.goto(CLIENTS);
        const dialog = await openAddNew(page);
        await dialog.getByRole('textbox', { name: 'Client ID' }).fill(id);
        await dialog.getByRole('textbox', { name: 'Client Name' }).fill('E2E Confidential');
        await tagInput(dialog, 'Redirect URIs').add('https://conf.example.com/callback');
        await tagInput(dialog, 'Post Logout Redirect URIs').add('https://conf.example.com/bye');
        await saveAndWait(page, dialog.getByRole('button', { name: 'Save' }), 'POST', '/clients');

        // the dialog closes, the new client is selected and deep linked
        await expect(dialog).toBeHidden();
        await expect(page).toHaveURL(new RegExp(`[?&]cid=${id}(&|$)`));
        await expectEntryCurrent(entry(page, id));
        await expect(page.getByRole('textbox', { name: 'Client Name', exact: true })).toHaveValue(
            'E2E Confidential',
        );

        const c = await api(page, 'GET', `clients/${id}`);
        expect(c).toMatchObject({
            id,
            name: 'E2E Confidential',
            confidential: true,
            enabled: true,
            redirect_uris: ['https://conf.example.com/callback'],
            post_logout_redirect_uris: ['https://conf.example.com/bye'],
            flows_enabled: ['authorization_code'],
        });

        // enable the client_credentials flow in the UI
        await tab(page, 'Login').click();
        await checkbox(page, 'client_credentials').check();
        await saveAndWait(page, saveButton(page), 'PUT', `/clients/${id}`);
        expect((await api(page, 'GET', `clients/${id}`)).flows_enabled).toContain(
            'client_credentials',
        );

        // read the shown secret and use it for a real token request
        const secretInput = page.getByLabel('Client Secret', { exact: true });
        await expect(secretInput).not.toHaveValue('');
        const secret = await secretInput.inputValue();
        expect(secret).toMatch(/^[a-zA-Z0-9]{32,}$/);

        const res = await clientCredentials(request, id, secret);
        expect(res.status()).toBe(200);
        const token = await res.json();
        expect(token.token_type).toMatch(/bearer|dpop/i);
        expect(token.access_token).toBeTruthy();

        // a wrong secret is rejected
        const bad = await clientCredentials(request, id, `${secret.slice(0, -1)}X`);
        expect(bad.status()).toBeGreaterThanOrEqual(400);
    });

    test('creates a public client without secret', async ({ page }) => {
        const id = unique('e2e-public');
        created.push(id);
        await page.goto(CLIENTS);
        const dialog = await openAddNew(page);
        await dialog.getByRole('textbox', { name: 'Client ID' }).fill(id);
        await dialog.getByRole('checkbox', { name: 'Confidential' }).uncheck();
        await tagInput(dialog, 'Redirect URIs').add('http://localhost:5173/*');
        await saveAndWait(page, dialog.getByRole('button', { name: 'Save' }), 'POST', '/clients');
        await expect(page).toHaveURL(new RegExp(`[?&]cid=${id}(&|$)`));

        const c = await api(page, 'GET', `clients/${id}`);
        expect(c.confidential).toBe(false);
        expect(c.redirect_uris).toEqual(['http://localhost:5173/*']);
        // a public client gets S256 PKCE enforced by default
        expect(c.challenges).toEqual(['S256']);

        await tab(page, 'Login').click();
        await expect(checkbox(page, 'Confidential')).not.toBeChecked();
        await expect(checkbox(page, 'PKCE S256')).toBeChecked();
        await expect(
            page.getByText('This is a non-confidential client and therefore has not secret.'),
        ).toBeVisible();
        await expect(page.getByRole('button', { name: 'Generate New Secret' })).toHaveCount(0);
    });
});

test.describe('client config', () => {
    /** @type {string} */
    let id;
    test.beforeEach(async ({ page }) => {
        id = unique('e2e-cfg');
        await page.goto(CLIENTS);
        await createClient(page, id);
        await openClient(page, id);
    });
    test.afterEach(async ({ page }) => {
        await deleteClientQuietly(page, id);
    });

    const save = async (/** @type {import('@playwright/test').Page} */ page) => {
        await saveAndWait(page, saveButton(page), 'PUT', `/clients/${id}`);
        await expect(page.getByText('Invalid Input')).toHaveCount(0);
    };

    test('information: ID copy, name, URI and contacts', async ({ page, context }) => {
        await context.grantPermissions(['clipboard-read', 'clipboard-write']);
        const main = page.getByRole('main');
        await expect(main.getByRole('heading', { name: 'Information' })).toBeVisible();

        // the ID is read-only and has an icon-only copy button without a name (FINDING)
        const copyNamed = main.getByRole('button', { name: /copy/i });
        const copy = (await copyNamed.count())
            ? copyNamed.first()
            : main.getByText(id, { exact: true }).locator('xpath=following::button[1]');
        await copy.click();
        await expect.poll(() => page.evaluate(() => navigator.clipboard.readText())).toBe(id);

        const name = main.getByRole('textbox', { name: 'Client Name', exact: true });
        const uri = main.getByRole('textbox', { name: 'URI', exact: true });
        const contacts = tagInput(main, 'Contact');

        // invalid values are flagged and block the save
        await name.fill('x');
        await expectInvalid(name);
        expect(
            await countRequests(page, 'PUT', `/clients/${id}`, () => saveButton(page).click()),
        ).toBe(0);
        await name.fill('E2E Renamed (Test)');

        await uri.fill('javascript:alert(1)');
        await expectInvalid(uri);
        expect(
            await countRequests(page, 'PUT', `/clients/${id}`, () => saveButton(page).click()),
        ).toBe(0);
        await uri.fill('https://renamed.example.com');

        await contacts.add('not a contact!');
        await expectInvalid(contacts.input);
        await contacts.input.fill('');
        await contacts.add('admin@renamed.example.com');
        await contacts.add('ops@renamed.example.com');
        await contacts.remove('ops@renamed.example.com');
        await expect(contacts.item('admin@renamed.example.com')).toBeVisible();
        await expect(contacts.item('ops@renamed.example.com')).toHaveCount(0);

        await save(page);
        // the list entry shows the new name right away
        await expect(entry(page, id)).toContainText('E2E Renamed (Test)');

        const c = await api(page, 'GET', `clients/${id}`);
        expect(c.name).toBe('E2E Renamed (Test)');
        expect(c.client_uri).toBe('https://renamed.example.com');
        expect(c.contacts).toEqual(['admin@renamed.example.com']);

        await page.reload();
        await expect(name).toHaveValue('E2E Renamed (Test)');
        await expect(uri).toHaveValue('https://renamed.example.com');
        await expect(contacts.item('admin@renamed.example.com')).toBeVisible();

        // clearing optional values removes them again
        await name.fill('');
        await uri.fill('');
        await contacts.remove('admin@renamed.example.com');
        await save(page);
        const c2 = await api(page, 'GET', `clients/${id}`);
        expect(c2.name ?? null).toBeNull();
        expect(c2.client_uri ?? null).toBeNull();
        expect(c2.contacts ?? null).toBeNull();
    });

    test('configuration: enabled, confidential, force MFA and group prefix', async ({ page }) => {
        await expect(page.getByRole('heading', { name: 'Client Configuration' })).toBeVisible();
        const prefix = page.getByRole('textbox', { name: 'Login Group Prefix', exact: true });
        await expect(checkbox(page, 'Enabled')).toBeChecked();
        await expect(checkbox(page, 'Force MFA')).not.toBeChecked();

        await checkbox(page, 'Enabled').uncheck();
        await checkbox(page, 'Force MFA').check();

        // `password` together with `Force MFA` shows a warning
        await tab(page, 'Login').click();
        const warn = page.getByText(/CAUTION: 'Force MFA' and the 'password' flow/);
        await expect(warn).toHaveCount(0);
        await checkbox(page, 'password').check();
        await expect(warn).toBeVisible();
        await checkbox(page, 'password').uncheck();
        await expect(warn).toHaveCount(0);

        await tab(page, 'General').click();
        await prefix.fill('x');
        await expectInvalid(prefix);
        await prefix.fill('e2e:group');
        await save(page);

        let c = await api(page, 'GET', `clients/${id}`);
        expect(c.enabled).toBe(false);
        expect(c.force_mfa).toBe(true);
        expect(c.restrict_group_prefix).toBe('e2e:group');

        await page.reload();
        await expect(checkbox(page, 'Enabled')).not.toBeChecked();
        await expect(checkbox(page, 'Force MFA')).toBeChecked();
        await expect(prefix).toHaveValue('e2e:group');

        // a public client needs PKCE: without any challenge an error is shown
        await tab(page, 'Login').click();
        await expect(checkbox(page, 'Confidential')).toBeChecked();
        await checkbox(page, 'Confidential').uncheck();
        await checkbox(page, 'PKCE S256').uncheck();
        const pkceErr = page.getByText(
            'The client must either be confidential or have at least one PKCE challenge activated.',
        );
        await expect(pkceErr).toBeVisible();
        await checkbox(page, 'PKCE S256').check();
        await expect(pkceErr).toHaveCount(0);

        await tab(page, 'General').click();
        await checkbox(page, 'Enabled').check();
        await checkbox(page, 'Force MFA').uncheck();
        await prefix.fill('');
        await save(page);
        c = await api(page, 'GET', `clients/${id}`);
        expect(c.confidential).toBe(false);
        expect(c.enabled).toBe(true);
        expect(c.force_mfa).toBe(false);
        expect(c.restrict_group_prefix ?? null).toBeNull();
    });

    test('authentication flows and PKCE challenges', async ({ page }) => {
        await tab(page, 'Login').click();
        const flows = {
            authorization_code: 'authorization_code',
            'urn:ietf:params:oauth:grant-type:device_code':
                'urn:ietf:params:oauth:grant-type:device_code',
            client_credentials: 'client_credentials',
            password: 'password',
            refresh_token: 'refresh_token',
            'urn:ietf:params:oauth:grant-type:token-exchange':
                'urn:ietf:params:oauth:grant-type:token-exchange',
        };
        for (const name of Object.keys(flows)) await checkbox(page, name).check();
        await checkbox(page, 'PKCE plain').check();
        await save(page);

        let c = await api(page, 'GET', `clients/${id}`);
        expect([...c.flows_enabled].sort()).toEqual(Object.values(flows).sort());
        expect([...c.challenges].sort()).toEqual(['S256', 'plain']);

        await page.reload();
        await tab(page, 'Login').click();
        for (const name of Object.keys(flows)) await expect(checkbox(page, name)).toBeChecked();
        await expect(checkbox(page, 'PKCE plain')).toBeChecked();

        for (const name of Object.keys(flows)) {
            if (name !== 'client_credentials') await checkbox(page, name).uncheck();
        }
        await checkbox(page, 'PKCE plain').uncheck();
        await checkbox(page, 'PKCE S256').uncheck();
        await save(page);
        c = await api(page, 'GET', `clients/${id}`);
        expect(c.flows_enabled).toEqual(['client_credentials']);
        expect(c.challenges ?? null).toBeNull();
    });

    test('saves upstream Login settings together with edits on another tab', async ({ page }) => {
        await tab(page, 'Login').click();
        const tokenExchange = 'urn:ietf:params:oauth:grant-type:token-exchange';
        await checkbox(page, tokenExchange).check();
        const actor = checkbox(page, 'Actor token from this client only');
        await actor.check();

        const consent = page.getByRole('combobox', { name: 'Consent Source' });
        await chooseOption(page, consent, 'URL');
        const source = page.getByRole('textbox', { name: 'Consent Source' });
        await source.fill('https://consent.example.com/content');

        await tab(page, 'General').click();
        await page
            .getByRole('textbox', { name: 'Client Name', exact: true })
            .fill('Cross-tab edit');
        await save(page);

        expect(await api(page, 'GET', `clients/${id}`)).toMatchObject({
            name: 'Cross-tab edit',
            actor_same_client: true,
            consent: 'https://consent.example.com/content',
        });

        await page.reload();
        await tab(page, 'Login').click();
        await expect(actor).toBeChecked();
        await expect(source).toHaveValue('https://consent.example.com/content');
    });

    test('origins, redirect and post logout URIs, resources and audiences', async ({ page }) => {
        const main = page.getByRole('main');
        const origins = tagInput(main, 'Allowed Origins');
        const redirect = tagInput(main, 'Redirect URIs');
        const postLogout = tagInput(main, 'Post Logout Redirect URIs');
        const resources = tagInput(main, 'Allowed Resources');
        const aud = tagInput(main, 'Default Audiences');

        await tab(page, 'Login').click();
        await origins.add('https://spa.example.com/path');
        await expectInvalid(origins.input);
        await origins.input.fill('');
        await origins.add('https://spa.example.com');
        await origins.add('http://localhost:5173');

        await expect(redirect.item('http://localhost:9999/callback')).toBeVisible();
        await redirect.add('https://app.example.com/cb#x');
        await expectInvalid(redirect.input);
        await redirect.input.fill('');
        await redirect.add('https://app.example.com/cb');
        await redirect.remove('http://localhost:9999/callback');

        await postLogout.add('https://app.example.com/logout');

        await tab(page, 'Tokens').click();
        await resources.add('https://api.example.com#x');
        await expectInvalid(resources.input);
        await resources.input.fill('');
        await resources.add('https://api.example.com');
        await aud.add('https://aud.example.com');

        await save(page);
        const c = await api(page, 'GET', `clients/${id}`);
        expect(c.allowed_origins).toEqual(['https://spa.example.com', 'http://localhost:5173']);
        expect(c.redirect_uris).toEqual(['https://app.example.com/cb']);
        expect(c.post_logout_redirect_uris).toEqual(['https://app.example.com/logout']);
        expect(c.allowed_resources).toEqual(['https://api.example.com']);
        expect(c.default_aud).toEqual(['https://aud.example.com']);

        await page.reload();
        await tab(page, 'Login').click();
        await expect(origins.item('https://spa.example.com')).toBeVisible();
        await expect(redirect.item('https://app.example.com/cb')).toBeVisible();
        await expect(postLogout.item('https://app.example.com/logout')).toBeVisible();
        await origins.remove('https://spa.example.com');
        await origins.remove('http://localhost:5173');
        await postLogout.remove('https://app.example.com/logout');
        await tab(page, 'Tokens').click();
        await expect(resources.item('https://api.example.com')).toBeVisible();
        await expect(aud.item('https://aud.example.com')).toBeVisible();
        await resources.remove('https://api.example.com');
        await aud.remove('https://aud.example.com');
        await save(page);
        const c2 = await api(page, 'GET', `clients/${id}`);
        expect(c2.allowed_origins ?? null).toBeNull();
        expect(c2.post_logout_redirect_uris ?? null).toBeNull();
        expect(c2.allowed_resources ?? null).toBeNull();
        expect(c2.default_aud ?? null).toBeNull();
    });

    test('a redirect URI is required while authorization_code is enabled', async ({ page }) => {
        await tab(page, 'Login').click();
        const redirect = tagInput(page.getByRole('main'), 'Redirect URIs');
        await redirect.remove('http://localhost:9999/callback');
        await tab(page, 'General').click();
        const sent = await countRequests(page, 'PUT', `/clients/${id}`, () =>
            saveButton(page).click(),
        );
        expect(sent).toBe(0);
        await expectTabSelected(tab(page, 'Login'));
        await expectInvalid(redirect.input);
        expect((await api(page, 'GET', `clients/${id}`)).redirect_uris).toEqual([
            'http://localhost:9999/callback',
        ]);

        // without authorization_code the client can be saved without a redirect URI
        await checkbox(page, 'authorization_code').uncheck();
        await checkbox(page, 'client_credentials').check();
        await save(page);
        expect((await api(page, 'GET', `clients/${id}`)).redirect_uris ?? []).toEqual([]);
    });

    test('allowed and default scopes are edited in a dialog', async ({ page }) => {
        await tab(page, 'Tokens').click();
        const main = page.getByRole('main');
        // the two scope lists only expose an icon button named by an inner `title` (FINDING:
        // both are "Edit" without saying which list they edit)
        const editButtons = main.getByRole('button', { name: 'Edit', exact: true });
        await expect(editButtons).toHaveCount(2);
        await expect(main.getByText('email, openid, profile, groups')).toBeVisible();

        await editButtons.nth(0).click();
        let dialog = page.getByRole('dialog');
        await expect(dialog.getByRole('heading', { name: 'Allowed Scopes' })).toBeVisible();
        await expect(dialog.getByRole('checkbox', { name: 'openid', exact: true })).toBeChecked();
        await dialog.getByRole('checkbox', { name: 'address', exact: true }).check();
        await dialog.getByRole('checkbox', { name: 'groups', exact: true }).uncheck();
        await dialog.getByRole('button', { name: 'Close' }).last().click();
        await expect(dialog).toBeHidden();
        await expect(main.getByText('address', { exact: false }).first()).toBeVisible();

        await editButtons.nth(1).click();
        dialog = page.getByRole('dialog');
        await expect(dialog.getByRole('heading', { name: 'Default Scopes' })).toBeVisible();
        await dialog.getByRole('checkbox', { name: 'email', exact: true }).check();
        await dialog.getByRole('button', { name: 'Close' }).last().click();
        await expect(dialog).toBeHidden();

        await save(page);
        const c = await api(page, 'GET', `clients/${id}`);
        expect([...c.scopes].sort()).toEqual(['address', 'email', 'openid', 'profile']);
        expect([...c.default_scopes].sort()).toEqual(['email', 'openid']);

        await page.reload();
        await tab(page, 'Tokens').click();
        await main.getByRole('button', { name: 'Edit', exact: true }).nth(0).click();
        dialog = page.getByRole('dialog');
        await expect(dialog.getByRole('checkbox', { name: 'address', exact: true })).toBeChecked();
        await expect(
            dialog.getByRole('checkbox', { name: 'groups', exact: true }),
        ).not.toBeChecked();
        await page.keyboard.press('Escape');
        await expect(dialog).toBeHidden();
    });

    test('token lifetimes are validated and algorithms are selectable', async ({ page }) => {
        await tab(page, 'Tokens').click();
        const main = page.getByRole('main');
        const tokenLifetime = main.getByRole('spinbutton', { name: 'Token Lifetime' });
        const authCode = main.getByRole('spinbutton', { name: 'Auth Code Lifetime' });
        await expect(tokenLifetime).toHaveValue('1800');
        await expect(authCode).toHaveValue('60');

        for (const [input, bad, msg] of /** @type {const} */ ([
            [tokenLifetime, '5', '10 <= Token Lifetime <= 86400'],
            [tokenLifetime, '86401', '10 <= Token Lifetime <= 86400'],
            [authCode, '301', '10 <= Auth Code Lifetime <= 300'],
        ])) {
            const good = await input.inputValue();
            await input.fill(bad);
            await expectInvalid(input);
            await expect(main.getByText(msg).first()).toBeVisible();
            expect(
                await countRequests(page, 'PUT', `/clients/${id}`, () => saveButton(page).click()),
            ).toBe(0);
            await input.fill(good);
        }

        await tokenLifetime.fill('900');
        await authCode.fill('30');
        await chooseOption(
            page,
            main.getByRole('combobox', { name: 'Access Token Algorithm' }),
            'RS256',
        );
        await chooseOption(
            page,
            main.getByRole('combobox', { name: 'ID Token Algorithm' }),
            'RS512',
        );
        await save(page);

        const c = await api(page, 'GET', `clients/${id}`);
        expect(c.access_token_lifetime).toBe(900);
        expect(c.auth_code_lifetime).toBe(30);
        expect(c.access_token_alg).toBe('RS256');
        expect(c.id_token_alg).toBe('RS512');

        await page.reload();
        await tab(page, 'Tokens').click();
        await expect(tokenLifetime).toHaveValue('900');
        await expect(authCode).toHaveValue('30');
        await expect(main.getByRole('combobox', { name: 'Access Token Algorithm' })).toContainText(
            'RS256',
        );
        await expect(main.getByRole('combobox', { name: 'ID Token Algorithm' })).toContainText(
            'RS512',
        );
    });

    test('custom claims, claims at root and backchannel logout URI', async ({ page }) => {
        await tab(page, 'Tokens').click();
        const main = page.getByRole('main');
        const claims = main.getByRole('textbox', { name: 'Custom claims (client_credentials)' });
        await claims.fill('{"tenant": "e2e", "level": 3}');

        const rootWarn = main.getByText(/This client's claims are written at the token root/);
        await expect(rootWarn).toHaveCount(0);
        await checkbox(page, 'Emit claims at the token root').check();
        await expect(rootWarn).toBeVisible();
        await expect(main.getByRole('link', { name: 'IANA JWT Claims Registry' })).toBeVisible();

        const bcl = main.getByRole('textbox', { name: 'Backchannel Logout URI', exact: true });
        await tab(page, 'Integrations').click();
        await bcl.fill('https://app.example.com/backchannel logout');
        await expectInvalid(bcl);
        await bcl.fill('https://app.example.com/backchannel');
        await save(page);

        const c = await api(page, 'GET', `clients/${id}`);
        expect(c.claims).toEqual({ tenant: 'e2e', level: 3 });
        expect(c.claims_at_root).toBe(true);
        expect(c.backchannel_logout_uri).toBe('https://app.example.com/backchannel');

        await page.reload();
        await tab(page, 'Tokens').click();
        await expect(claims).toHaveValue(/"tenant": "e2e"/);
        await expect(checkbox(page, 'Emit claims at the token root')).toBeChecked();
        await claims.fill('');
        await checkbox(page, 'Emit claims at the token root').uncheck();
        await tab(page, 'Integrations').click();
        await expect(bcl).toHaveValue('https://app.example.com/backchannel');
        await bcl.fill('');
        await save(page);
        const c2 = await api(page, 'GET', `clients/${id}`);
        expect(c2.claims ?? null).toBeNull();
        expect(c2.claims_at_root).toBe(false);
        expect(c2.backchannel_logout_uri ?? null).toBeNull();
    });

    test('SCIM can be enabled with base URI, token and group sync', async ({ page }) => {
        await tab(page, 'Integrations').click();
        const main = page.getByRole('main');
        const baseUri = main.getByRole('textbox', { name: 'SCIM Base URI' });
        await expect(baseUri).toHaveCount(0);
        await checkbox(page, 'Enable SCIMv2').check();
        await expect(baseUri).toBeVisible();
        const token = main.getByLabel('Bearer Token', { exact: true });
        await expect(checkbox(page, 'Synchronize Groups')).toBeChecked();

        // base URI and token are required once SCIM is enabled
        expect(
            await countRequests(page, 'PUT', `/clients/${id}`, () => saveButton(page).click()),
        ).toBe(0);
        await expectInvalid(baseUri);

        await baseUri.fill('https://scim.example.com/v2');
        await token.fill('e2eScimBearerToken123');
        await main.getByRole('textbox', { name: 'Groups Filter Prefix' }).fill('app:');
        // with group sync on, the backend syncs the groups with the (unreachable) SCIM server
        // during the save and answers 500 on purpose, so the sync is switched off here; the
        // prefix belongs to the group sync and is hidden and dropped with it
        await checkbox(page, 'Synchronize Groups').uncheck();
        await expect(main.getByRole('textbox', { name: 'Groups Filter Prefix' })).toHaveCount(0);
        await save(page);

        const c = await api(page, 'GET', `clients/${id}`);
        expect(c.scim).toMatchObject({
            base_uri: 'https://scim.example.com/v2',
            bearer_token: 'e2eScimBearerToken123',
            sync_groups: false,
        });
        expect(c.scim.group_sync_prefix ?? null).toBeNull();

        await page.reload();
        await tab(page, 'Integrations').click();
        await expect(checkbox(page, 'Enable SCIMv2')).toBeChecked();
        // the stored choice is shown again (upstream always showed "on")
        await expect(checkbox(page, 'Synchronize Groups')).not.toBeChecked();
        await expect(baseUri).toHaveValue('https://scim.example.com/v2');

        await checkbox(page, 'Enable SCIMv2').uncheck();
        await expect(baseUri).toHaveCount(0);
        await save(page);
        expect((await api(page, 'GET', `clients/${id}`)).scim ?? null).toBeNull();
    });
});

test.describe('client secret', () => {
    /** @type {string} */
    let id;
    test.beforeEach(async ({ page }) => {
        id = unique('e2e-secret');
        await page.goto(CLIENTS);
        await createClient(page, id, { flows_enabled: ['client_credentials'] });
        await openClient(page, id);
        await tab(page, 'Login').click();
        await expectTabSelected(tab(page, 'Login'));
    });
    test.afterEach(async ({ page }) => {
        await deleteClientQuietly(page, id);
    });

    test('shows, copies and hides the current secret', async ({ page, context }) => {
        await context.grantPermissions(['clipboard-read', 'clipboard-write']);
        const secretInput = page.getByLabel('Client Secret', { exact: true });
        await expect(secretInput).not.toHaveValue('');
        await expect(secretInput).toBeDisabled();
        const { secret } = await api(page, 'POST', `clients/${id}/secret`);
        await expect(secretInput).toHaveValue(secret);

        // masked by default, "Show" reveals it, "Hide" masks it again
        await expect(secretInput).toHaveAttribute('type', 'password');
        await page.getByRole('button', { name: 'Show', exact: true }).click();
        await expect(secretInput).toHaveAttribute('type', 'text');
        await page.getByRole('button', { name: 'Hide', exact: true }).click();
        await expect(secretInput).toHaveAttribute('type', 'password');

        await page.getByRole('button', { name: 'Copy value to clipboard' }).click();
        await expect.poll(() => page.evaluate(() => navigator.clipboard.readText())).toBe(secret);
    });

    test('rotates the secret without cache: the old one stops working', async ({
        page,
        request,
    }) => {
        const secretInput = page.getByLabel('Client Secret', { exact: true });
        await expect(secretInput).not.toHaveValue('');
        const old = await secretInput.inputValue();
        expect((await clientCredentials(request, id, old)).status()).toBe(200);

        await page.getByRole('button', { name: 'Generate New Secret' }).click();
        await expect(
            page.getByText(/keep the\s+current secret in an in-memory cache/),
        ).toBeVisible();
        await expect(
            page.getByText('Caution: You should not cache the current secret if you had a leak!'),
        ).toBeVisible();
        await expect(page.getByRole('checkbox', { name: 'Client Secret Cache' })).not.toBeChecked();

        await saveAndWait(
            page,
            page.getByRole('button', { name: 'Generate New Secret' }),
            'PUT',
            `/clients/${id}/secret`,
        );
        await expect(secretInput).not.toHaveValue(old);
        const fresh = await secretInput.inputValue();
        expect(fresh).toMatch(/^[a-zA-Z0-9]{32,}$/);
        expect((await api(page, 'POST', `clients/${id}/secret`)).secret).toBe(fresh);

        expect((await clientCredentials(request, id, fresh)).status()).toBe(200);
        expect((await clientCredentials(request, id, old)).status()).toBeGreaterThanOrEqual(400);
    });

    test('rotates the secret and caches the current one for a while', async ({ page, request }) => {
        const secretInput = page.getByLabel('Client Secret', { exact: true });
        await expect(secretInput).not.toHaveValue('');
        const old = await secretInput.inputValue();

        await page.getByRole('button', { name: 'Generate New Secret' }).click();
        await page.getByRole('checkbox', { name: 'Client Secret Cache' }).check();
        const hours = page.getByRole('spinbutton', { name: 'Cache Duration (hours)' });
        await expect(hours).toHaveValue('1');

        // 1..24 hours only: an invalid value disables the button
        const generate = page.getByRole('button', { name: 'Generate New Secret' });
        for (const bad of ['0', '25']) {
            await hours.fill(bad);
            await expectInvalid(hours);
            await expect(generate).toBeDisabled();
        }
        await hours.fill('2');
        await expect(generate).toBeEnabled();

        await saveAndWait(page, generate, 'PUT', `/clients/${id}/secret`);
        await expect(secretInput).not.toHaveValue(old);
        const fresh = await secretInput.inputValue();

        // both the new and the cached old secret are accepted
        expect((await clientCredentials(request, id, fresh)).status()).toBe(200);
        expect((await clientCredentials(request, id, old)).status()).toBe(200);
    });
});

test.describe('client branding', () => {
    // A client without its own favicon answers 404 for it, the preview probes it on purpose.
    test.use({ allowErrors: [/^http 404 GET \/auth\/v1\/clients\/[^/]+\/favicon$/] });

    /** @type {string} */
    let id;
    test.beforeEach(async ({ page }) => {
        id = unique('e2e-brand');
        await page.goto(CLIENTS);
        await createClient(page, id);
        await openClient(page, id);
        await tab(page, 'Branding').click();
        await expect(tab(page, 'Light Theme')).toBeVisible();
    });
    test.afterEach(async ({ page }) => {
        try {
            await api(page, 'DELETE', `theme/${id}`);
        } catch {}
        await deleteClientQuietly(page, id);
    });

    /** The live preview container, labelled by `aria-label` (it has no role, FINDING). */
    const preview = (/** @type {import('@playwright/test').Page} */ page) =>
        page.getByLabel(/^Preview: All components inside/);
    const cssVar = (/** @type {import('@playwright/test').Page} */ page, /** @type {string} */ v) =>
        preview(page).evaluate(
            (el, v) => /** @type {HTMLElement} */ (el).style.getPropertyValue(v),
            v,
        );

    function hsl(/** @type {import("@playwright/test").Page} */ page, /** @type {string} */ name) {
        const group = page.getByRole('group', { name, exact: true });
        return {
            heading: group.getByRole('heading', { name, exact: true }),
            hue: group.getByRole('slider', { name: 'Hue' }),
            sat: group.getByRole('slider', { name: 'Sat' }),
            lum: group.getByRole('slider', { name: 'Lum' }),
            color: group.getByLabel(name, { exact: true }),
        };
    }

    test('border radius is validated and saved', async ({ page }) => {
        const radius = page.getByRole('textbox', { name: '--border-radius' });
        await expect(radius).toHaveValue('5px');
        expect(await cssVar(page, '--border-radius')).toBe('5px');

        await radius.fill('5px; color: red');
        await expectInvalid(radius);
        expect(
            await countRequests(page, 'PUT', `/theme/${id}`, () =>
                page.getByRole('main').getByRole('button', { name: 'Save', exact: true }).click(),
            ),
        ).toBe(0);

        await radius.fill('12px');
        await expect.poll(() => cssVar(page, '--border-radius')).toBe('12px');
        await saveAndWait(
            page,
            page.getByRole('main').getByRole('button', { name: 'Save', exact: true }),
            'PUT',
            `/theme/${id}`,
        );
        expect((await api(page, 'POST', `theme/${id}`)).border_radius).toBe('12px');

        await page.reload();
        await tab(page, 'Branding').click();
        await expect(page.getByRole('textbox', { name: '--border-radius' })).toHaveValue('12px');
    });

    test('HSL sliders and color pickers update the live preview and are saved', async ({
        page,
    }) => {
        const action = hsl(page, '--action');
        await expect(action.heading).toBeVisible();
        await expect(action.hue).toHaveValue('34');
        await expect(action.sat).toHaveValue('100');
        await expect(action.lum).toHaveValue('40');
        await expect(action.color).toHaveValue('#cc7400');
        expect(await cssVar(page, '--action')).toBe('34 100% 40%');

        // keyboard on the sliders
        await action.hue.focus();
        await action.hue.press('ArrowRight');
        await action.hue.press('ArrowRight');
        await expect(action.hue).toHaveValue('36');
        await action.lum.focus();
        await action.lum.press('ArrowLeft');
        await expect(action.lum).toHaveValue('39');
        await expect.poll(() => cssVar(page, '--action')).toBe('36 100% 39%');
        await expect(action.color).not.toHaveValue('#cc7400');

        // the color picker sets all three sliders at once
        const error = hsl(page, '--error');
        await error.color.fill('#ff0000');
        await expect(error.hue).toHaveValue('0');
        await expect(error.sat).toHaveValue('100');
        await expect(error.lum).toHaveValue('50');
        await expect.poll(() => cssVar(page, '--error')).toBe('0 100% 50%');

        // every other color exposes its sliders as well
        for (const mode of ['Light Theme', 'Dark Theme']) {
            await tab(page, mode).click();
            for (const name of ['--text', '--text-high', '--bg', '--bg-high', '--accent']) {
                const c = hsl(page, name);
                await expect(c.hue).toBeVisible();
                await expect(c.sat).toBeVisible();
                await expect(c.lum).toBeVisible();
                await expect(c.color).toHaveValue(/^#[0-9a-f]{6}$/);
            }
        }

        // a dark theme value
        const darkBg = hsl(page, '--bg');
        await darkBg.sat.focus();
        await darkBg.sat.press('ArrowRight');
        await expect(darkBg.sat).toHaveValue('41');

        await saveAndWait(
            page,
            page.getByRole('main').getByRole('button', { name: 'Save', exact: true }),
            'PUT',
            `/theme/${id}`,
        );
        const theme = await api(page, 'POST', `theme/${id}`);
        expect(theme.client_id).toBe(id);
        expect(theme.light.action).toEqual([36, 100, 39]);
        expect(theme.light.error).toEqual([0, 100, 50]);
        expect(theme.dark.bg).toEqual([200, 41, 6]);

        await page.reload();
        await tab(page, 'Branding').click();
        await expect(hsl(page, '--action').hue).toHaveValue('36');
        await tab(page, 'Dark Theme').click();
        await expect(hsl(page, '--bg').sat).toHaveValue('41');
    });

    test('full CSS values are validated and previewed', async ({ page }) => {
        await page.getByText('Advanced CSS', { exact: true }).click();
        const btnText = page.getByRole('textbox', { name: '--btn-text', exact: true }).first();
        const sun = page.getByRole('textbox', { name: '--theme-sun', exact: true }).first();
        const moon = page.getByRole('textbox', { name: '--theme-moon', exact: true }).first();
        await expect(btnText).toHaveValue('white');
        await expect(sun).toHaveValue('hsla(var(--action) / .7)');
        await expect(moon).toHaveValue('hsla(var(--accent) / .85)');

        await btnText.fill('red; background: url(x)');
        await expectInvalid(btnText);
        expect(
            await countRequests(page, 'PUT', `/theme/${id}`, () =>
                page.getByRole('main').getByRole('button', { name: 'Save', exact: true }).click(),
            ),
        ).toBe(0);

        await btnText.fill('#123456');
        await expect.poll(() => cssVar(page, '--btn-text')).toBe('#123456');
        await sun.fill('hsl(var(--error))');
        await saveAndWait(
            page,
            page.getByRole('main').getByRole('button', { name: 'Save', exact: true }),
            'PUT',
            `/theme/${id}`,
        );
        const theme = await api(page, 'POST', `theme/${id}`);
        expect(theme.light.btn_text).toBe('#123456');
        expect(theme.light.theme_sun).toBe('hsl(var(--error))');
    });

    test('preview switches between light and dark theme', async ({ page }) => {
        const light = tab(page, 'Light Theme');
        const dark = tab(page, 'Dark Theme');
        await expectTabSelected(light);
        expect(await cssVar(page, '--bg')).toBe('34 25% 97%');
        await dark.click();
        await expectTabSelected(dark);
        await expect.poll(() => cssVar(page, '--bg')).toBe('200 40% 6%');
        // the preview follows edits of the dark theme
        const darkBg = hsl(page, '--bg');
        await darkBg.lum.focus();
        await darkBg.lum.press('ArrowRight');
        await expect.poll(() => cssVar(page, '--bg')).toBe('200 40% 7%');
        await light.click();
        await expect.poll(() => cssVar(page, '--bg')).toBe('34 25% 97%');
    });

    test('reset restores the default theme', async ({ page }) => {
        const radius = page.getByRole('textbox', { name: '--border-radius' });
        await radius.fill('20px');
        await saveAndWait(
            page,
            page.getByRole('main').getByRole('button', { name: 'Save', exact: true }),
            'PUT',
            `/theme/${id}`,
        );
        expect((await api(page, 'POST', `theme/${id}`)).border_radius).toBe('20px');

        const del = page.waitForResponse(
            r => r.request().method() === 'DELETE' && r.url().endsWith(`/theme/${id}`),
        );
        await page.getByRole('main').getByRole('button', { name: 'Reset theme' }).click();
        await page
            .getByRole('dialog', { name: 'Reset theme' })
            .getByRole('button', { name: 'Reset theme' })
            .click();
        expect((await del).status()).toBeLessThan(400);
        await expect(radius).toHaveValue('5px');
        expect((await api(page, 'POST', `theme/${id}`)).border_radius).toBe('5px');
    });

    test('uploads and removes a client logo and favicon', async ({ page }) => {
        const png = await makePng();
        const main = page.getByRole('main');
        const logoUpload = main.getByRole('form', { name: 'Logo', exact: true });
        const faviconUpload = main.getByRole('form', { name: 'Favicon Upload', exact: true });
        const logoFile = { name: 'logo.png', mimeType: 'image/png', buffer: png };

        // no own logo yet: the preview falls back to the default logo
        await expect(preview(page).getByRole('img', { name: 'Rauthy Logo' })).toBeVisible();

        const up = page.waitForResponse(
            r => r.request().method() === 'PUT' && r.url().includes(`/clients/${id}/logo`),
        );
        await logoUpload.locator('input[type=file]').setInputFiles(logoFile);
        expect((await up).status()).toBeLessThan(400);
        await expect(preview(page).getByRole('img', { name: 'Client Logo' })).toBeVisible();
        const logo = await page.request.get(`/auth/v1/clients/${id}/logo`);
        expect(logo.status()).toBe(200);
        expect(logo.headers()['content-type']).toMatch(/^image\//);

        const upFav = page.waitForResponse(
            r => r.request().method() === 'PUT' && r.url().includes(`/clients/${id}/favicon`),
        );
        await faviconUpload
            .locator('input[type=file]')
            .setInputFiles({ ...logoFile, name: 'favicon.png' });
        expect((await upFav).status()).toBeLessThan(400);
        await expect(main.getByRole('img', { name: 'Client favicon preview' })).toBeVisible();
        expect((await page.request.get(`/auth/v1/clients/${id}/favicon`)).status()).toBe(200);

        await main.getByRole('button', { name: 'Reset theme' }).click();
        await page
            .getByRole('dialog', { name: 'Reset theme' })
            .getByRole('button', { name: 'Reset theme' })
            .click();
        await expect(page.getByRole('dialog', { name: 'Reset theme' })).toBeHidden();
        await expect(preview(page).getByRole('img', { name: 'Client Logo' })).toBeVisible();
        await expect(main.getByRole('img', { name: 'Client favicon preview' })).toBeVisible();

        const delLogo = page.waitForResponse(
            r => r.request().method() === 'DELETE' && r.url().includes(`/clients/${id}/logo`),
        );
        await main.getByRole('button', { name: 'Delete · Logo', exact: true }).click();
        await page
            .getByRole('dialog', { name: 'Delete · Logo' })
            .getByRole('button', { name: 'Delete · Logo' })
            .click();
        expect((await delLogo).status()).toBeLessThan(400);
        await expect(preview(page).getByRole('img', { name: 'Rauthy Logo' })).toBeVisible();

        const delFav = page.waitForResponse(
            r => r.request().method() === 'DELETE' && r.url().includes(`/clients/${id}/favicon`),
        );
        await main.getByRole('button', { name: 'Delete · Favicon Upload', exact: true }).click();
        await page
            .getByRole('dialog', { name: 'Delete · Favicon Upload' })
            .getByRole('button', { name: 'Delete · Favicon Upload' })
            .click();
        expect((await delFav).status()).toBeLessThan(400);
        await expect(main.getByRole('img', { name: 'Client favicon preview' })).toBeHidden();
        expect((await page.request.get(`/auth/v1/clients/${id}/favicon`)).status()).toBe(404);
    });
});

async function confirmClientDeletion(
    /** @type {import('@playwright/test').Page} */ page,
    /** @type {string} */ path,
) {
    await page.locator('.admin-entity-header').getByRole('button', { name: 'Delete' }).click();
    const dialog = page.getByRole('dialog', { name: 'Delete' });
    const deleted = page.waitForResponse(
        r => r.request().method() === 'DELETE' && r.url().endsWith(path),
    );
    await dialog.getByRole('button', { name: 'Delete' }).click();
    return (await deleted).status();
}

test.describe('client delete', () => {
    test('cancelling the header confirmation keeps the client, confirming deletes it', async ({
        page,
    }) => {
        const id = unique('e2e-del');
        await page.goto(CLIENTS);
        await createClient(page, id);
        try {
            await openClient(page, id);
            const action = page
                .locator('.admin-entity-header')
                .getByRole('button', { name: 'Delete' });
            await action.click();
            const dialog = page.getByRole('dialog', { name: 'Delete' });
            await expect(
                dialog.getByText('Are you sure you want to delete this client?'),
            ).toBeVisible();
            await expect(dialog.getByText(id, { exact: true })).toBeVisible();

            const cancel = dialog.getByRole('button', { name: 'Cancel' });
            await expect(cancel).toBeFocused();
            await cancel.click();
            await expect(dialog).toBeHidden();
            await expect(
                page.getByRole('textbox', { name: 'Client Name', exact: true }),
            ).toBeVisible();
            expect((await api(page, 'GET', `clients/${id}`)).id).toBe(id);

            expect(await confirmClientDeletion(page, `/clients/${id}`)).toBeLessThan(400);
            await expect(entry(page, id)).toHaveCount(0);
            await expect(
                page.getByRole('textbox', { name: 'Client Name', exact: true }),
            ).toHaveCount(0);
            await expect(api(page, 'GET', `clients/${id}`)).rejects.toThrow(/-> 404/);
        } finally {
            await deleteClientQuietly(page, id);
        }
    });
});

test.describe('clients at phone size', () => {
    test('list, search and deep link @mobile', async ({ page }) => {
        await page.goto(CLIENTS);
        await openSubNav(page);
        await expect(entry(page, 'rauthy')).toBeVisible();
        await subNav(page).getByRole('searchbox', { name: 'Search' }).fill('init');
        await expect(entries(page)).toHaveCount(1);
        await entry(page, 'init_client').click();
        await expect(page).toHaveURL(/[?&]cid=init_client(&|$)/);
        await expect(page.getByRole('textbox', { name: 'Client Name', exact: true })).toHaveValue(
            'Init Client',
        );
        for (const name of ['General', 'Login', 'Tokens', 'Integrations', 'Branding']) {
            await expect(tab(page, name)).toBeVisible();
        }
    });

    test('create, edit and delete a client @mobile', async ({ page }) => {
        const id = unique('e2e-mob');
        await page.goto(CLIENTS);
        try {
            const dialog = await openAddNew(page);
            await dialog.getByRole('textbox', { name: 'Client ID' }).fill(id);
            await tagInput(dialog, 'Redirect URIs').add('https://mobile.example.com/cb');
            await saveAndWait(
                page,
                dialog.getByRole('button', { name: 'Save' }),
                'POST',
                '/clients',
            );
            await expect(page).toHaveURL(new RegExp(`[?&]cid=${id}(&|$)`));

            const name = page.getByRole('textbox', { name: 'Client Name', exact: true });
            await name.fill('Mobile Client');
            await saveAndWait(page, saveButton(page), 'PUT', `/clients/${id}`);
            expect((await api(page, 'GET', `clients/${id}`)).name).toBe('Mobile Client');

            expect(await confirmClientDeletion(page, `/clients/${id}`)).toBeLessThan(400);
            await expect(api(page, 'GET', `clients/${id}`)).rejects.toThrow(/-> 404/);
        } finally {
            await deleteClientQuietly(page, id);
        }
    });
});
