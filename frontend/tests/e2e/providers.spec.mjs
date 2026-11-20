import { openEntityDelete } from './helpers/entities.mjs';
import { test, expect } from './fixtures.mjs';
import { api } from './helpers/api.mjs';
import { unique } from './helpers/env.mjs';
import { makePng } from './helpers/clients.mjs';
import { chooseOption } from './helpers/ui.mjs';
import {
    deleteAction,
    deleteQuietly,
    entry,
    entryList,
    expectCurrent,
    listedNames,
    openAddNew,
    openEntry,
    recordRequests,
    saveButton,
    showSubNav,
} from './helpers/entities.mjs';

const PAGE = '/auth/v1/admin/providers';

// The OIDC / Auto modes let the BACKEND fetch the provider's openid-configuration. To never
// contact a real external service, the backend looks up its own discovery document. This is the
// backend's URL as seen from the backend itself (Rauthy listens on 8080 in dev_mode).
const SELF = process.env.E2E_BACKEND_SELF_URL || 'http://localhost:8080';
const SELF_ISSUER = `${SELF}/auth/v1`;

// Logos only exist once uploaded: the logo <img> probes the URL and gets a 404 by design.
const NO_LOGO = /http 404 GET \/auth\/v1\/providers\/[^/]+\/img$/;

/** @typedef {Record<string, any> & { id: string, name: string }} Provider */

/** @param {import('@playwright/test').Page} page */
async function providers(page) {
    return /** @type {Provider[]} */ (await api(page, 'POST', 'providers'));
}

/** @param {import('@playwright/test').Page} page @param {string} id */
async function providerById(page, id) {
    return (await providers(page)).find(p => p.id === id);
}

/** @param {import('@playwright/test').Page} page @param {string} name */
async function providerByName(page, name) {
    return (await providers(page)).find(p => p.name === name);
}

/** A complete custom provider with dummy endpoints (never contacted). */
function customPayload(/** @type {string} */ name) {
    return {
        name,
        typ: 'custom',
        enabled: true,
        issuer: 'https://idp.example.invalid',
        authorization_endpoint: 'https://idp.example.invalid/authorize',
        token_endpoint: 'https://idp.example.invalid/token',
        userinfo_endpoint: 'https://idp.example.invalid/userinfo',
        use_pkce: true,
        client_secret_basic: true,
        client_secret_post: false,
        auto_onboarding: false,
        auto_link: false,
        client_id: 'e2e-client',
        client_secret: 'e2e-secret',
        scope: 'openid profile',
        admin_claim_path: '$.roles.*',
        admin_claim_value: 'rauthy_admin',
    };
}

/**
 * FINDING: the client secret field is named "Password" instead of "Client Secret"
 * (`ProviderConfigClientInfo.svelte` does not pass `ariaLabel` to `InputPassword`).
 */
function secretField(/** @type {import('@playwright/test').Locator} */ scope) {
    return scope.getByRole('textbox', { name: /^(Client Secret|Password)$/ });
}

/** @param {import('@playwright/test').Locator} scope */
function field(scope, /** @type {string} */ name) {
    return scope.getByRole('textbox', { name, exact: true });
}

/** Waits until the dialog closed after a successful create and returns the provider. */
async function expectCreated(
    /** @type {import('@playwright/test').Page} */ page,
    /** @type {import('@playwright/test').Locator} */ dialog,
    /** @type {string} */ name,
) {
    await expect(dialog).toBeHidden();
    await showSubNav(page);
    await expect(entry(page, name)).toBeVisible();
    return /** @type {Provider} */ (await providerByName(page, name));
}

