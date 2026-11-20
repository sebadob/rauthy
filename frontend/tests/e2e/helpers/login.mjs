import { expect } from '@playwright/test';

/**
 * Walks through the real login page: e-mail first, then the password prompt the backend asks for.
 * @param {import('@playwright/test').Page} page
 * @param {{ email: string, password: string, startPath?: string }} opts
 */
export async function loginWithPassword(page, { email, password, startPath = '/auth/v1/admin' }) {
    await page.goto(startPath);
    const emailInput = page.getByRole('textbox', { name: /e-?mail/i });
    await emailInput.fill(email);
    await emailInput.press('Enter');
    const pwd = page.getByLabel('Password', { exact: true });
    await expect(pwd).toBeVisible({ timeout: 20_000 });
    await pwd.fill(password);
    await pwd.press('Enter');
}
