import { test, expect } from './fixtures.mjs';
import { chooseOption } from './helpers/ui.mjs';
import { ACCOUNT_EXPECTED_ERRORS } from './helpers/account.mjs';

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
    ['Docs', 'admin/docs'],
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

    test('opens and browses Account from the sidebar footer', async ({ page, allowErrors }) => {
        allowErrors.push(...ACCOUNT_EXPECTED_ERRORS);
        await page.goto('/auth/v1/admin/users');
        await page.locator('#mainNav').getByRole('link', { name: 'User Account' }).click();
        await expect(page).toHaveURL(/\/auth\/v1\/account/);
        const main = page.getByRole('main');
        await expect(main.getByRole('heading', { name: 'User Account' })).toBeVisible();
        const password = main.getByRole('button', { name: 'Password', exact: true });
        await password.click();
        await expect(password).toHaveAttribute('aria-current', 'step');
        await page.setViewportSize({ width: 390, height: 850 });
        await expect(password).toHaveAttribute('aria-current', 'step');
        await expect(main.getByRole('button', { name: 'Logout', exact: true })).toBeVisible();
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
        expect(positions.workspace).toBe(positions.users);
        const nav = page.locator('#mainNav');
        await expect(nav.getByText(/^v\d+\./)).toHaveCount(0);
        await nav.getByRole('button', { name: 'About' }).click();
        await expect(nav.getByText(/^v\d+\./)).toBeVisible();
        await page.keyboard.press('Escape');
        await page.setViewportSize({ width: 960, height: 768 });
        await expect(nav).toHaveAttribute('data-compact', 'true');
        await expect(nav.getByRole('link', { name: 'Docs', exact: true })).toBeInViewport({
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
});
