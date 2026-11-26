import { deleteAction } from './helpers/entities.mjs';
// Characterization tests for the admin sessions list: loading, counts, pagination, search, sort,
// the session details and invalidating single sessions of a dedicated test user that is logged in
// from separate browser contexts. "Invalidate All Sessions" is never confirmed, because it would
// log out the admin and every other test.
import { test, expect } from './fixtures.mjs';
import { api } from './helpers/api.mjs';
import { AUTH_STATE } from './helpers/env.mjs';
import {
    createUser,
    deleteUser,
    fmtTs,
    loginInNewContext,
    pick,
    rowDeleteButton,
    searchBox,
    topOf,
} from './helpers/ops.mjs';

const SESSIONS = '/auth/v1/admin/sessions';

/** @param {import('@playwright/test').Page} page */
const main = page => page.getByRole('main');
/** One expand button per session entry. */
const rows = (/** @type {import('@playwright/test').Page} */ page) =>
    main(page).getByRole('button', { name: 'Expand content' });

/**
 * @param {import('@playwright/test').Page} page
 * @returns {Promise<{ id: string, user_id: string, exp: number, last_seen: number,
 *   state: string, remote_ip: string, is_mfa: boolean }[]>}
 */
const listSessions = page => api(page, 'GET', 'sessions');

/** @param {import('@playwright/test').Page} page */
async function openSessions(page) {
    await page.goto(SESSIONS);
    await expect(main(page).getByRole('button', { name: 'Invalidate All Sessions' })).toBeVisible();
    await expect(main(page).getByText(/Total: \d+/)).toBeVisible();
}

/**
 * @param {import('@playwright/test').Page} page
 * @param {string} option
 * @param {string} value
 */
async function search(page, option, value) {
    await pick(page, main(page).getByRole('combobox', { name: 'Search Options' }), option);
    await searchBox(main(page)).fill(value);
}

/** Expands every visible session entry. */
async function expandAll(/** @type {import('@playwright/test').Page} */ page) {
    const n = await rows(page).count();
    for (let i = 0; i < n; i++) await rows(page).nth(i).click();
}

test.describe('sessions list', () => {
    test('shows the total, pages through the sessions and changes the page size', async ({
        page,
        browser,
        baseURL,
    }) => {
        await page.goto('/auth/v1/admin');
        const user = await createUser(page, 'e2e-pagination');
        /** @type {Awaited<ReturnType<typeof loginInNewContext>>[]} */
        const sessions = [];
        try {
            for (let i = 0; i < 6; i++) {
                sessions.push(await loginInNewContext(browser, baseURL, user));
            }
            await openSessions(page);
            const total = (await listSessions(page)).length;
            await expect(main(page).getByText(`Total: ${total}`, { exact: true })).toBeVisible();
            await expect(rows(page)).toHaveCount(Math.min(total, 20));

            const pageSize = main(page).getByRole('combobox', { name: 'Show count' });
            await expect(pageSize).toHaveValue('20');
            await pageSize.selectOption('5');
            await expect(rows(page)).toHaveCount(Math.min(total, 5));

            const pagination = main(page).getByRole('navigation', { name: 'Pagination' });
            const prev = main(page).getByRole('button', { name: 'Go to previous page' });
            const next = main(page).getByRole('button', { name: 'Go to next page' });
            await expect(prev).toBeDisabled();
            await expect(
                pagination.getByRole('listitem', { name: 'Go to page 1' }),
            ).toHaveAttribute('aria-current', /step|page|true/);
            expect(total).toBeGreaterThan(5);
            await expect(next).toBeEnabled();
            await next.click();
            await expect(
                pagination.getByRole('listitem', { name: 'Go to page 2' }),
            ).toHaveAttribute('aria-current', /step|page|true/);
            await expect(prev).toBeEnabled();
            await expect(rows(page)).toHaveCount(Math.min(total - 5, 5));
            await prev.click();
            await expect(
                pagination.getByRole('listitem', { name: 'Go to page 1' }),
            ).toHaveAttribute('aria-current', /step|page|true/);
            await pagination.getByRole('button', { name: '2', exact: true }).click();
            await expect(
                pagination.getByRole('listitem', { name: 'Go to page 2' }),
            ).toHaveAttribute('aria-current', /step|page|true/);
        } finally {
            await Promise.all(sessions.map(session => session.close()));
            await deleteUser(page, user.id);
        }
    });

    test('"Invalidate All Sessions" exists and is not hidden behind a menu', async ({ page }) => {
        await openSessions(page);
        await expect(
            main(page).getByRole('button', { name: 'Invalidate All Sessions' }),
        ).toBeEnabled();
    });

    test('"Invalidate All Sessions" asks for confirmation that can be cancelled', async ({
        page,
    }) => {
        let deleteAllRequested = false;
        // Safety net: the request is never allowed to reach the backend.
        await page.route(
            url => url.pathname === '/auth/v1/sessions',
            route => {
                if (route.request().method() === 'DELETE') {
                    deleteAllRequested = true;
                    return route.abort();
                }
                return route.continue();
            },
        );
        page.on('dialog', d => d.dismiss());
        await openSessions(page);
        await main(page).getByRole('button', { name: 'Invalidate All Sessions' }).click();
        const confirm = page.getByRole('dialog').or(page.getByRole('alertdialog'));
        await expect(confirm).toBeVisible({ timeout: 3_000 });
        await confirm.getByRole('button', { name: /cancel/i }).click();
        await expect(confirm).toBeHidden();
        expect(deleteAllRequested).toBe(false);
        await expect(page).toHaveURL(/\/admin\/sessions/);
    });
});