test.describe('providers list', () => {
    test.use({ allowErrors: [NO_LOGO] });

    test('paginates long lists and opens the selected page from a deep link', async ({ page }) => {
        const rows = Array.from({ length: 24 }, (_, index) => ({
            id: `provider-${index}`,
            ...customPayload(`Provider ${index + 1}`),
        }));
        await page.route('**/auth/v1/providers', route => route.fulfill({ json: rows }));
        await page.goto(PAGE);
        const pager = page.getByRole('navigation', { name: 'Pagination', exact: true });
        await expect(entryList(page).getByRole('listitem')).toHaveCount(20);
        await pager.getByRole('button', { name: 'Go to next page' }).click();
        await expect(entryList(page).getByRole('listitem')).toHaveCount(4);
        await page.goto(`${PAGE}?pid=${rows[23].id}`);
        await expectCurrent(page, rows[23].name);
        await expect(pager.getByRole('listitem', { name: 'Go to page 2' })).toHaveAttribute(
            'aria-current',
            'page',
        );
    });

    test('lists all providers and opens one by click and by deep link', async ({ page }) => {
        await page.goto(PAGE);
        const a = await api(page, 'POST', 'providers/create', customPayload(unique('E2E Prov A')));
        const b = await api(page, 'POST', 'providers/create', customPayload(unique('E2E Prov B')));
        try {
            await page.goto(PAGE);
            const all = await providers(page);
            await expect(entry(page, a.name)).toBeVisible();
            await expect(entryList(page).getByRole('listitem')).toHaveCount(all.length);
            expect((await listedNames(page)).sort()).toEqual(all.map(p => p.name).sort());

            await openEntry(page, b.name);
            await expect(page).toHaveURL(new RegExp(`[?&]pid=${b.id}`));
            await expectCurrent(page, b.name);
            await expect(page.getByRole('main').getByText(b.id)).toBeVisible();
            await expect(field(page.getByRole('main'), 'Client Name')).toHaveValue(b.name);

            await page.goto(`${PAGE}?pid=${a.id}`);
            await expectCurrent(page, a.name);
            await expect(field(page.getByRole('main'), 'Client Name')).toHaveValue(a.name);
        } finally {
            await deleteQuietly(page, `providers/${a.id}`);
            await deleteQuietly(page, `providers/${b.id}`);
        }
    });
});

