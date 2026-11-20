import { enterTime } from './helpers/ui.mjs';
// Characterization tests for events: the archive page with its date / time / level / type filters,
// search and sort, the live events side panel (level selector, "Test" button) and real events
// caused by a test user (registration, password reset, one failed login).
import { test, expect } from './fixtures.mjs';
import { api } from './helpers/api.mjs';
import {
    createUser,
    deleteUser,
    fmtTs,
    loginInNewContext,
    NO_SESSION,
    pick,
    searchBox,
    topOf,
} from './helpers/ops.mjs';

const ARCHIVE = '/auth/v1/admin/events';
const TEST_TEXT = 'This is a Test-Event';

/** @param {import('@playwright/test').Page} page */
const main = page => page.getByRole('main');

/**
 * @typedef {{ id: string, timestamp: number, level: string, typ: string, ip?: string,
 *   data?: number, text?: string }} Ev
 */

/**
 * Events of the last `seconds` from the archive API.
 * @param {import('@playwright/test').Page} page
 * @param {{ seconds?: number, level?: string, typ?: string }} [opts]
 * @returns {Promise<Ev[]>}
 */
async function recentEvents(page, { seconds = 600, level = 'info', typ = undefined } = {}) {
    return api(page, 'POST', 'events', {
        from: Math.floor(Date.now() / 1000) - seconds,
        level,
        typ,
    });
}

/**
 * Creates a test event through the API and returns it (the one with the caller's IP; the dev
 * backend additionally emits one dummy event of every type from 0.0.0.0).
 * @param {import('@playwright/test').Page} page
 */
async function createTestEvent(page) {
    const since = Date.now() - 1000;
    await api(page, 'POST', 'events/test');
    /** @type {Ev | undefined} */
    let found;
    await expect
        .poll(async () => {
            const evs = await recentEvents(page, { seconds: 60, typ: 'Test' });
            found = evs
                .filter(e => e.ip !== '0.0.0.0' && e.timestamp >= since)
                .sort((a, b) => b.timestamp - a.timestamp)[0];
            return !!found;
        })
        .toBe(true);
    return /** @type {Ev} */ (found);
}

/** Local `YYYY-MM-DD` and `HH:MM` of a unix timestamp (ms) in the browser time zone. */
async function localDateTime(
    /** @type {import('@playwright/test').Page} */ page,
    /** @type {number} */ tsMs,
) {
    return page.evaluate(ts => {
        const d = new Date(ts);
        const p = (/** @type {number} */ n) => String(n).padStart(2, '0');
        return {
            date: `${d.getFullYear()}-${p(d.getMonth() + 1)}-${p(d.getDate())}`,
            time: `${p(d.getHours())}:${p(d.getMinutes())}`,
        };
    }, tsMs);
}

/**
 * The date and time inputs of the archive. Date and time of a pair share the same name
 * ("From" / "Until", FINDING), so they are told apart by order.
 * @param {import('@playwright/test').Page} page
 * @param {'From' | 'Until'} which
 */
function dateTime(page, which) {
    return {
        date: main(page).getByRole('textbox', { name: which, exact: true }),
        time: main(page).locator(`input[name="${which.toLowerCase()}_time"]`),
    };
}

/**
 * Sets a date + time filter and waits for the archive to reload.
 * @param {import('@playwright/test').Page} page
 * @param {'From' | 'Until'} which
 * @param {{ date: string, time: string }} value
 */
async function setFilter(page, which, { date, time }) {
    const f = dateTime(page, which);
    const reload = page.waitForResponse(
        (/** @type {import('@playwright/test').Response} */ r) =>
            r.url().endsWith('/auth/v1/events') && r.request().method() === 'POST',
    );
    await f.date.fill(date);
    await enterTime(f.time, time);
    await f.time.blur();
    await reload;
}

/**
 * An archive entry: the event whose text contains `text` (and optionally `also`).
 * @param {import('@playwright/test').Page} page
 * @param {string} text
 * @param {string} [also]
 */