test.describe.serial('sessions of a test user', () => {
    /** @type {import('@playwright/test').Page} */
    let admin;
    /** @type {{ id: string, email: string, password: string }} */
    let user;
    /** @type {Awaited<ReturnType<typeof loginInNewContext>>} */
    let first;
    /** @type {Awaited<ReturnType<typeof loginInNewContext>>} */
    let second;

    test.beforeAll(async ({ browser, baseURL }) => {
        const ctx = await browser.newContext({ baseURL, storageState: AUTH_STATE });
        admin = await ctx.newPage();
        await admin.goto('/auth/v1/admin');
        user = await createUser(admin, 'e2e-sess');
        first = await loginInNewContext(browser, baseURL, user);
        second = await loginInNewContext(browser, baseURL, user);
    });

    test.afterAll(async () => {
        await first?.close();
        await second?.close();
        if (user) await deleteUser(admin, user.id);
        await admin?.context().close();
    });

    test('search by user ID shows both sessions with their details', async ({ page }) => {
        await openSessions(page);
        const all = await listSessions(page);
        const a = all.find(s => s.id === first.info.id);
        expect(a, 'session of the first login').toBeTruthy();
        if (!a) return;

        // default search option is the user ID
        await expect(main(page).getByRole('combobox', { name: 'Search Options' })).toContainText(
            'User ID',
        );
        await searchBox(main(page)).fill(user.id);
        await expect(rows(page)).toHaveCount(2);
        await expect(main(page).getByText('Total: 2', { exact: true })).toBeVisible();
        await expect(main(page).getByText(user.id, { exact: true })).toHaveCount(2);
        // the remote IP is shown as a copy button
        await expect(main(page).getByRole('button', { name: a.remote_ip })).toHaveCount(2);

        await expandAll(page);
        const sid = main(page).getByText(a.id, { exact: true });
        await expect(sid).toBeVisible();
        for (const label of [
            'Session ID',
            'User ID',
            'Expires',
            'Last Seen',
            'State',
            'IP',
            'MFA',
        ]) {
            // "Session ID" etc. are also hidden options of the search popover
            await expect(
                main(page).getByText(label, { exact: true }).locator('visible=true').first(),
            ).toBeVisible();
        }
        await expect(main(page).getByText(fmtTs(a.exp), { exact: true })).toBeVisible();
        await expect(main(page).getByText(a.state, { exact: true }).first()).toBeVisible();
        expect(a.state).toBe('Auth');
        expect(a.is_mfa).toBe(false);
    });

    test('search by session ID, by IP and without match; clearing the search', async ({ page }) => {
        await openSessions(page);
        const total = (await listSessions(page)).length;

        await search(page, 'Session ID', second.info.id);
        await expect(rows(page)).toHaveCount(1);
        await expandAll(page);
        await expect(main(page).getByText(second.info.id, { exact: true })).toBeVisible();
        await expect(main(page).getByText(first.info.id, { exact: true })).toHaveCount(0);

        // a partial session id matches as well (case-insensitive)
        await searchBox(main(page)).fill(second.info.id.slice(2, 12).toLowerCase());
        await expect(rows(page)).not.toHaveCount(0);

        const ip = (await listSessions(page)).find(s => s.id === first.info.id)?.remote_ip || '';
        await search(page, 'IP', ip);
        await expect(main(page).getByText(/Total: \d+/)).not.toHaveText('Total: 0');
        // newest sessions come first by default, so the test user is on the first page
        await expect(main(page).getByText(user.id, { exact: true }).first()).toBeVisible();

        await searchBox(main(page)).fill('no-such-session-xyz');
        await expect(rows(page)).toHaveCount(0);
        await expect(main(page).getByText('Total: 0', { exact: true })).toBeVisible();

        await main(page)
            .getByRole('search')
            .getByRole('button', { name: /delete|clear/i })
            .click();
        await expect(searchBox(main(page))).toHaveValue('');
        await expect(main(page).getByText(`Total: ${total}`, { exact: true })).toBeVisible();
    });

    test('sorts by expiry and by session ID in both directions', async ({ page }) => {
        await openSessions(page);
        await searchBox(main(page)).fill(user.id);
        await expect(rows(page)).toHaveCount(2);
        await expandAll(page);
        const sidA = main(page).getByText(first.info.id, { exact: true });
        const sidB = main(page).getByText(second.info.id, { exact: true });
        await expect(sidA).toBeVisible();
        await expect(sidB).toBeVisible();

        const orderBy = main(page).getByRole('combobox', { name: 'Order by ...' });
        await expect(orderBy).toContainText('Expires');
        // default: latest expiry first, the second login expires last
        const toAsc = main(page).getByRole('button', { name: 'Change sort to ascending' });
        const toDesc = main(page).getByRole('button', { name: 'Change sort to descending' });
        await expect(toAsc).toBeVisible();
        await expect.poll(async () => (await topOf(sidB)) < (await topOf(sidA))).toBe(true);
        await toAsc.click();
        await expect(toDesc).toBeVisible();
        await expect.poll(async () => (await topOf(sidA)) < (await topOf(sidB))).toBe(true);

        await pick(page, orderBy, 'Session ID');
        const [lower, higher] =
            first.info.id.localeCompare(second.info.id) < 0 ? [sidA, sidB] : [sidB, sidA];
        // still ascending
        await expect.poll(async () => (await topOf(lower)) < (await topOf(higher))).toBe(true);
        await toDesc.click();
        await expect(toAsc).toBeVisible();
        await expect.poll(async () => (await topOf(higher)) < (await topOf(lower))).toBe(true);

        // the remaining order options exist
        await orderBy.click();
        const listbox = page.getByRole('listbox').filter({ visible: true }).last();
        for (const opt of ['Expires', 'Last Seen', 'Session ID', 'User ID', 'State', 'IP']) {
            await expect(
                listbox
                    .getByRole('option', { name: opt, exact: true })
                    .or(listbox.getByRole('button', { name: opt, exact: true })),
            ).toBeVisible();
        }
        await page.keyboard.press('Escape');
    });

    test('copies the session IP to the clipboard', async ({ page, baseURL }) => {
        await page
            .context()
            .grantPermissions(['clipboard-read', 'clipboard-write'], { origin: baseURL });
        await openSessions(page);
        await searchBox(main(page)).fill(user.id);
        await expect(rows(page)).toHaveCount(2);
        const ip = (await listSessions(page)).find(s => s.id === first.info.id)?.remote_ip || '';
        await main(page).getByRole('button', { name: ip }).first().click();
        await expect.poll(() => page.evaluate(() => navigator.clipboard.readText())).toBe(ip);
    });

    test('invalidates single sessions from each row, including when expanded', async ({ page }) => {
        await openSessions(page);
        await search(page, 'Session ID', first.info.id);
        await expect(rows(page)).toHaveCount(1);

        // the trash icon in the summary row has no accessible name (FINDING): it follows the
        // IP button of the entry
        const ip = (await listSessions(page)).find(s => s.id === first.info.id)?.remote_ip || '';
        const trash = await rowDeleteButton(
            page,
            main(page).getByRole('button', { name: ip }),
            first.info.id,
        );
        await trash.click();
        await deleteAction(page).click();
        await expect(rows(page)).toHaveCount(0);
        await expect
            .poll(async () => (await first.page.request.get('/auth/v1/oidc/sessioninfo')).status())
            .toBe(401);
        expect((await listSessions(page)).some(s => s.id === first.info.id)).toBe(false);
        // the other session of the same user is untouched
        expect((await second.page.request.get('/auth/v1/oidc/sessioninfo')).status()).toBe(200);

        await searchBox(main(page)).fill(second.info.id);
        await expect(rows(page)).toHaveCount(1);
        await rows(page).first().click();
        await expect(main(page).getByText(second.info.id, { exact: true })).toBeVisible();
        const card = page.locator('.admin-session-card');
        await expect(card.getByRole('button', { name: 'Delete', exact: true })).toHaveCount(1);
        await card.getByRole('button', { name: 'Delete', exact: true }).click();
        await deleteAction(page).click();
        await expect(rows(page)).toHaveCount(0);
        await expect
            .poll(async () => (await second.page.request.get('/auth/v1/oidc/sessioninfo')).status())
            .toBe(401);

        // the user is really logged out: the account page asks for a login again
        await second.page.reload();
        await expect(second.page.getByRole('textbox', { name: /e-?mail/i })).toBeVisible({
            timeout: 20_000,
        });
        await page.reload();
        await searchBox(main(page)).fill(user.id);
        await expect(rows(page)).toHaveCount(0);
    });
});

