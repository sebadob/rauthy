import { deleteAction } from './helpers/entities.mjs';
import { enterTime } from './helpers/ui.mjs';
// Characterization tests for the IP blacklist: list, add dialog (validation, cancel, success),
// search, sort, copy and delete. Only documentation ranges (RFC 5737) are ever blacklisted, never
// the address the browser connects from.
import { test, expect } from './fixtures.mjs';
import { api } from './helpers/api.mjs';
import { rowDeleteButton, searchBox, fmtTs, pick, topOf } from './helpers/ops.mjs';

const BLACKLIST = '/auth/v1/admin/blacklist';

/** @param {import('@playwright/test').Page} page */
const main = page => page.getByRole('main');

/** A random address from one of the documentation ranges. */
function docIp(/** @type {'203.0.113' | '198.51.100'} */ net) {
    return `${net}.${1 + Math.floor(Math.random() * 254)}`;
}

/**
 * @param {import('@playwright/test').Page} page
 * @returns {Promise<{ ip: string, exp: number }[]>}
 */
async function listBlacklist(page) {
    return (await api(page, 'GET', 'blacklist')).ips;
}

/** @param {import('@playwright/test').Page} page @param {string[]} ips */
async function cleanup(page, ips) {
    for (const ip of ips) {
        try {
            await api(page, 'DELETE', `blacklist/${ip}`);
        } catch {
            // already removed by the test
        }
    }
}

/** `YYYY-MM-DD` of today + `days` in the browser time zone. */
async function dateInDays(/** @type {import('@playwright/test').Page} */ page, days = 1) {
    return page.evaluate(d => {
        const t = new Date(Date.now() + d * 86400_000);
        const p = (/** @type {number} */ n) => String(n).padStart(2, '0');
        return `${t.getFullYear()}-${p(t.getMonth() + 1)}-${p(t.getDate())}`;
    }, days);
}

/**
 * Unix timestamp of a local date + time, as the browser interprets it.
 * @param {import('@playwright/test').Page} page
 * @param {string} date
 * @param {string} time
 */
async function localTs(page, date, time) {
    return page.evaluate(([d, t]) => Date.parse(`${d}T${t}`) / 1000, [date, time]);
}

/** Opens the "Add New" dialog. */
async function openAdd(/** @type {import('@playwright/test').Page} */ page) {
    await main(page).getByRole('button', { name: 'Add New' }).click();
    const dialog = page.getByRole('dialog');
    await expect(dialog).toBeVisible();
    return dialog;
}

/**
 * Fills the add dialog.
 * @param {import('@playwright/test').Locator} dialog
 * @param {{ ip: string, date?: string, time?: string }} v
 */
async function fillAdd(dialog, { ip, date, time }) {
    await dialog.getByRole('textbox', { name: 'IP', exact: true }).fill(ip);
    if (date) await dialog.locator('input[type="date"]').fill(date);
    if (time) await enterTime(dialog.getByRole('textbox', { name: 'Time', exact: true }), time);
}