function eventEntry(page, text, also) {
    // the type is shown readable: "NewUserRegistered" -> "New User Registered"
    const readable = (/** @type {string} */ s) =>
        /^[A-Z][a-z]+[A-Z]\w*$/.test(s) ? s.replace(/([a-z])([A-Z])/g, '$1 $2') : s;
    const loc = main(page)
        .getByRole('article')
        .filter({ hasText: readable(text) });
    return also ? loc.filter({ hasText: readable(also) }) : loc;
}

test.describe('events archive', () => {
    // Below 1025px the live side panel is not rendered, so every event in `main` belongs to the
    // archive (the panel has no landmark / name to scope by, FINDING).
    test.use({ viewport: { width: 1000, height: 900 } });

    test('loads with default filters starting one day back', async ({ page }) => {
        const req = page.waitForRequest(
            r => r.url().endsWith('/auth/v1/events') && r.method() === 'POST',
        );
        await page.goto(ARCHIVE);
        const payload = (await req).postDataJSON();
        expect(payload.level).toBe('info');
        expect(payload.typ).toBeUndefined();
        expect(payload.until).toBeUndefined();

        const dayBack = await localDateTime(page, Date.now() - 86_400_000);
        const from = dateTime(page, 'From');
        await expect(from.date).toHaveValue(dayBack.date);
        await expect(from.time).toHaveValue(/^\d{2}:\d{2}$/);
        const until = dateTime(page, 'Until');
        await expect(until.date).toHaveValue('');
        await expect(
            main(page).getByRole('button', { name: 'Event Level', exact: true }),
        ).toContainText('All levels');
        await expect(main(page).getByRole('combobox', { name: 'Event Type' })).toContainText('-');
        await expect(main(page).getByRole('combobox', { name: 'Order by ...' })).toContainText(
            'Timestamp',
        );
        await expect(main(page).getByRole('combobox', { name: 'Search Options' })).toContainText(
            'IP',
        );
    });

    test('filters by time range, level and type, searches and sorts real events', async ({
        page,
    }) => {
        await page.goto(ARCHIVE);
        // a registration and an admin password reset of a fresh user create two events with the
        // user's unique e-mail
        const user = await createUser(page, 'e2e-evt');
        try {
            const testEv = await createTestEvent(page);
            const start = await localDateTime(page, Date.now() - 10 * 60_000);
            await setFilter(page, 'From', start);

            const registered = eventEntry(page, user.email, 'NewUserRegistered');
            const reset = eventEntry(page, `for user ${user.email}`, 'UserPasswordReset');
            const testEntry = eventEntry(page, TEST_TEXT, fmtTs(testEv.timestamp / 1000)).filter({
                hasText: testEv.ip,
            });
            await expect(registered).toBeVisible();
            await expect(reset).toBeVisible();
            await expect(testEntry.first()).toBeVisible();
            // the test event shows the IP it came from
            await expect(
                testEntry.first().getByRole('button', { name: 'IP Address' }),
            ).toContainText(testEv.ip || '');

            // level: the registration is "info", the admin password reset "notice"
            const level = main(page).getByRole('button', { name: 'Event Level', exact: true });
            await level.click();
            const levels = page.getByRole('group', { name: 'Event Level' });
            await levels.getByRole('checkbox', { name: 'All levels' }).uncheck();
            await levels.getByRole('checkbox', { name: 'notice', exact: true }).check();
            await expect(reset).toBeVisible();
            await expect(registered).toBeHidden();
            await expect(testEntry).toHaveCount(0);
            await levels.getByRole('checkbox', { name: 'info', exact: true }).check();
            await levels.getByRole('checkbox', { name: 'notice', exact: true }).uncheck();
            await expect(reset).toBeHidden();
            await expect(registered).toBeVisible();
            await levels.getByRole('checkbox', { name: 'critical', exact: true }).check();
            await expect(reset).toBeHidden();
            await expect(registered).toBeVisible();
            await levels.getByRole('checkbox', { name: 'All levels' }).check();
            await page.keyboard.press('Escape');

            // type
            const type = main(page).getByRole('combobox', { name: 'Event Type' });
            await pick(page, type, 'NewUserRegistered');
            await expect(registered).toBeVisible();
            await expect(reset).toBeHidden();
            await expect(testEntry).toHaveCount(0);
            await pick(page, type, 'Test');
            await expect(testEntry.first()).toBeVisible();
            await expect(registered).toBeHidden();
            await pick(page, type, '-');
            await expect(reset).toBeVisible();

            // until: one minute after the start hides everything that happened now
            const untilStart = await localDateTime(page, Date.now() - 9 * 60_000);
            await setFilter(page, 'Until', untilStart);
            await expect(registered).toBeHidden();
            await expect(testEntry).toHaveCount(0);
            // the clear button of the until filter (named "Delete", like the search clear button)
            const reload = page.waitForResponse(
                (/** @type {import('@playwright/test').Response} */ r) =>
                    r.url().endsWith('/auth/v1/events') && r.request().method() === 'POST',
            );
            await main(page).getByRole('button', { name: 'Delete', exact: true }).last().click();
            await reload;
            await expect(dateTime(page, 'Until').date).toHaveValue('');
            await expect(registered).toBeVisible();

            // search by content and by IP
            const searchOpts = main(page).getByRole('combobox', { name: 'Search Options' });
            await pick(page, searchOpts, 'Content');
            await searchBox(main(page)).fill(user.email);
            await expect(registered).toBeVisible();
            await expect(reset).toBeVisible();
            await expect(testEntry).toHaveCount(0);
            await searchBox(main(page)).fill('test-event');
            await expect(testEntry.first()).toBeVisible();
            await expect(registered).toBeHidden();
            await pick(page, searchOpts, 'IP');
            await searchBox(main(page)).fill(testEv.ip || '');
            await expect(testEntry.first()).toBeVisible();
            await searchBox(main(page)).fill('203.0.113.254');
            // the admin UI shows the translated "No Entries" instead of a fixed English text
            await expect(main(page).getByText(/^(No events found|No Entries)$/)).toBeVisible();
            await searchBox(main(page)).fill('');

            // sort: newest first by default
            const toAsc = main(page).getByRole('button', { name: 'Change sort to ascending' });
            const toDesc = main(page).getByRole('button', { name: 'Change sort to descending' });
            await expect(toAsc).toBeVisible();
            await expect
                .poll(async () => (await topOf(reset)) < (await topOf(registered)))
                .toBe(true);
            await toAsc.click();
            await expect
                .poll(async () => (await topOf(registered)) < (await topOf(reset)))
                .toBe(true);
            const orderBy = main(page).getByRole('combobox', { name: 'Order by ...' });
            // by type, still ascending: NewUserRegistered < UserPasswordReset
            await pick(page, orderBy, 'Type');
            await expect
                .poll(async () => (await topOf(registered)) < (await topOf(reset)))
                .toBe(true);
            await toDesc.click();
            await expect
                .poll(async () => (await topOf(reset)) < (await topOf(registered)))
                .toBe(true);
            // by level, descending: notice before info
            await pick(page, orderBy, 'Level');
            await expect
                .poll(async () => (await topOf(reset)) < (await topOf(registered)))
                .toBe(true);
            await toAsc.click();
            await expect
                .poll(async () => (await topOf(registered)) < (await topOf(reset)))
                .toBe(true);
        } finally {
            await deleteUser(page, user.id);
        }
    });

    test('an event in the "From" minute is included', async ({ page }) => {
        await page.goto(ARCHIVE);
        const ev = await createTestEvent(page);
        await setFilter(page, 'From', await localDateTime(page, ev.timestamp));
        await expect(
            eventEntry(page, TEST_TEXT, fmtTs(ev.timestamp / 1000))
                .filter({ hasText: ev.ip })
                .first(),
        ).toBeVisible({
            timeout: 3_000,
        });
    });

    test('a failed login shows up as event', async ({ page, browser, baseURL }) => {
        await page.goto(ARCHIVE);
        const user = await createUser(page, 'e2e-evtl');
        const ctx = await browser.newContext({ baseURL, storageState: NO_SESSION });
        const other = await ctx.newPage();
        try {
            const since = Date.now();
            // exactly one wrong password (the dev cred-stuffing threshold is 3)
            await other.goto('/auth/v1/account');
            const email = other.getByRole('textbox', { name: /e-?mail/i });
            await email.fill(user.email);
            await email.press('Enter');
            const pwd = other.getByLabel('Password', { exact: true });
            await expect(pwd).toBeVisible({ timeout: 20_000 });
            await pwd.fill('Definitely-Wrong-123');
            await pwd.press('Enter');
            /** @type {Ev | undefined} */
            let failed;
            await expect
                .poll(
                    async () => {
                        const evs = await recentEvents(page, {
                            seconds: 120,
                            typ: 'InvalidLogins',
                        });
                        failed = evs.find(e => e.timestamp >= since);
                        return !!failed;
                    },
                    { timeout: 20_000 },
                )
                .toBe(true);
            // a successful login resets the failed-login counter of this IP again
            await other.context().close();
            const ok = await loginInNewContext(browser, baseURL, user);
            await ok.close();

            const ev = /** @type {Ev} */ (failed);
            await setFilter(page, 'From', await localDateTime(page, since - 2 * 60_000));
            await pick(
                page,
                main(page).getByRole('combobox', { name: 'Event Type' }),
                'InvalidLogins',
            );
            const entry = eventEntry(page, 'InvalidLogins', fmtTs(ev.timestamp / 1000));
            await expect(entry.first()).toBeVisible();
            // the admin UI shows the count next to the type, without the ": " of the original
            await expect(entry.first()).toContainText(
                new RegExp(`Invalid Logins:?\\s*${ev.data}\\b`),
            );
            await expect(entry.first().getByRole('button', { name: 'IP Address' })).toContainText(
                ev.ip || '',
            );
        } finally {
            await ctx.close().catch(() => {});
            await deleteUser(page, user.id);
        }
    });

    test('copies the IP of an event to the clipboard', async ({ page, baseURL }) => {
        await page
            .context()
            .grantPermissions(['clipboard-read', 'clipboard-write'], { origin: baseURL });
        await page.goto(ARCHIVE);
        const ev = await createTestEvent(page);
        await setFilter(page, 'From', await localDateTime(page, Date.now() - 5 * 60_000));
        await pick(page, main(page).getByRole('combobox', { name: 'Event Type' }), 'Test');
        const ipBtn = eventEntry(page, TEST_TEXT, fmtTs(ev.timestamp / 1000))
            .filter({ hasText: ev.ip })
            .first()
            .getByRole('button', { name: 'IP Address' });
        await ipBtn.hover();
        const tip = page.getByText('Copy value to clipboard').first();
        await expect(tip).toBeVisible();
        // the tooltip springs to the pointer and settles there, not far away from the button
        await expect
            .poll(async () => {
                const [b, t] = [await ipBtn.boundingBox(), await tip.boundingBox()];
                return Math.abs((t?.y ?? 0) - (b?.y ?? 0));
            })
            .toBeLessThan(120);
        await ipBtn.click();
        await expect.poll(() => page.evaluate(() => navigator.clipboard.readText())).toBe(ev.ip);
        await expect(page.getByText('Value has been copied to clipboard').first()).toBeVisible();
    });
});