test.describe('sessions at phone size', () => {
    test('finds and inspects a session of a test user @mobile', async ({
        page,
        browser,
        baseURL,
    }) => {
        await page.goto('/auth/v1/admin');
        const user = await createUser(page, 'e2e-sessm');
        const other = await loginInNewContext(browser, baseURL, user);
        try {
            await openSessions(page);
            await searchBox(main(page)).fill(user.id);
            await expect(rows(page)).toHaveCount(1);
            const collapsed = page.locator('.admin-session-card .admin-expandable > .header');
            const offsets = await collapsed.evaluate(el => {
                const card = el.getBoundingClientRect();
                const center = card.y + card.height / 2;
                return [
                    ...el.querySelectorAll(':scope > button svg, .session-action button svg'),
                ].map(icon => {
                    const rect = icon.getBoundingClientRect();
                    return Math.abs(rect.y + rect.height / 2 - center);
                });
            });
            expect(offsets).toHaveLength(2);
            expect(Math.max(...offsets)).toBeLessThanOrEqual(1);
            await rows(page).first().click();
            const sid = main(page).getByText(other.info.id, { exact: true });
            await sid.scrollIntoViewIfNeeded();
            await expect(sid).toBeInViewport();
            const del = page.locator('.admin-session-card').getByRole('button', {
                name: 'Delete',
                exact: true,
            });
            await del.scrollIntoViewIfNeeded();
            await expect(del).toBeInViewport();
            await del.click();
            await deleteAction(page).click();
            await expect(rows(page)).toHaveCount(0);
            await expect
                .poll(async () =>
                    (await other.page.request.get('/auth/v1/oidc/sessioninfo')).status(),
                )
                .toBe(401);
        } finally {
            await other.close();
            await deleteUser(page, user.id);
        }
    });
});
