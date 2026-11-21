import { mkdir } from 'node:fs/promises';
import { test as setup, expect } from '@playwright/test';
import { ADMIN_EMAIL, ADMIN_PASSWORD, AUTH_STATE } from './helpers/env.mjs';
import { loginWithPassword } from './helpers/login.mjs';

setup('admin session', async ({ page }) => {
    await loginWithPassword(page, { email: ADMIN_EMAIL, password: ADMIN_PASSWORD });
    await expect(page).toHaveURL(/\/auth\/v1\/admin/, { timeout: 30_000 });
    await mkdir('./test-results/.auth', { recursive: true });
    await page.context().storageState({ path: AUTH_STATE });
});