test.describe('add provider', () => {
    /** @type {string[]} */
    let cleanupNames = [];
    test.afterEach(async ({ page }) => {
        for (const n of cleanupNames) {
            const p = await providerByName(page, n);
            if (p) await deleteQuietly(page, `providers/${p.id}`);
        }
        cleanupNames = [];
    });

    test('offers every template', async ({ page }) => {
        await page.goto(PAGE);
        const dialog = await openAddNew(page);
        const mode = dialog.getByRole('combobox', { name: 'Select Mode' });
        await expect(mode).toHaveText(/OIDC/);
        await mode.click();
        const listbox = page.getByRole('listbox').last();
        for (const m of ['OIDC', 'Auto', 'Custom', 'GitHub', 'Google']) {
            await expect(
                listbox
                    .getByRole('option', { name: m, exact: true })
                    .or(listbox.getByRole('button', { name: m, exact: true })),
            ).toBeVisible();
        }
        await page.keyboard.press('Escape');
        await dialog.getByRole('button', { name: 'Close' }).click();
        await expect(dialog).toBeHidden();
    });

    test.describe('OIDC discovery', () => {
        // The unreachable issuer is looked up by the backend and fails with a 5xx on purpose.
        test.use({ allowErrors: [/http 5\d\d POST \/auth\/v1\/providers\/lookup$/] });

        test('validates the issuer, shows lookup errors, discovers and saves', async ({ page }) => {
            const name = unique('E2E OIDC');
            cleanupNames.push(name);
            await page.goto(PAGE);
            const lookups = recordRequests(page, 'POST', /\/auth\/v1\/providers\/lookup$/);
            const dialog = await openAddNew(page);
            const issuer = field(dialog, 'Issuer URL');
            const lookup = dialog.getByRole('button', { name: 'Lookup' });

            await lookup.click();
            await expect(issuer).toHaveAttribute('aria-invalid', 'true');
            await issuer.fill('not a url');
            await expect(dialog.getByText('Invalid Input')).toBeVisible();
            await lookup.click();
            expect(lookups).toEqual([]);

            // unreachable issuer: the backend error is shown, nothing else happens
            await issuer.fill('http://localhost:1/nothing');
            await lookup.click();
            await expect(dialog.getByText(/Cannot send out HTTP request/)).toBeVisible();
            await expect(lookup).toBeVisible();
            expect(lookups).toHaveLength(1);

            // the backend's own discovery document
            await issuer.fill(SELF_ISSUER);
            await lookup.click();
            await expect(dialog.getByRole('button', { name: 'Save' })).toBeVisible();
            await expect(field(dialog, 'Issuer URL')).toBeDisabled();
            await expect(field(dialog, 'Issuer URL')).toHaveValue(`${SELF_ISSUER}/`);
            await expect(field(dialog, 'Authorization Endpoint')).toHaveValue(
                `${SELF_ISSUER}/oidc/authorize`,
            );
            await expect(field(dialog, 'Token Endpoint')).toHaveValue(`${SELF_ISSUER}/oidc/token`);
            await expect(field(dialog, 'Userinfo Endpoint')).toHaveValue(
                `${SELF_ISSUER}/oidc/userinfo`,
            );
            await expect(field(dialog, 'Userinfo Endpoint')).toBeDisabled();
            await expect(field(dialog, 'Scope')).toHaveValue(/openid profile email/);
            // PKCE is discovered, not editable
            await expect(dialog.getByRole('checkbox', { name: 'PKCE' })).toHaveCount(0);
            await expect(
                dialog.getByRole('checkbox', { name: 'client_secret_basic' }),
            ).toBeChecked();
            await expect(
                dialog.getByRole('checkbox', { name: 'client_secret_post' }),
            ).not.toBeChecked();

            // Reset goes back to the lookup
            await dialog.getByRole('button', { name: 'Reset' }).click();
            await expect(lookup).toBeVisible();
            await expect(field(dialog, 'Issuer URL')).toBeEnabled();
            await expect(field(dialog, 'Issuer URL')).toHaveValue('');

            await field(dialog, 'Issuer URL').fill(SELF_ISSUER);
            await lookup.click();
            await expect(dialog.getByRole('button', { name: 'Save' })).toBeVisible();
            // required client values
            await dialog.getByRole('button', { name: 'Save' }).click();
            await expect(field(dialog, 'Client Name')).toHaveAttribute('aria-invalid', 'true');
            await field(dialog, 'Client Name').fill(name);
            await field(dialog, 'Client ID').fill('e2e-oidc-client');
            await dialog.getByRole('button', { name: 'Save' }).click();

            const p = await expectCreated(page, dialog, name);
            expect(p).toMatchObject({
                typ: 'oidc',
                enabled: true,
                issuer: `${SELF_ISSUER}/`,
                token_endpoint: `${SELF_ISSUER}/oidc/token`,
                client_id: 'e2e-oidc-client',
                use_pkce: true,
                client_secret_basic: true,
                client_secret_post: false,
            });
            expect(p.scope.replaceAll('+', ' ')).toBe('openid profile email');
        });
    });

    test('Auto: discovers from a metadata URL', async ({ page }) => {
        const name = unique('E2E Auto');
        cleanupNames.push(name);
        await page.goto(PAGE);
        const dialog = await openAddNew(page);
        await chooseOption(page, dialog.getByRole('combobox', { name: 'Select Mode' }), 'Auto');
        const meta = field(dialog, 'Metadata URL');
        await expect(meta).toBeVisible();
        await expect(field(dialog, 'Issuer URL')).toHaveCount(0);
        await meta.fill(`${SELF_ISSUER}/.well-known/openid-configuration`);
        await dialog.getByRole('button', { name: 'Lookup' }).click();
        await expect(field(dialog, 'Token Endpoint')).toHaveValue(`${SELF_ISSUER}/oidc/token`);
        await field(dialog, 'Client Name').fill(name);
        await field(dialog, 'Client ID').fill('e2e-auto-client');
        await dialog.getByRole('button', { name: 'Save' }).click();

        const p = await expectCreated(page, dialog, name);
        expect(p).toMatchObject({ typ: 'custom', issuer: `${SELF_ISSUER}/`, use_pkce: true });
    });

    test('Custom: every field manually, with validation', async ({ page }) => {
        const name = unique('E2E Custom');
        cleanupNames.push(name);
        await page.goto(PAGE);
        const creates = recordRequests(page, 'POST', /\/auth\/v1\/providers\/create$/);
        const dialog = await openAddNew(page);
        await chooseOption(page, dialog.getByRole('combobox', { name: 'Select Mode' }), 'Custom');

        for (const f of [
            'Issuer URL',
            'Authorization Endpoint',
            'Token Endpoint',
            'Userinfo Endpoint',
            'Scope',
            'Client Name',
            'Client ID',
        ]) {
            await expect(field(dialog, f)).toBeEditable();
            await expect(field(dialog, f)).toHaveValue('');
        }
        await expect(dialog.getByRole('checkbox', { name: 'PKCE' })).toBeChecked();
        await expect(dialog.getByRole('checkbox', { name: 'client_secret_basic' })).toBeChecked();

        // empty form is blocked
        await dialog.getByRole('button', { name: 'Save' }).click();
        await expect(field(dialog, 'Issuer URL')).toHaveAttribute('aria-invalid', 'true');

        await field(dialog, 'Issuer URL').fill('https://idp.example.invalid');
        await field(dialog, 'Authorization Endpoint').fill('https://idp.example.invalid/auth');
        await field(dialog, 'Token Endpoint').fill('https://idp.example.invalid/token');
        await field(dialog, 'Userinfo Endpoint').fill('https://idp.example.invalid/userinfo');
        await field(dialog, 'Scope').fill('openid email');
        await field(dialog, 'Client Name').fill(name);
        await field(dialog, 'Client ID').fill('custom-client');

        // invalid scope characters
        await field(dialog, 'Scope').fill('openid, email!');
        await expect(field(dialog, 'Scope')).toHaveAttribute('aria-invalid', 'true');
        await field(dialog, 'Scope').fill('openid email');
        await expect(field(dialog, 'Scope')).toHaveAttribute('aria-invalid', 'false');

        // without PKCE a secret is required
        await dialog.getByRole('checkbox', { name: 'PKCE' }).uncheck();
        await dialog.getByRole('button', { name: 'Save' }).click();
        await expect(
            dialog.getByText('Must at least be a confidential client or use PKCE').first(),
        ).toBeVisible();

        // a secret without any auth method
        await secretField(dialog).fill('custom-secret');
        await dialog.getByRole('checkbox', { name: 'client_secret_basic' }).uncheck();
        await expect(
            dialog
                .getByText('You have given a client secret, but no client auth method is active')
                .first(),
        ).toBeVisible();
        await dialog.getByRole('button', { name: 'Save' }).click();
        expect(creates).toEqual([]);
        await dialog.getByRole('checkbox', { name: 'client_secret_post' }).check();

        // an admin claim path needs a value
        await field(dialog, 'Admin Claim Path').fill('$.roles.*');
        await dialog.getByRole('button', { name: 'Save' }).click();
        await expect(field(dialog, 'Admin Claim Value')).toHaveAttribute('aria-invalid', 'true');
        await field(dialog, 'Admin Claim Value').fill('idp-admin');
        await field(dialog, 'MFA Claim Path').fill('$.amr.*');
        await field(dialog, 'MFA Claim Value').fill('mfa');
        expect(creates).toEqual([]);

        // show / hide the secret
        await dialog.getByRole('button', { name: 'Show' }).click();
        await expect(secretField(dialog)).toHaveAttribute('type', 'text');
        await dialog.getByRole('button', { name: 'Hide' }).click();
        await expect(secretField(dialog)).toHaveAttribute('type', 'password');

        await dialog.getByRole('button', { name: 'Save' }).click();
        const p = await expectCreated(page, dialog, name);
        expect(creates).toHaveLength(1);
        expect(p).toMatchObject({
            typ: 'custom',
            enabled: true,
            issuer: 'https://idp.example.invalid',
            authorization_endpoint: 'https://idp.example.invalid/auth',
            token_endpoint: 'https://idp.example.invalid/token',
            userinfo_endpoint: 'https://idp.example.invalid/userinfo',
            client_id: 'custom-client',
            use_pkce: false,
            client_secret_basic: false,
            client_secret_post: true,
            admin_claim_path: '$.roles.*',
            admin_claim_value: 'idp-admin',
            mfa_claim_path: '$.amr.*',
            mfa_claim_value: 'mfa',
        });
        expect(p.scope.replaceAll('+', ' ')).toBe('openid email');
    });

    test('GitHub: template values are prefilled', async ({ page }) => {
        const name = unique('E2E GitHub');
        cleanupNames.push(name);
        await page.goto(PAGE);
        const dialog = await openAddNew(page);
        await chooseOption(page, dialog.getByRole('combobox', { name: 'Select Mode' }), 'GitHub');

        await expect(field(dialog, 'Issuer URL')).toHaveValue('github.com');
        await expect(field(dialog, 'Issuer URL')).toBeDisabled();
        await expect(field(dialog, 'Authorization Endpoint')).toHaveValue(
            'https://github.com/login/oauth/authorize',
        );
        await expect(field(dialog, 'Token Endpoint')).toHaveValue(
            'https://github.com/login/oauth/access_token',
        );
        await expect(field(dialog, 'Userinfo Endpoint')).toHaveValue('https://api.github.com/user');
        await expect(field(dialog, 'Scope')).toHaveValue('user:email');
        await expect(field(dialog, 'Client Name')).toHaveValue('GitHub');
        await expect(field(dialog, 'MFA Claim Path')).toHaveValue('$.two_factor_authentication');
        await expect(field(dialog, 'MFA Claim Value')).toHaveValue('true');
        await expect(dialog.getByRole('checkbox', { name: 'client_secret_post' })).toBeChecked();

        await field(dialog, 'Client Name').fill(name);
        await field(dialog, 'Client ID').fill('gh-client-id');
        // GitHub has no PKCE: the secret is required
        await dialog.getByRole('button', { name: 'Save' }).click();
        await expect(secretField(dialog)).toHaveAttribute('aria-invalid', /true/);
        await secretField(dialog).fill('gh-secret');
        await dialog.getByRole('button', { name: 'Save' }).click();

        const p = await expectCreated(page, dialog, name);
        expect(p).toMatchObject({
            typ: 'github',
            issuer: 'github.com',
            use_pkce: false,
            client_id: 'gh-client-id',
            mfa_claim_path: '$.two_factor_authentication',
        });
    });

    test('Google: template discovers its config', async ({ page }) => {
        const name = unique('E2E Google');
        cleanupNames.push(name);
        await page.goto(PAGE);
        // Never let the backend contact Google: the lookup is answered with Google's
        // well-known values directly in the browser.
        /** @type {any[]} */
        const lookupBodies = [];
        await page.route('**/auth/v1/providers/lookup', async route => {
            lookupBodies.push(route.request().postDataJSON());
            await route.fulfill({
                json: {
                    issuer: 'https://accounts.google.com',
                    authorization_endpoint: 'https://accounts.google.com/o/oauth2/v2/auth',
                    token_endpoint: 'https://oauth2.googleapis.com/token',
                    userinfo_endpoint: 'https://openidconnect.googleapis.com/v1/userinfo',
                    jwks_endpoint: 'https://www.googleapis.com/oauth2/v3/certs',
                    scope: 'openid profile email ',
                    use_pkce: true,
                    client_secret_basic: true,
                    client_secret_post: true,
                },
            });
        });
        const dialog = await openAddNew(page);
        await chooseOption(page, dialog.getByRole('combobox', { name: 'Select Mode' }), 'Google');
        await expect(field(dialog, 'Issuer URL')).toHaveValue('https://accounts.google.com');
        expect(lookupBodies).toEqual([{ issuer: 'accounts.google.com' }]);
        await expect(field(dialog, 'Token Endpoint')).toBeDisabled();
        // basic only, if both are supported
        await expect(dialog.getByRole('checkbox', { name: 'client_secret_basic' })).toBeChecked();
        await expect(
            dialog.getByRole('checkbox', { name: 'client_secret_post' }),
        ).not.toBeChecked();

        await field(dialog, 'Client Name').fill(name);
        await field(dialog, 'Client ID').fill('google-client.apps.example');
        await secretField(dialog).fill('google-secret');
        await dialog.getByRole('button', { name: 'Save' }).click();

        const p = await expectCreated(page, dialog, name);
        expect(p).toMatchObject({
            typ: 'google',
            issuer: 'https://accounts.google.com',
            token_endpoint: 'https://oauth2.googleapis.com/token',
            client_id: 'google-client.apps.example',
            use_pkce: true,
        });
    });
});