/**
 * Opens the live events panel next to the admin page. It starts closed and is opened with the
 * "Events" button of the admin header, which exposes its state with `aria-expanded`.
 * @param {import('@playwright/test').Page} page
 * @param {string} path
 */
async function openWithEventsPanel(page, path) {
    await page.goto(path);
    const toggle = page.getByRole('button', { name: 'Events', exact: true });
    await expect(toggle).toBeVisible();
    if ((await toggle.getAttribute('aria-expanded')) !== 'true') await toggle.click();
    await expect(toggle).toHaveAttribute('aria-expanded', 'true');
}

test.describe('live events panel', () => {
    test('collapses at medium widths and can still be opened beside the page', async ({ page }) => {
        await openWithEventsPanel(page, '/auth/v1/admin/sessions');
        const toggle = page.getByRole('button', { name: 'Events', exact: true });
        const panel = page.locator('#admin-events-panel');
        await expect(panel).toBeVisible();

        await page.setViewportSize({ width: 1180, height: 900 });
        await expect(toggle).toHaveAttribute('aria-expanded', 'false');
        await expect(panel).toBeHidden();

        await toggle.click();
        await expect(panel).toBeVisible();
        const [content, events] = await Promise.all([
            page.locator('.admin-workspace > .content').boundingBox(),
            panel.boundingBox(),
        ]);
        expect(content && events && content.x + content.width <= events.x).toBeTruthy();

        await page.setViewportSize({ width: 960, height: 900 });
        await expect(toggle).toBeHidden();
        await expect(panel).toBeHidden();
    });

    test('is shown next to every admin page on wide screens', async ({ page }) => {
        await openWithEventsPanel(page, '/auth/v1/admin/sessions');
        await expect(
            main(page).getByRole('button', { name: 'Event Level', exact: true }),
        ).toBeVisible();
        await expect(main(page).getByRole('button', { name: 'Send test event' })).toBeVisible();
        // the stream delivers the latest events right away
        await expect(
            main(page)
                .getByText(/\d{4}\/\d{2}\/\d{2} \d{2}:\d{2}:\d{2}/)
                .first(),
        ).toBeVisible();
    });

    test('"Test" creates a test event that appears live', async ({ page }) => {
        await openWithEventsPanel(page, '/auth/v1/admin/sessions');
        const since = Date.now() - 1000;
        await main(page).getByRole('button', { name: 'Send test event' }).click();
        /** @type {Ev | undefined} */
        let ev;
        await expect
            .poll(async () => {
                const evs = await recentEvents(page, { seconds: 60, typ: 'Test' });
                ev = evs.find(e => e.ip !== '0.0.0.0' && e.timestamp >= since);
                return !!ev;
            })
            .toBe(true);
        const live = eventEntry(
            page,
            TEST_TEXT,
            fmtTs(/** @type {Ev} */ (ev).timestamp / 1000),
        ).filter({ hasText: /** @type {Ev} */ (ev).ip });
        await expect(live.first()).toBeVisible();
        await expect(live.first().getByRole('button', { name: 'IP Address' })).toContainText(
            /** @type {Ev} */ (ev).ip || '',
        );
    });

    test('the level selector filters the stream and is remembered', async ({ page }) => {
        await openWithEventsPanel(page, '/auth/v1/admin/sessions');
        const level = main(page).getByRole('button', { name: 'Event Level', exact: true });
        await expect(level).toContainText('All levels');
        // an info event of a fresh user registration
        const user = await createUser(page, 'e2e-evtp');
        try {
            const registered = eventEntry(page, user.email, 'NewUserRegistered');
            await expect(registered).toBeVisible();
            await level.click();
            const levels = page.getByRole('group', { name: 'Event Level' });
            await levels.getByRole('checkbox', { name: 'All levels' }).uncheck();
            await levels.getByRole('checkbox', { name: 'notice', exact: true }).check();
            await levels.getByRole('checkbox', { name: 'critical', exact: true }).check();
            await expect(level).toContainText('2 / 4');
            await expect(registered).toBeHidden();

            await page.reload();
            await expect(level).toContainText('2 / 4');
            await level.click();
            await expect(
                levels.getByRole('checkbox', { name: 'notice', exact: true }),
            ).toBeChecked();
            await expect(
                levels.getByRole('checkbox', { name: 'critical', exact: true }),
            ).toBeChecked();
            await expect(
                levels.getByRole('checkbox', { name: 'warning', exact: true }),
            ).not.toBeChecked();
            await levels.getByRole('checkbox', { name: 'All levels' }).check();
            await expect(registered).toBeVisible();
        } finally {
            await page.evaluate(() => localStorage.removeItem('eventLevels'));
            await deleteUser(page, user.id);
        }
    });
});

test.describe('events at phone size', () => {
    test('filters the archive by type @mobile', async ({ page }) => {
        await page.goto(ARCHIVE);
        const ev = await createTestEvent(page);
        await setFilter(page, 'From', await localDateTime(page, Date.now() - 5 * 60_000));
        await pick(page, main(page).getByRole('combobox', { name: 'Event Type' }), 'Test');
        const entry = eventEntry(page, TEST_TEXT, fmtTs(ev.timestamp / 1000))
            .filter({ hasText: ev.ip })
            .first();
        await entry.scrollIntoViewIfNeeded();
        await expect(entry).toBeInViewport();
        await pick(page, main(page).getByRole('combobox', { name: 'Event Type' }), 'JwksRotated');
        await expect(eventEntry(page, TEST_TEXT)).toHaveCount(0);
    });
});
