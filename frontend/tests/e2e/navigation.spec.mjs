import { test, expect } from './fixtures.mjs';
import { chooseOption } from './helpers/ui.mjs';

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

    test('language selector switches the UI language', async ({ page }) => {
        await page.goto('/auth/v1/admin/users');
        await chooseOption(page, page.getByRole('combobox', { name: 'Select Language' }), 'de');
        await expect(page.getByRole('link', { name: 'Benutzer', exact: true })).toBeVisible();
        await chooseOption(page, page.getByRole('combobox', { name: /Sprache/ }), 'en');
        await expect(page.getByRole('link', { name: 'Users', exact: true })).toBeVisible();
    });
});