test.describe('ip blacklist', () => {
    test('lists the blacklisted IPs from the backend', async ({ page }) => {
        await page.goto(BLACKLIST);
        await expect(main(page).getByRole('button', { name: 'Add New' })).toBeVisible();
        const ips = await listBlacklist(page);
        if (ips.length === 0) {
            await expect(main(page).getByText('No Entries', { exact: true })).toBeVisible();
        }
        for (const e of ips.slice(0, 30)) {
            await expect(main(page).getByRole('button', { name: e.ip, exact: true })).toBeVisible();
        }
    });

    test('adds two IPs with expiry, searches, sorts, copies and deletes them', async ({
        page,
        baseURL,
    }) => {
        await page
            .context()
            .grantPermissions(['clipboard-read', 'clipboard-write'], { origin: baseURL });
        const ip1 = docIp('203.0.113');
        const ip2 = docIp('198.51.100');
        await page.goto(BLACKLIST);
        try {
            const date1 = await dateInDays(page, 1);
            let dialog = await openAdd(page);
            // the focus moves into the dialog (upstream Modal focuses "Close" first, FINDING)
            await expect(dialog.locator(':focus')).toHaveCount(1);
            await fillAdd(dialog, { ip: ip1, date: date1, time: '12:30' });
            await dialog.getByRole('button', { name: 'Save' }).click();
            await expect(dialog).toBeHidden();
            const entry1 = main(page).getByRole('button', { name: ip1, exact: true });
            await expect(entry1).toBeVisible();

            const exp1 = await localTs(page, date1, '12:30');
            const stored1 = (await listBlacklist(page)).find(e => e.ip === ip1);
            expect(stored1?.exp).toBe(exp1);
            await expect(main(page).getByText(fmtTs(exp1), { exact: true })).toBeVisible();

            dialog = await openAdd(page);
            await fillAdd(dialog, { ip: ip2, date: await dateInDays(page, 2), time: '08:05' });
            await dialog.getByRole('button', { name: 'Save' }).click();
            await expect(dialog).toBeHidden();
            const entry2 = main(page).getByRole('button', { name: ip2, exact: true });
            await expect(entry2).toBeVisible();
            expect((await listBlacklist(page)).map(e => e.ip)).toEqual(
                expect.arrayContaining([ip1, ip2]),
            );

            // reload: the entries are persisted
            await page.reload();
            await expect(entry1).toBeVisible();
            await expect(entry2).toBeVisible();

            // search
            await searchBox(main(page)).fill('203.0.113.');
            await expect(entry1).toBeVisible();
            await expect(entry2).toBeHidden();
            await searchBox(main(page)).fill(ip2);
            await expect(entry2).toBeVisible();
            await expect(entry1).toBeHidden();
            await searchBox(main(page)).fill('');
            await expect(entry1).toBeVisible();
            await expect(entry2).toBeVisible();

            // sort: ascending by IP first ("198..." before "203...")
            const orderBy = main(page).getByRole('combobox', { name: 'Order by ...' });
            await expect(orderBy).toContainText('IP');
            await pick(page, orderBy, 'IP');
            await expect.poll(async () => (await topOf(entry2)) < (await topOf(entry1))).toBe(true);
            await main(page).getByRole('button', { name: 'Change sort to descending' }).click();
            await expect.poll(async () => (await topOf(entry1)) < (await topOf(entry2))).toBe(true);
            await main(page).getByRole('button', { name: 'Change sort to ascending' }).click();
            await expect.poll(async () => (await topOf(entry2)) < (await topOf(entry1))).toBe(true);

            // a click on the IP copies it
            await entry1.click();
            await expect.poll(() => page.evaluate(() => navigator.clipboard.readText())).toBe(ip1);

            // delete (the delete icon has no accessible name, FINDING)
            await (await rowDeleteButton(page, entry1, ip1)).click();
            await deleteAction(page).click();
            await expect(entry1).toBeHidden();
            await expect(entry2).toBeVisible();
            await expect
                .poll(async () => (await listBlacklist(page)).some(e => e.ip === ip1))
                .toBe(false);
            await (await rowDeleteButton(page, entry2, ip2)).click();
            await deleteAction(page).click();
            await expect(entry2).toBeHidden();
            await expect
                .poll(async () => (await listBlacklist(page)).some(e => e.ip === ip2))
                .toBe(false);
        } finally {
            await cleanup(page, [ip1, ip2]);
        }
    });

    test.describe('validation', () => {
        // the backend rejects a malformed IP with a 400, which the dialog shows
        test.use({ allowErrors: [/http 4\d\d POST \/auth\/v1\/blacklist$/] });

        test('an empty or malformed IP is rejected and nothing is stored', async ({ page }) => {
            await page.goto(BLACKLIST);
            const before = await listBlacklist(page);
            const dialog = await openAdd(page);
            const ipInput = dialog.getByRole('textbox', { name: 'IP', exact: true });

            // empty: the required field blocks the submit
            await dialog.getByRole('button', { name: 'Save' }).click();
            await expect(ipInput).toHaveAttribute('aria-invalid', 'true');
            await expect(dialog).toBeVisible();

            // malformed: rejected with an error message, dialog stays open
            await fillAdd(dialog, {
                ip: '203.0.113.999',
                date: await dateInDays(page, 1),
                time: '10:00',
            });
            await dialog.getByRole('button', { name: 'Save' }).click();
            await expect(
                dialog.getByText(/invalid|error|parse|deserial|bad request/i).first(),
            ).toBeVisible();
            await expect(dialog).toBeVisible();
            await fillAdd(dialog, { ip: 'not-an-ip' });
            await dialog.getByRole('button', { name: 'Save' }).click();
            await expect(
                dialog.getByText(/invalid|error|parse|deserial|bad request/i).first(),
            ).toBeVisible();
            expect(await listBlacklist(page)).toHaveLength(before.length);
        });
    });

    test('the add dialog can be cancelled without storing anything', async ({ page }) => {
        const ip = docIp('203.0.113');
        await page.goto(BLACKLIST);
        try {
            let dialog = await openAdd(page);
            await fillAdd(dialog, { ip, date: await dateInDays(page, 1), time: '09:00' });
            await dialog.getByRole('button', { name: 'Close' }).click();
            await expect(dialog).toBeHidden();
            dialog = await openAdd(page);
            await page.keyboard.press('Escape');
            await expect(dialog).toBeHidden();
            await expect(main(page).getByRole('button', { name: ip, exact: true })).toHaveCount(0);
            expect((await listBlacklist(page)).some(e => e.ip === ip)).toBe(false);
        } finally {
            await cleanup(page, [ip]);
        }
    });

    test('saving with the pre-filled expiry gives feedback', async ({ page }) => {
        const ip = docIp('198.51.100');
        await page.goto(BLACKLIST);
        try {
            const dialog = await openAdd(page);
            await fillAdd(dialog, { ip });
            await dialog.getByRole('button', { name: 'Save' }).click();
            await expect
                .poll(async () => (await listBlacklist(page)).some(e => e.ip === ip))
                .toBe(true);
        } finally {
            await cleanup(page, [ip]);
        }
    });

    test('deleting an entry asks for confirmation', async ({ page }) => {
        const ip = docIp('203.0.113');
        await page.goto(BLACKLIST);
        await api(page, 'POST', 'blacklist', { ip, exp: Math.floor(Date.now() / 1000) + 3600 });
        try {
            await page.reload();
            const entry = main(page).getByRole('button', { name: ip, exact: true });
            await (await rowDeleteButton(page, entry, ip)).click();
            const confirm = page.getByRole('dialog').or(page.getByRole('alertdialog'));
            await expect(confirm).toBeVisible({ timeout: 3_000 });
            await confirm.getByRole('button', { name: /cancel/i }).click();
            await expect(entry).toBeVisible();
        } finally {
            await cleanup(page, [ip]);
        }
    });

    test('adds and removes an IP at phone size @mobile', async ({ page }) => {
        const ip = docIp('198.51.100');
        await page.goto(BLACKLIST);
        try {
            const dialog = await openAdd(page);
            await fillAdd(dialog, { ip, date: await dateInDays(page, 1), time: '18:45' });
            const save = dialog.getByRole('button', { name: 'Save' });
            await save.scrollIntoViewIfNeeded();
            await expect(save).toBeInViewport();
            await save.click();
            await expect(dialog).toBeHidden();
            const entry = main(page).getByRole('button', { name: ip, exact: true });
            await expect(entry).toBeVisible();
            const del = await rowDeleteButton(page, entry, ip);
            const offset = await del.evaluate(button => {
                const card = button.closest('.blacklisted')?.getBoundingClientRect();
                const icon = button.querySelector('svg')?.getBoundingClientRect();
                if (!card || !icon) throw new Error('Missing blacklist card or delete icon');
                return Math.abs(card.y + card.height / 2 - icon.y - icon.height / 2);
            });
            expect(offset).toBeLessThanOrEqual(1);
            await del.scrollIntoViewIfNeeded();
            await expect(del).toBeInViewport();
            await del.click();
            await deleteAction(page).click();
            await expect(entry).toBeHidden();
            await expect
                .poll(async () => (await listBlacklist(page)).some(e => e.ip === ip))
                .toBe(false);
        } finally {
            await cleanup(page, [ip]);
        }
    });
});
