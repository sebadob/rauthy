import { test, expect } from './fixtures.mjs';
import { chooseOption } from './helpers/ui.mjs';
import { api } from './helpers/api.mjs';
import {
    ACCOUNT_EXPECTED_ERRORS,
    DEVICE_GRANT,
    NO_SESSION,
    accountTab,
    createTestClient,
    createTestUser,
    deleteTestUser,
    loginToAccount,
    pollDeviceToken,
    startDeviceAuthorization,
} from './helpers/account.mjs';

// Every admin section reachable from the main navigation, with the landing path it must open.
// Some sections append their own query, e.g. the KV store selects its default namespace.
const landing = (/** @type {string} */ path) => new RegExp(`/auth/v1/${path}(\\?.*)?$`);
const SECTIONS = [
    ['Users', 'admin/users'],
    ['Clients', 'admin/clients'],
    ['Roles', 'admin/roles'],
    ['Groups', 'admin/groups'],
    ['Attributes', 'admin/attributes'],
    ['Scopes', 'admin/scopes'],
    ['PAM', 'admin/pam'],
    ['Sessions', 'admin/sessions'],
    ['Events', 'admin/events'],
    ['Blacklist', 'admin/blacklist'],
    ['API Keys', 'admin/api_keys'],
    ['Providers', 'admin/providers'],
    ['KV Store', 'admin/kv'],
    ['Config', 'admin/config/policy'],
    ['About', 'admin/docs'],
];