test.describe('edit provider', () => {
    test.use({ allowErrors: [NO_LOGO] });

    /** @type {Provider} */
    let provider;
    test.beforeEach(async ({ page }) => {
        await page.goto(PAGE);
        provider = await api(page, 'POST', 'providers/create', customPayload(unique('E2E Edit')));
    });
    test.afterEach(async ({ page }) => {
        await deleteQuietly(page, `providers/${provider.id}`);
    });

    test('shows the stored config', async ({ page }) => {
        await page.goto(`${PAGE}?pid=${provider.id}`);
        const main = page.getByRole('main');
        await expect(field(main, 'Client Name')).toHaveValue(provider.name);
        await expect(main.getByRole('checkbox', { name: 'Enabled' })).toBeChecked();
        await expect(main.getByRole('checkbox', { name: 'Auto-Onboarding' })).not.toBeChecked();
        await expect(main.getByRole('checkbox', { name: 'Auto-Link User' })).not.toBeChecked();
        await expect(field(main, 'Issuer URL')).toHaveValue('https://idp.example.invalid');
        await expect(field(main, 'Issuer URL')).toBeEditable();
        await expect(field(main, 'Authorization Endpoint')).toHaveValue(
            'https://idp.example.invalid/authorize',
        );
        await expect(field(main, 'Token Endpoint')).toHaveValue(
            'https://idp.example.invalid/token',
        );
        await expect(field(main, 'Userinfo Endpoint')).toHaveValue(
            'https://idp.example.invalid/userinfo',
        );
        await expect(main.getByRole('checkbox', { name: 'PKCE' })).toBeChecked();
        await expect(field(main, 'Scope')).toHaveValue('openid profile');
        await expect(field(main, 'Client ID')).toHaveValue('e2e-client');
        await expect(main.getByRole('checkbox', { name: 'client_secret_basic' })).toBeChecked();
        await expect(main.getByRole('checkbox', { name: 'client_secret_post' })).not.toBeChecked();
        await expect(field(main, 'Admin Claim Path')).toHaveValue('$.roles.*');
        await expect(field(main, 'Admin Claim Value')).toHaveValue('rauthy_admin');
        await expect(field(main, 'MFA Claim Path')).toHaveValue('');
        await expect(field(main, 'MFA Claim Value')).toHaveValue('');
    });

    test('edit every section, save, reload: persisted', async ({ page }) => {
        await page.goto(`${PAGE}?pid=${provider.id}`);
        const main = page.getByRole('main');
        const newName = unique('E2E Edited');
        await expect(field(main, 'Client Name')).toHaveValue(provider.name);

        await main.getByRole('checkbox', { name: 'Enabled' }).uncheck();
        await main.getByRole('checkbox', { name: 'Auto-Onboarding' }).check();
        await main.getByRole('checkbox', { name: 'Auto-Link User' }).check();
        await expect(main.getByText(/This option can be very dangerous/)).toBeVisible();

        await field(main, 'Issuer URL').fill('https://idp2.example.invalid');
        await field(main, 'Authorization Endpoint').fill('https://idp2.example.invalid/a');
        await field(main, 'Token Endpoint').fill('https://idp2.example.invalid/t');
        await field(main, 'Userinfo Endpoint').fill('https://idp2.example.invalid/u');
        await main.getByRole('checkbox', { name: 'PKCE' }).uncheck();
        await field(main, 'Scope').fill('openid email groups');
        await field(main, 'Client Name').fill(newName);
        await field(main, 'Client ID').fill('edited-client');
        await secretField(main).fill('edited-secret');
        await main.getByRole('checkbox', { name: 'client_secret_basic' }).uncheck();
        await main.getByRole('checkbox', { name: 'client_secret_post' }).check();
        await field(main, 'Admin Claim Path').fill('$.groups.*');
        await field(main, 'Admin Claim Value').fill('admins');
        await field(main, 'MFA Claim Path').fill('$.acr');
        await field(main, 'MFA Claim Value').fill('mfa');
        await saveButton(page).click();

        const expected = {
            name: newName,
            enabled: false,
            auto_onboarding: true,
            auto_link: true,
            issuer: 'https://idp2.example.invalid',
            authorization_endpoint: 'https://idp2.example.invalid/a',
            token_endpoint: 'https://idp2.example.invalid/t',
            userinfo_endpoint: 'https://idp2.example.invalid/u',
            use_pkce: false,
            client_id: 'edited-client',
            client_secret_basic: false,
            client_secret_post: true,
            admin_claim_path: '$.groups.*',
            admin_claim_value: 'admins',
            mfa_claim_path: '$.acr',
            mfa_claim_value: 'mfa',
        };
        await expect
            .poll(async () => {
                const p = await providerById(page, provider.id);
                return Object.fromEntries(Object.keys(expected).map(k => [k, p?.[k]]));
            })
            .toEqual(expected);
        expect((await providerById(page, provider.id))?.scope.replaceAll('+', ' ')).toBe(
            'openid email groups',
        );
        await showSubNav(page);
        await expect(entry(page, newName)).toBeVisible();

        await page.reload();
        await expect(field(main, 'Client Name')).toHaveValue(newName);
        await expect(main.getByRole('checkbox', { name: 'Enabled' })).not.toBeChecked();
        await expect(main.getByRole('checkbox', { name: 'Auto-Onboarding' })).toBeChecked();
        await expect(main.getByRole('checkbox', { name: 'Auto-Link User' })).toBeChecked();
        await expect(field(main, 'Issuer URL')).toHaveValue('https://idp2.example.invalid');
        await expect(field(main, 'Userinfo Endpoint')).toHaveValue(
            'https://idp2.example.invalid/u',
        );
        await expect(main.getByRole('checkbox', { name: 'PKCE' })).not.toBeChecked();
        await expect(field(main, 'Scope')).toHaveValue('openid email groups');
        await expect(field(main, 'Client ID')).toHaveValue('edited-client');
        await expect(main.getByRole('checkbox', { name: 'client_secret_post' })).toBeChecked();
        await expect(main.getByRole('checkbox', { name: 'client_secret_basic' })).not.toBeChecked();
        await expect(field(main, 'Admin Claim Path')).toHaveValue('$.groups.*');
        await expect(field(main, 'MFA Claim Value')).toHaveValue('mfa');

        // clearing the optional mappings removes them
        await field(main, 'Admin Claim Path').fill('');
        await field(main, 'Admin Claim Value').fill('');
        await field(main, 'MFA Claim Path').fill('');
        await field(main, 'MFA Claim Value').fill('');
        await saveButton(page).click();
        await expect
            .poll(async () => {
                const p = await providerById(page, provider.id);
                // cleared values come back as null or are left out
                return [p?.admin_claim_path ?? null, p?.mfa_claim_value ?? null];
            })
            .toEqual([null, null]);
    });

    test('invalid input shows errors and blocks the save', async ({ page }) => {
        await page.goto(`${PAGE}?pid=${provider.id}`);
        const main = page.getByRole('main');
        await expect(field(main, 'Client Name')).toHaveValue(provider.name);
        const puts = recordRequests(page, 'PUT', new RegExp(`/providers/${provider.id}$`));

        await field(main, 'Issuer URL').fill('has spaces in it');
        await expect(field(main, 'Issuer URL')).toHaveAttribute('aria-invalid', 'true');
        await saveButton(page).click();
        await field(main, 'Issuer URL').fill('https://idp.example.invalid');

        await field(main, 'Client Name').fill('x');
        await expect(field(main, 'Client Name')).toHaveAttribute('aria-invalid', 'true');
        await saveButton(page).click();
        await field(main, 'Client Name').fill(provider.name);

        await field(main, 'MFA Claim Path').fill('$.amr.*');
        await saveButton(page).click();
        await expect(field(main, 'MFA Claim Value')).toHaveAttribute('aria-invalid', 'true');
        await field(main, 'MFA Claim Path').fill('');

        // a secret without any auth method
        await secretField(main).fill('some-secret');
        await main.getByRole('checkbox', { name: 'client_secret_basic' }).uncheck();
        await saveButton(page).click();
        await expect(
            main.getByText('You have given a client secret, but no client auth method is active'),
        ).toBeVisible();

        // neither PKCE nor a secret
        await main.getByRole('checkbox', { name: 'client_secret_basic' }).check();
        await secretField(main).fill('');
        await main.getByRole('checkbox', { name: 'PKCE' }).uncheck();
        await saveButton(page).click();
        await expect(
            main.getByText('Must at least be a confidential client or use PKCE').first(),
        ).toBeVisible();

        expect(puts).toEqual([]);
        expect((await providerById(page, provider.id))?.use_pkce).toBe(true);
    });

    test('upload and remove the logo', async ({ page }) => {
        await page.goto(`${PAGE}?pid=${provider.id}`);
        const main = page.getByRole('main');
        await expect(field(main, 'Client Name')).toHaveValue(provider.name);
        const logoUrl = `/auth/v1/providers/${provider.id}/img`;
        expect((await page.request.get(logoUrl)).status()).toBe(404);

        const header = main.locator('.admin-entity-header');
        const upload = header.getByLabel('Edit · Logo', { exact: true });
        await expect(upload).toBeVisible();
        // providers need a logo of at least 20px
        const png = await makePng();
        await upload.setInputFiles({ name: 'logo.png', mimeType: 'image/png', buffer: png });
        await expect.poll(async () => (await page.request.get(logoUrl)).status()).toBe(200);

        const remove = main.getByRole('button', { name: 'Delete · Logo', exact: true });
        await expect(remove).toBeVisible();
        await remove.click();
        const confirm = page.getByRole('dialog', { name: 'Delete · Logo', exact: true });
        await confirm.getByRole('button', { name: 'Delete · Logo', exact: true }).click();
        await expect.poll(async () => (await page.request.get(logoUrl)).status()).toBe(404);
        await expect(remove).toHaveCount(0);
    });
});

