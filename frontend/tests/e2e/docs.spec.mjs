import { test, expect } from './fixtures.mjs';

// The links of the docs page: accessible name, target and whether it leaves the instance.
const LINKS = [
    { name: 'Rauthy Book', href: 'https://sebadob.github.io/rauthy/', external: true },
    { name: 'Swagger UI', href: '/auth/v1/docs/', external: false },
    { name: 'GitHub', href: 'https://github.com/sebadob/rauthy', external: true },
];

test.describe('docs', () => {
    test('renders every section with its links @mobile', async ({ page }) => {
        await page.goto('/auth/v1/admin/docs');
        await expect(page).toHaveTitle('Rauthy · About');
        const main = page.getByRole('main');
        await expect(main.getByRole('heading', { name: 'About', level: 2 })).toBeVisible();
        await expect(main.getByRole('heading', { name: 'Docs', level: 3 })).toBeVisible();
        await expect(main.getByText(/^v\d+\./)).toBeVisible();
        await expect(main.getByText(/general documentation about Rauthy itself/)).toBeVisible();
        await expect(main.getByText(/integrate an external application/)).toBeVisible();
        await expect(main.getByText(/swagger_ui_enable/)).toBeVisible();
        await expect(main.getByText(/source code can be found here/)).toBeVisible();

        for (const l of LINKS) {
            const link = main.getByRole('link', { name: l.name, exact: true });
            await expect(link).toBeVisible();
            await expect(link).toHaveAttribute('href', l.href);
            // every docs link opens in a new tab so the admin UI stays open
            await expect(link).toHaveAttribute('target', '_blank');
        }
    });

    test('external links open in a new tab without access to the admin UI', async ({
        page,
        context,
    }) => {
        // Never leave the test machine: answer the external pages locally.
        await context.route(/^https:\/\/(sebadob\.github\.io|github\.com)\//, route =>
            route.fulfill({ contentType: 'text/html', body: '<title>external</title>ok' }),
        );
        await page.goto('/auth/v1/admin/docs');
        for (const l of LINKS.filter(l => l.external)) {
            const popupP = page.waitForEvent('popup');
            await page.getByRole('main').getByRole('link', { name: l.name, exact: true }).click();
            const popup = await popupP;
            await popup.waitForLoadState();
            expect(popup.url()).toBe(l.href);
            // FINDING: the links have no rel="noopener noreferrer"; current browsers imply
            // noopener for target=_blank, older ones do not.
            expect(await popup.evaluate(() => window.opener)).toBeNull();
            await popup.close();
            await expect(page).toHaveURL(/\/auth\/v1\/admin\/docs$/);
        }
    });

    test('the About link of the main navigation is marked as current page', async ({ page }) => {
        await page.goto('/auth/v1/admin/docs');
        await expect(
            page.getByRole('navigation').first().getByRole('link', { name: 'About', exact: true }),
        ).toHaveAttribute('aria-current', 'page');
    });
});
