import { expect } from '@playwright/test';
import { readFile } from 'node:fs/promises';
import { after, before, test } from 'node:test';
import { injectTemplates, launchChromium, readBuiltHtml, serveUi } from './helpers/browser.mjs';

/** @type {Awaited<ReturnType<typeof launchChromium>>} */
let browser;
let html = '';
let theme = '';
const documentPath = '/auth/v1/';

before(async () => {
    html = await readBuiltHtml('index.html');
    theme = await readFile(new URL('../src/css/theme.css', import.meta.url), 'utf8');
    browser = await launchChromium();
});
after(async () => browser?.close());

for (const showOptionalLinks of [true, false]) {
    test(`public home respects registration and admin visibility (${showOptionalLinks})`, async t => {
        const context = await browser.newContext({ locale: 'en-US' });
        t.after(() => context.close());
        await serveUi({
            context,
            documentPath,
            html: injectTemplates(html, {
                tpl_admin_btn_hide: String(!showOptionalLinks),
                tpl_is_reg_open: String(showOptionalLinks),
            }),
            requests: [],
            handle: (_, request, url) => {
                if (request.method() === 'GET' && url.pathname.startsWith('/auth/v1/theme/')) {
                    return { contentType: 'text/css', body: theme };
                }
            },
        });

        const page = await context.newPage();
        await page.goto(`http://localhost${documentPath}`);
        const navigation = page.getByRole('navigation', { name: 'Rauthy' });
        await expect(page.getByRole('button', { name: /Change Theme/ })).toBeVisible();
        await expect(navigation.getByRole('link', { name: 'Account' })).toHaveAttribute(
            'href',
            '/auth/v1/account',
        );

        if (showOptionalLinks) {
            const admin = navigation.getByRole('link', { name: 'Admin' });
            await expect(admin).toHaveAttribute('href', '/auth/v1/admin');
            await expect(admin).toHaveCSS('border-top-style', 'solid');
            await expect(navigation.locator('[href="/auth/v1/users/register"]')).toBeVisible();
        } else {
            await expect(navigation.getByRole('link')).toHaveCount(1);
        }
    });
}