test.describe('delete provider', () => {
    test.use({ allowErrors: [NO_LOGO] });

    test('header action asks first, cancelling keeps the entry, confirming deletes it', async ({
        page,
    }) => {
        await page.goto(PAGE);
        const p = await api(page, 'POST', 'providers/create', customPayload(unique('E2E Del')));
        try {
            await page.goto(`${PAGE}?pid=${p.id}`);
            const main = page.getByRole('main');
            await expect(field(main, 'Client Name')).toHaveValue(p.name);

            await openEntityDelete(page);
            await expect(
                main.getByText('Are you sure you want to delete this provider?'),
            ).toBeVisible();
            // not in use by any user: no force delete needed
            await expect(main.getByRole('checkbox', { name: 'Force Delete' })).toHaveCount(0);

            await page
                .getByRole('dialog', { name: 'Delete' })
                .getByRole('button', { name: 'Cancel' })
                .click();
            await expect(field(main, 'Client Name')).toHaveValue(p.name);
            expect(await providerById(page, p.id)).toBeTruthy();

            await openEntityDelete(page);
            await deleteAction(page).click();
            await expect.poll(async () => await providerById(page, p.id)).toBeUndefined();
            await expect(entry(page, p.name)).toHaveCount(0);
        } finally {
            await deleteQuietly(page, `providers/${p.id}`);
        }
    });
});

test.describe('providers on a phone', () => {
    test.use({ allowErrors: [NO_LOGO] });

    test('open, edit and delete a provider @mobile', async ({ page }) => {
        await page.goto(PAGE);
        const p = await api(page, 'POST', 'providers/create', customPayload(unique('E2E Mob')));
        try {
            await page.goto(PAGE);
            await openEntry(page, p.name);
            const main = page.getByRole('main');
            await expect(field(main, 'Client Name')).toHaveValue(p.name);
            await field(main, 'Client ID').fill('mobile-client');
            await saveButton(page).click();
            await expect
                .poll(async () => (await providerById(page, p.id))?.client_id)
                .toBe('mobile-client');

            await openEntityDelete(page);
            await deleteAction(page).click();
            await expect.poll(async () => await providerById(page, p.id)).toBeUndefined();
        } finally {
            await deleteQuietly(page, `providers/${p.id}`);
        }
    });
});
