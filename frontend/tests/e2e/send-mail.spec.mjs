import { test, expect } from './fixtures.mjs';
import { api } from './helpers/api.mjs';
import { MAILCRAB_URL, unique } from './helpers/env.mjs';
import { waitForMail } from './helpers/mail.mjs';
import { enterTime, chooseOption, showEntryList } from './helpers/ui.mjs';
import { ACCOUNT_EXPECTED_ERRORS } from './helpers/account.mjs';
import {
    berlinTs,
    createMailTarget,
    dateInput,
    guardMassMail,
    mockMassMail,
    nowSecs,
} from './helpers/config.mjs';

// The mass mail dialog of the users page (frontend/src/lib/SendMail.svelte).
// Only the "real send" test lets a mail through, and only to a dedicated group with a single
// fresh user in it (`guardMassMail` aborts anything else). All other tests answer POST /email
// with a mock, so nothing ever reaches real users.

/** @param {import('@playwright/test').Page} page */
async function openCompose(page) {
    await page.goto('/auth/v1/admin/users');
    await showEntryList(page);
    await page.getByRole('button', { name: 'Send E-Mail' }).click();
    const dialog = page.getByRole('dialog');
    await expect(dialog.getByRole('textbox', { name: 'Subject' })).toBeVisible();
    return dialog;
}

/** @param {import('@playwright/test').Locator} dialog */
const editor = dialog => dialog.getByRole('textbox', { name: 'Edit Text' });
/** @param {import('@playwright/test').Locator} dialog */
const filters = dialog => dialog.getByRole('combobox', { name: /^(User Filter|Groups|Roles)$/ });