test.describe('admin navigation', () => {
    test('every main section is reachable from the sidebar', async ({ page }) => {
        await page.goto('/auth/v1/admin/users');
        const nav = page.getByRole('navigation').first();
        for (const [name, url] of SECTIONS) {
            await nav.getByRole('link', { name, exact: true }).click();
            await expect(page).toHaveURL(landing(url));
            await expect(page.getByRole('main')).toBeVisible();
        }
    });

    test('deep links load directly', async ({ page }) => {
        for (const [, url] of SECTIONS) {
            await page.goto(`/auth/v1/${url}`);
            await expect(page).toHaveURL(landing(url));
            await expect(page.getByRole('main')).toBeVisible();
        }
    });

    test('clicking a sidebar link leaves Add New unfocused', async ({ page }) => {
        await page.goto('/auth/v1/admin/users');
        for (const section of ['Clients', 'Attributes', 'Users']) {
            await page
                .locator('#mainNav')
                .getByRole('link', { name: section, exact: true })
                .click();
            await expect(page.getByRole('heading', { name: section, level: 1 })).toBeVisible();
            // The old navigation handler moved focus after a 100 ms timeout.
            await page.waitForTimeout(180);
            await expect(
                page.getByRole('button', { name: 'Add New', exact: true }),
            ).not.toBeFocused();
        }
    });

    test('theme switch toggles and persists', async ({ page }) => {
        await page.goto('/auth/v1/admin/users');
        const toggle = page.getByRole('button', { name: /Change Theme/ });
        const before = await toggle.getAttribute('aria-label');
        await toggle.click();
        await expect(toggle).not.toHaveAttribute('aria-label', /** @type {string} */ (before));
        const after = await toggle.getAttribute('aria-label');
        await page.reload();
        await expect(page.getByRole('button', { name: /Change Theme/ })).toHaveAttribute(
            'aria-label',
            /** @type {string} */ (after),
        );
        await page.getByRole('button', { name: /Change Theme/ }).click();
    });

    for (const [area, key, param] of [
        ['groups', 'group', 'gid'],
        ['roles', 'role', 'rid'],
    ]) {
        test(`shows the users assigned to a ${key} alongside its settings`, async ({ page }) => {
            await page.goto(`/auth/v1/admin/${area}`);
            const name = `membership-${Date.now()}`;
            const entity = await api(page, 'POST', area, { [key]: name });
            try {
                const user = await api(page, 'POST', 'users', {
                    email: `${name}@example.invalid`,
                    given_name: 'Membership',
                    language: 'en',
                    roles: [],
                    [area]: [name],
                });
                try {
                    await page.goto(`/auth/v1/admin/${area}?${param}=${entity.id}`);
                    const main = page.getByRole('main');
                    await main.getByRole('button', { name: 'Users', exact: true }).click();
                    await expect(main.getByText(user.email, { exact: true })).toBeVisible();
                    await expect(
                        main.getByRole('button', { name: `Remove from this ${key}` }),
                    ).toBeVisible();
                    await main.getByRole('button', { name: 'Config', exact: true }).click();
                    const field = key === 'group' ? 'Group Name' : 'Role Name';
                    await expect(main.getByRole('textbox', { name: field })).toHaveValue(name);
                } finally {
                    await deleteTestUser(page, user.id);
                }
            } finally {
                await api(page, 'DELETE', `${area}/${entity.id}`);
            }
        });
    }

    test('opens and browses Account from the sidebar footer @webkit', async ({
        page,
        allowErrors,
    }) => {
        allowErrors.push(...ACCOUNT_EXPECTED_ERRORS);
        await page.goto('/auth/v1/admin/users');
        await page.locator('#mainNav').getByRole('link', { name: 'User Account' }).click();
        await expect(page).toHaveURL(/\/auth\/v1\/account\?v=info$/);
        const main = page.getByRole('main');
        await expect(main.getByRole('heading', { name: 'User Account' })).toBeVisible();
        await expect(main.getByRole('button', { name: 'Info', exact: true })).toHaveAttribute(
            'aria-current',
            'step',
        );
        await page.setViewportSize({ width: 390, height: 500 });
        await main.locator('.tabs').getByRole('button', { name: 'Edit', exact: true }).click();
        await expect(main.getByRole('textbox', { name: 'E-Mail', exact: true })).toHaveValue(/@/);
        const coverage = await main.evaluate(el => {
            const background = el.querySelector('.account-shell');
            if (!background) throw new Error('Missing account background');
            el.scrollTop = el.scrollHeight;
            return {
                scrollTop: el.scrollTop,
                gap: el.getBoundingClientRect().bottom - background.getBoundingClientRect().bottom,
            };
        });
        expect(coverage.scrollTop).toBeGreaterThan(0);
        expect(coverage.gap).toBeLessThanOrEqual(1);
        const password = main.getByRole('button', { name: 'Password', exact: true });
        await password.click();
        await expect(password).toHaveAttribute('aria-current', 'step');
        await page.setViewportSize({ width: 390, height: 850 });
        await expect(password).toHaveAttribute('aria-current', 'step');
        await expect(main.getByRole('button', { name: 'Logout', exact: true })).toBeVisible();
    });

    test('links, renames and revokes a device from Account', async ({
        page,
        browser,
        baseURL,
        request,
    }) => {
        await page.goto('/auth/v1/admin/users');
        const client = await createTestClient(page, {
            flows_enabled: [DEVICE_GRANT, 'refresh_token'],
        });
        const user = await createTestUser(page, { prefix: 'e2e-device' });
        const context = await browser.newContext({ baseURL, storageState: NO_SESSION });
        try {
            const account = await context.newPage();
            await loginToAccount(account, user);
            await accountTab(account, 'Devices').click();
            await expect(account.getByText('No devices linked yet.')).toBeVisible();

            const grant = await startDeviceAuthorization(request, client);
            await account.goto(grant.verification_uri_complete);
            await account.getByRole('button', { name: 'Submit', exact: true }).click();
            await account.getByRole('button', { name: 'Accept', exact: true }).click();
            await expect(accountTab(account, 'Devices')).toHaveAttribute('aria-current', 'step');
            await new Promise(resolve => setTimeout(resolve, grant.interval * 1000));
            const tokens = await pollDeviceToken(request, client, grant.device_code);
            expect(tokens.status).toBe(200);

            await account.getByRole('button', { name: 'Refresh', exact: true }).click();
            await account.getByRole('button', { name: 'Expand content', exact: true }).click();
            await account.getByRole('textbox', { name: 'Name', exact: true }).fill('QA Terminal');
            await account.getByRole('button', { name: 'Save', exact: true }).click();
            await expect(account.getByText('QA Terminal', { exact: true })).toBeVisible();

            await account.getByRole('button', { name: 'Delete the possibility to renew' }).click();
            await account.getByRole('button', { name: 'Cancel', exact: true }).click();
            await expect(
                account.getByText('Access renewable until', { exact: true }),
            ).toBeVisible();
            await account.getByRole('button', { name: 'Delete the possibility to renew' }).click();
            await account
                .getByRole('dialog')
                .getByRole('button', { name: 'Delete', exact: true })
                .click();
            await expect(account.getByText('Access renewable until', { exact: true })).toHaveCount(
                0,
            );

            await account.reload();
            await expect(account.getByText('QA Terminal', { exact: true })).toBeVisible();
            await account.getByRole('button', { name: 'Expand content', exact: true }).click();
            await expect(account.getByText('Access renewable until', { exact: true })).toHaveCount(
                0,
            );
        } finally {
            await context.close();
            await deleteTestUser(page, user.id);
            await api(page, 'DELETE', `clients/${client}`);
        }
    });

    test('aligns the admin header with the sidebar and shows the version in About', async ({
        page,
    }) => {
        await page.setViewportSize({ width: 1440, height: 768 });
        await page.goto('/auth/v1/admin/users');
        await expect(page.getByRole('heading', { name: 'Users', level: 1 })).toBeVisible();
        const positions = await page.evaluate(() => {
            const rect = (/** @type {string} */ selector) =>
                /** @type {Element} */ (document.querySelector(selector)).getBoundingClientRect();
            return {
                header: rect('.admin-header').bottom,
                logo: rect('#mainNav .logo').bottom,
                workspace: rect('.admin-workspace').top,
                users: rect('#mainNav a[href*="/admin/users"]').top,
            };
        });
        expect(positions.header).toBe(positions.logo);
        expect(positions.users - positions.workspace).toBe(6);
        const nav = page.locator('#mainNav');
        await expect(nav.getByText(/^v\d+\./)).toHaveCount(0);
        await nav.getByRole('link', { name: 'About', exact: true }).click();
        await expect(page.getByRole('main').getByText(/^v\d+\./)).toBeVisible();
        await page.setViewportSize({ width: 960, height: 768 });
        await expect(nav).toHaveAttribute('data-compact', 'true');
        await expect(nav.getByRole('link', { name: 'About', exact: true })).toBeInViewport({
            ratio: 1,
        });
    });

    test('switches to Italian in the admin and keeps it on Account', async ({
        page,
        allowErrors,
    }) => {
        allowErrors.push(...ACCOUNT_EXPECTED_ERRORS);
        await page.goto('/auth/v1/admin/users');
        const config = await page.request.get('/auth/v1/i18n_config');
        const languages = await config.json();
        expect(languages.common).toContain('it');
        expect(languages.admin).toContain('it');
        await chooseOption(page, page.getByRole('combobox', { name: 'Select Language' }), 'it');
        await expect(page.locator('html')).toHaveAttribute('lang', 'it-IT');
        await expect(page.getByRole('heading', { name: 'Utenti', level: 1 })).toBeVisible();
        await page.locator('#mainNav').getByRole('link', { name: 'Account utente' }).click();
        await expect(page.getByRole('heading', { name: 'Account utente' })).toBeVisible();
        await expect(page.getByRole('button', { name: 'Esci', exact: true })).toBeVisible();
    });

    test('language selector switches the UI language', async ({ page }) => {
        await page.goto('/auth/v1/admin/users');
        await chooseOption(page, page.getByRole('combobox', { name: 'Select Language' }), 'de');
        await expect(page.getByRole('link', { name: 'Benutzer', exact: true })).toBeVisible();
        await chooseOption(page, page.getByRole('combobox', { name: /Sprache/ }), 'en');
        await expect(page.getByRole('link', { name: 'Users', exact: true })).toBeVisible();
    });

    test('stores Italian and sends the English setup email', async ({ page }) => {
        await page.goto('/auth/v1/admin/users');
        // createTestUser completes password setup via the English New Password email.
        const user = await createTestUser(page, { language: 'it' });
        try {
            expect(await api(page, 'GET', `users/${user.id}`)).toMatchObject({ language: 'it' });
        } finally {
            await deleteTestUser(page, user.id);
        }
    });
});