test.describe('send e-mail', () => {
    // some tests start on the account page to get a session for the API helpers
    test.use({ allowErrors: ACCOUNT_EXPECTED_ERRORS });
    test('opens the compose dialog with safe defaults and closes it again @mobile', async ({
        page,
    }) => {
        const dialog = await openCompose(page);
        await expect(
            dialog.getByRole('checkbox', { name: 'Scheduled Dispatch' }),
        ).not.toBeChecked();
        await expect(filters(dialog)).toHaveCount(1);
        await expect(filters(dialog)).toHaveText('None');
        await expect(dialog.getByText('This E-Mail will be sent to all users.')).toBeVisible();
        await expect(dialog.getByRole('textbox', { name: 'Subject' })).toHaveAttribute(
            'aria-required',
            'true',
        );
        await expect(dialog.getByRole('button', { name: 'Markdown' })).toHaveAttribute(
            'aria-current',
            'step',
        );
        await expect(dialog.getByRole('button', { name: 'Send' })).toBeDisabled();

        await dialog.getByRole('button', { name: 'Close' }).click();
        await expect(dialog).toBeHidden();

        await page.getByRole('button', { name: 'Send E-Mail' }).click();
        await expect(page.getByRole('dialog')).toBeVisible();
        await page.keyboard.press('Escape');
        await expect(page.getByRole('dialog')).toBeHidden();
    });

    test('send stays disabled until subject and body are set', async ({ page }) => {
        const dialog = await openCompose(page);
        const send = dialog.getByRole('button', { name: 'Send' });
        const subject = dialog.getByRole('textbox', { name: 'Subject' });

        await subject.fill('Only a subject');
        await expect(send).toBeDisabled();
        await subject.fill('');
        await expect(subject).toHaveAttribute('aria-invalid', 'true');
        await editor(dialog).fill('Only a body');
        await expect(send).toBeDisabled();
        await subject.fill('Both');
        await expect(subject).toHaveAttribute('aria-invalid', 'false');
        await expect(send).toBeEnabled();
        await editor(dialog).fill('');
        await expect(send).toBeDisabled();
    });

    test('the recipient filter offers groups and roles and explains the selection', async ({
        page,
    }) => {
        // the UI lists both alphabetically
        const groups = (await api(page, 'GET', 'groups'))
            .map((/** @type {any} */ g) => g.name)
            .sort((/** @type {string} */ a, /** @type {string} */ b) => a.localeCompare(b));
        const roles = (await api(page, 'GET', 'roles'))
            .map((/** @type {any} */ r) => r.name)
            .sort((/** @type {string} */ a, /** @type {string} */ b) => a.localeCompare(b));
        const dialog = await openCompose(page);
        const type = filters(dialog).first();

        // the chosen group / role is kept when switching between "In" and "Not in"
        /** @type {Map<readonly string[], string>} */
        const kept = new Map();
        for (const [filter, values] of /** @type {const} */ ([
            ['In Group', groups],
            ['Not in Group', groups],
            ['Has Role', roles],
            ['Has not Role', roles],
        ])) {
            const current = kept.get(values) ?? values[0];
            await chooseOption(page, type, filter);
            await expect(type).toHaveText(filter);
            await expect(filters(dialog)).toHaveCount(2);
            const value = filters(dialog).nth(1);
            await expect(value).toHaveText(current);
            await expect(
                dialog.getByText(
                    `This E-Mail will be sent to all users filtered by: ${filter} ${current}`,
                ),
            ).toBeVisible();
            const last = values[values.length - 1];
            await chooseOption(page, value, last);
            kept.set(values, last);
            await expect(
                dialog.getByText(`filtered by: ${filter} ${last}`, { exact: false }),
            ).toBeVisible();
        }

        await chooseOption(page, type, 'None');
        await expect(filters(dialog)).toHaveCount(1);
        await expect(dialog.getByText('This E-Mail will be sent to all users.')).toBeVisible();
    });

    test('sends a markdown mail to a single test user and it arrives in Mailcrab', async ({
        page,
    }) => {
        test.setTimeout(90_000);
        await page.goto('/auth/v1/account');
        const target = await createMailTarget(page);
        try {
            await guardMassMail(page, target.group.name);
            const dialog = await openCompose(page);

            await chooseOption(page, filters(dialog).first(), 'In Group');
            await chooseOption(page, filters(dialog).nth(1), target.group.name);
            await expect(
                dialog.getByText(`filtered by: In Group ${target.group.name}`, { exact: false }),
            ).toBeVisible();

            const subject = unique('E2E mass mail');
            await dialog.getByRole('textbox', { name: 'Subject' }).fill(subject);
            await editor(dialog).fill('Hello **E2E** reader');
            await dialog.getByRole('button', { name: 'Preview', exact: true }).click();
            await expect(dialog.getByRole('strong').filter({ hasText: 'E2E' })).toBeVisible();

            const since = nowSecs() - 1;
            const res = page.waitForResponse(
                r => r.url().endsWith('/auth/v1/email') && r.request().method() === 'POST',
            );
            await dialog.getByRole('button', { name: 'Send' }).click();
            const req = (await res).request().postDataJSON();
            expect((await res).status()).toBe(200);
            expect(req).toEqual({
                filter_type: 'in_group',
                filter_value: target.group.name,
                content_type: 'markdown',
                subject,
                body: 'Hello **E2E** reader',
            });
            // the dialog closes on its own a few seconds after the success
            await expect(page.getByRole('dialog')).toBeHidden();

            const mail = await waitForMail(page.request, {
                to: target.email,
                subject: new RegExp(subject),
                since,
            });
            // the backend prefixes every subject with the configured product name
            expect(mail.subject).toMatch(new RegExp(`${subject}$`));
            expect(mail.html).toMatch(/<strong>E2E<\/strong>/);
            expect(mail.text).toContain('Hello **E2E** reader');

            // nobody else got it
            const all = await (await page.request.get(`${MAILCRAB_URL}/api/messages`)).json();
            const recipients = all
                // the backend prefixes the subject
                .filter((/** @type {any} */ m) => m.subject.endsWith(subject))
                .flatMap((/** @type {any} */ m) => m.to.map((/** @type {any} */ t) => t.email));
            expect(recipients).toEqual([target.email]);

            // and the backend recorded the job with its filter
            const jobs = await api(page, 'GET', 'email');
            expect(jobs.find((/** @type {any} */ j) => j.subject === subject)).toMatchObject({
                filter_type: 'in_group',
                filter_value: target.group.name,
            });

            // the dialog was reset for the next mail
            await page.getByRole('button', { name: 'Send E-Mail' }).click();
            const again = page.getByRole('dialog');
            await expect(again.getByRole('textbox', { name: 'Subject' })).toHaveValue('');
            await expect(filters(again).first()).toHaveText('None');
        } finally {
            await target.cleanup();
        }
    });

    test('plain text mode sends the text verbatim', async ({ page }) => {
        const sent = await mockMassMail(page);
        const roles = (await api(page, 'GET', 'roles'))
            .map((/** @type {any} */ r) => r.name)
            .sort((/** @type {string} */ a, /** @type {string} */ b) => a.localeCompare(b));
        const dialog = await openCompose(page);
        await chooseOption(page, filters(dialog).first(), 'Has not Role');
        await dialog.getByRole('textbox', { name: 'Subject' }).fill('Plain');
        await dialog.getByRole('button', { name: 'Text', exact: true }).click();
        await expect(dialog.getByRole('button', { name: 'Text', exact: true })).toHaveAttribute(
            'aria-current',
            'step',
        );
        await editor(dialog).fill('Just *plain* text');
        await dialog.getByRole('button', { name: 'Send' }).click();
        await expect(page.getByRole('dialog')).toBeHidden();
        expect(sent).toEqual([
            {
                filter_type: 'has_not_role',
                filter_value: roles[0],
                content_type: 'text',
                subject: 'Plain',
                body: 'Just *plain* text',
            },
        ]);
    });

    test('a scheduled mail to a group keeps the recipient and chosen local time', async ({
        page,
    }) => {
        const sent = await mockMassMail(page);
        const dialog = await openCompose(page);
        await chooseOption(page, filters(dialog).first(), 'In Group');
        const group = await dialog.getByRole('combobox', { name: 'Groups' }).innerText();
        await dialog.getByRole('checkbox', { name: 'Scheduled Dispatch' }).check();
        const date = dateInput(2);
        await dialog.locator('input[type="date"]').fill(date);
        const time = dialog.getByRole('textbox', { name: 'Time', exact: true });
        await enterTime(time, '09:15');
        await time.blur();
        await dialog.getByRole('textbox', { name: 'Subject' }).fill('Later');
        await editor(dialog).fill('Scheduled body');
        await dialog.getByRole('button', { name: 'Send' }).click();
        await expect.poll(() => sent.length).toBe(1);
        expect(sent[0]).toMatchObject({
            filter_type: 'in_group',
            filter_value: group.trim(),
            scheduled: berlinTs(date, '09:15'),
        });
    });

    test('scheduling reveals date and time and is sent with the job', async ({ page }) => {
        const sent = await mockMassMail(page);
        const dialog = await openCompose(page);
        const scheduled = dialog.getByRole('checkbox', { name: 'Scheduled Dispatch' });
        await expect(dialog.locator('input[type="date"]')).toHaveCount(0);
        await scheduled.check();
        await expect(dialog.locator('input[type="date"]')).toHaveValue(dateInput(0));
        await scheduled.uncheck();
        await expect(dialog.locator('input[type="date"]')).toHaveCount(0);
        await scheduled.check();
        await dialog.locator('input[type="date"]').fill(dateInput(2));
        await dialog.getByRole('textbox', { name: 'Subject' }).fill('Later');
        await editor(dialog).fill('Scheduled body');
        await dialog.getByRole('button', { name: 'Send' }).click();
        await expect.poll(() => sent.length).toBe(1);
        expect(sent[0]).toMatchObject({ filter_type: 'none', subject: 'Later' });
        // within a couple of minutes of the chosen day at the current time
        const expected = berlinTs(dateInput(2), '00:00');
        expect(sent[0].scheduled).toBeGreaterThan(expected);
        expect(sent[0].scheduled).toBeLessThan(expected + 86_400 + 120);
    });

    test.describe('editor toolbar', () => {
        // Button name -> markdown it produces around / in front of the selected word.
        const TOOLS = [
            ['Heading 1', '# hello'],
            ['Heading 2', '## hello'],
            ['Heading 3', '### hello'],
            ['Bold', '**hello**'],
            ['Italic', '*hello*'],
            ['Strikethrough', '~~hello~~'],
            ['List', '- hello'],
            ['Numbered List', '1. hello'],
            ['Tasks', '- [ ] hello'],
            ['Code', '`hello`'],
            ['Quote', '> hello'],
            ['Link', '[hello](https://...)'],
        ];

        test('every button inserts its markdown around the selection', async ({ page }) => {
            const dialog = await openCompose(page);
            const ed = editor(dialog);
            for (const [name, expected] of TOOLS) {
                await ed.fill('hello');
                await ed.press('ControlOrMeta+a');
                // FINDING: the toolbar buttons are only named through the `title` of an inner
                // div; they work, but there is no toolbar role and no keyboard shortcut.
                await dialog.getByRole('button', { name, exact: true }).click();
                await expect(ed, name).toHaveText(expected);
            }
        });

        test('the preview renders the markdown', async ({ page }) => {
            const dialog = await openCompose(page);
            const ed = editor(dialog);
            await ed.fill('hello');
            await ed.press('ControlOrMeta+a');
            await dialog.getByRole('button', { name: 'Heading 2', exact: true }).click();
            const edit = dialog.getByRole('button', { name: 'Edit', exact: true });
            const preview = dialog.getByRole('button', { name: 'Preview', exact: true });
            await expect(edit).toHaveAttribute('aria-pressed', 'true');
            await preview.click();
            await expect(preview).toHaveAttribute('aria-pressed', 'true');
            await expect(edit).toHaveAttribute('aria-pressed', 'false');
            await expect(dialog.getByRole('heading', { name: 'hello', level: 2 })).toBeVisible();
            await edit.click();
            await expect(edit).toHaveAttribute('aria-pressed', 'true');

            await ed.fill('a word');
            await ed.press('ControlOrMeta+a');
            await dialog.getByRole('button', { name: 'Italic', exact: true }).click();
            await dialog.getByRole('button', { name: 'Preview', exact: true }).click();
            await expect(dialog.getByRole('emphasis')).toHaveText('a word');
            await dialog.getByRole('button', { name: 'Edit', exact: true }).click();

            await ed.fill('item');
            await ed.press('ControlOrMeta+a');
            await dialog.getByRole('button', { name: 'List', exact: true }).click();
            await dialog.getByRole('button', { name: 'Preview', exact: true }).click();
            await expect(dialog.getByRole('listitem').filter({ hasText: 'item' })).toBeVisible();
            await dialog.getByRole('button', { name: 'Edit', exact: true }).click();

            await ed.fill('site');
            await ed.press('ControlOrMeta+a');
            await dialog.getByRole('button', { name: 'Link', exact: true }).click();
            await dialog.getByRole('button', { name: 'Preview', exact: true }).click();
            await expect(dialog.getByRole('link', { name: 'site' })).toHaveAttribute(
                'href',
                'https://...',
            );
        });

        test('the preview is a toggle at phone size @mobile', async ({ page, isMobile }) => {
            test.skip(!isMobile, 'phone layout only');
            const sent = await mockMassMail(page);
            const dialog = await openCompose(page);
            await dialog.getByRole('textbox', { name: 'Subject' }).fill('Phone');
            await editor(dialog).fill('**phone** body');
            const preview = dialog.getByRole('button', { name: 'Preview', exact: true });
            await preview.click();
            await expect(editor(dialog)).toHaveCount(0);
            await expect(dialog.getByRole('strong')).toHaveText('phone');
            await dialog.getByRole('button', { name: 'Edit', exact: true }).click();
            await expect(editor(dialog)).toHaveText('**phone** body');
            await dialog.getByRole('button', { name: 'Send' }).click();
            await expect(page.getByRole('dialog')).toBeHidden();
            expect(sent).toHaveLength(1);
            expect(sent[0]).toMatchObject({ subject: 'Phone', body: '**phone** body' });
        });
    });
});
