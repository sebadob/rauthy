import { test, expect } from './fixtures.mjs';
import { tab } from './helpers/clients.mjs';
import { ADMIN_EMAIL } from './helpers/env.mjs';
import { detailTab } from './helpers/users.mjs';
import { chooseOption } from './helpers/ui.mjs';

test('the compact language menu scrolls without visible scrollbars @webkit', async ({ page }) => {
    await page.goto('/auth/v1/');
    await page.getByRole('combobox', { name: 'Select Language' }).click();
    const menu = page.getByRole('listbox');
    await expect(menu).toBeVisible();
    const list = menu.locator('.popoverOptions');
    await expect(list).toHaveCSS('scrollbar-width', 'none');
    await expect.poll(() => list.evaluate(el => el.scrollWidth - el.clientWidth)).toBe(0);
    await list.hover();
    await page.mouse.wheel(0, 500);
    await expect.poll(() => list.evaluate(el => el.scrollTop)).toBeGreaterThan(0);
});

test('Add New User keeps focus on the dialog and aligns the language field', async ({ page }) => {
    await page.emulateMedia({ reducedMotion: 'reduce' });
    await page.goto('/auth/v1/admin/users');
    await page.getByRole('button', { name: 'Add New' }).click();
    const dialog = page.getByRole('dialog');
    const email = dialog.getByRole('textbox', { name: 'E-Mail', exact: true });
    await expect(email).toBeVisible();
    await expect(email).toHaveAttribute('data-bwignore', 'true');
    await expect(dialog.getByRole('button', { name: 'Close', exact: true })).toBeFocused();
    await page.keyboard.press('Tab');
    await expect(email).toBeFocused();
    const language = dialog.getByRole('combobox', { name: 'Select Language' });
    await expect(language).toBeVisible();
    const emailBox = await email.boundingBox();
    const languageBox = await language.boundingBox();
    expect(languageBox?.x).toBeCloseTo(emailBox?.x ?? -1, 0);
    expect(languageBox?.width).toBeCloseTo(emailBox?.width ?? -1, 0);
    expect(languageBox?.height).toBeCloseTo(emailBox?.height ?? -1, 0);
    await page.setViewportSize({ width: 375, height: 667 });
    const timezone = dialog.getByRole('combobox', { name: 'Timezone', exact: true });
    const timezoneBox = await timezone.boundingBox();
    const mobileLanguage = await language.boundingBox();
    expect(mobileLanguage?.x).toBeCloseTo(timezoneBox?.x ?? -1, 0);
    expect(mobileLanguage?.width).toBeCloseTo(timezoneBox?.width ?? -1, 0);
    await expect(language.locator('.btnText')).toHaveCSS('text-transform', 'uppercase');
    await language.click();
    await expect(dialog.getByRole('listbox').locator('.optPopover').first()).toHaveCSS(
        'text-transform',
        'uppercase',
    );
});

test('Add New User opens the language menu next to its field @mobile @webkit', async ({ page }) => {
    await page.emulateMedia({ reducedMotion: 'reduce' });
    await page.goto('/auth/v1/admin/users');
    await page.getByRole('button', { name: 'Add New' }).click();
    const dialog = page.getByRole('dialog');
    const language = dialog.getByRole('combobox', { name: 'Select Language' });
    await language.click();
    const menu = dialog.getByRole('listbox');
    await expect(menu).toBeVisible();
    const viewport = page.viewportSize();
    await expect
        .poll(async () => {
            const field = await language.boundingBox();
            const popup = await menu.boundingBox();
            if (!field || !popup || !viewport) return false;
            const gap = Math.min(
                Math.abs(popup.y + popup.height - field.y),
                Math.abs(popup.y - field.y - field.height),
            );
            return (
                Math.abs(popup.x - field.x) <= 2 &&
                gap <= 10 &&
                popup.x >= 0 &&
                popup.y >= 0 &&
                popup.x + popup.width <= viewport.width &&
                popup.y + popup.height <= viewport.height
            );
        })
        .toBe(true);
    await menu.getByRole('button', { name: /^it$/i }).click();
    await expect(menu).toBeHidden();
    await expect(language).toContainText(/it/i);
});

test('user Info fits the language menu and stays aligned across tabs', async ({ page }) => {
    await page.goto('/auth/v1/admin/users');
    await page
        .getByRole('main')
        .getByRole('navigation')
        .getByText(ADMIN_EMAIL, { exact: true })
        .click();
    const detail = page.locator('.admin-detail');
    const language = detail.getByRole('combobox', { name: 'Select Language' });
    await expect(language.locator('.btnText')).toHaveCSS('text-transform', 'uppercase');
    await language.click();
    const options = detail.getByRole('listbox').locator('.popoverOptions');
    await expect(options).toBeVisible();
    await expect(options.locator('.optPopover').first()).toHaveCSS('text-transform', 'uppercase');
    expect(await options.evaluate(list => list.scrollWidth)).toBe(
        await options.evaluate(list => list.clientWidth),
    );
    const width = await detail.evaluate(element => element.clientWidth);
    await detailTab(page, 'Password').click();
    await expect(language).toBeHidden();
    expect(await detail.evaluate(element => element.clientWidth)).toBe(width);
});

test('long Markdown scrolls inside the editor in both modes', async ({ page }) => {
    await page.goto('/auth/v1/admin/users');
    await page.getByRole('button', { name: 'Send E-Mail' }).click();
    const dialog = page.getByRole('dialog');
    const editor = dialog.getByRole('textbox', { name: 'Edit Text' });
    await expect(editor).toBeVisible();
    const initialHeight = await editor.evaluate(body => body.clientHeight);
    const paragraphs = Array.from({ length: 100 }, (_, i) => `Paragraph ${i + 1}`).join('\n\n');
    await editor.fill(paragraphs);
    expect(await editor.evaluate(body => body.clientHeight)).toBe(initialHeight);
    expect(await editor.evaluate(body => body.scrollHeight > body.clientHeight)).toBe(true);
    await dialog.getByRole('button', { name: 'Preview', exact: true }).click();
    const preview = dialog.locator('.renderer');
    await expect(preview).toContainText('Paragraph 100');
    expect(await preview.evaluate(body => body.scrollHeight > body.clientHeight)).toBe(true);
    await expect(dialog.getByRole('button', { name: 'Preview', exact: true })).toBeInViewport();
});

test('client tabs keep their width and list S256 first', async ({ page }) => {
    await page.goto('/auth/v1/admin/clients?cid=rauthy');
    const header = page.getByRole('heading', { name: 'Rauthy', exact: true });
    await expect(header).toBeVisible();
    const detail = page.locator('.admin-detail');
    await tab(page, 'Tokens').click();
    const tokens = await detail.evaluate(element => element.clientWidth);
    await tab(page, 'Integrations').click();
    expect(await detail.evaluate(element => element.clientWidth)).toBe(tokens);
    await tab(page, 'Login').click();
    const challenges = page.getByRole('checkbox', { name: /^PKCE (S256|plain)$/ });
    await expect(challenges.first()).toHaveAccessibleName('PKCE S256');
});

test('scheduled mail calendar supports arrows, confirm, cancel and native editing @mobile @webkit', async ({
    page,
}) => {
    await page.goto('/auth/v1/admin/users');
    await page.getByRole('button', { name: 'Send E-Mail' }).click();
    const dialog = page.getByRole('dialog');
    await dialog.getByRole('combobox', { name: 'Delivery', exact: true }).selectOption('scheduled');
    const field = dialog.locator('.admin-date-field');
    const input = field.locator('input[type="date"]');
    const trigger = field.getByRole('button', { name: 'Date', exact: true });
    const year = new Date().getFullYear() + 1;
    const january = `${year}-01-31`;
    const february = `${year}-02-01`;
    const day = (/** @type {string} */ date) =>
        field.getByRole('button').filter({ has: field.page().locator(`time[datetime="${date}"]`) });

    await input.fill(january);
    await expect(input).toHaveValue(january);
    await trigger.press('Enter');
    await expect(day(january)).toBeFocused();
    const popup = field.locator('[popover]:popover-open');
    await expect.poll(() => popup.evaluate(el => el.scrollWidth - el.clientWidth)).toBe(0);
    await page.keyboard.press('ArrowRight');
    await expect(day(february)).toBeFocused();
    await page.keyboard.press('ArrowDown');
    await expect(day(`${year}-02-08`)).toBeFocused();
    await page.keyboard.press('ArrowLeft');
    await expect(day(`${year}-02-07`)).toBeFocused();
    await page.keyboard.press('ArrowUp');
    await expect(day(january)).toBeFocused();
    await expect(input).toHaveValue(january);
    await page.keyboard.press('ArrowRight');
    await page.keyboard.press('Enter');
    await expect(input).toHaveValue(february);
    await expect(trigger).toBeFocused();

    await trigger.press('Space');
    await expect(day(february)).toBeFocused();
    await page.keyboard.press('ArrowRight');
    await page.keyboard.press('Escape');
    await expect(input).toHaveValue(february);
    await expect(trigger).toBeFocused();
});

test('scheduled delivery fields fit above the editor on a phone @mobile @webkit', async ({
    page,
}) => {
    await page.setViewportSize({ width: 375, height: 667 });
    await page.goto('/auth/v1/admin/users');
    await page.getByRole('button', { name: 'Send E-Mail' }).click();
    const dialog = page.getByRole('dialog');
    await chooseOption(page, dialog.getByRole('combobox', { name: 'User Filter' }), 'Has Role');
    await dialog.getByRole('combobox', { name: 'Delivery', exact: true }).selectOption('scheduled');
    await expect(dialog.locator('.admin-date-field')).toBeInViewport({ ratio: 1 });
    await expect(dialog.locator('.admin-time-field')).toBeInViewport({ ratio: 1 });
    await expect(dialog.getByRole('textbox', { name: 'Subject' })).toBeInViewport({ ratio: 1 });
    await expect(dialog.getByRole('textbox', { name: 'Edit Text' })).toBeInViewport({ ratio: 1 });
    await expect(dialog.getByRole('button', { name: 'Send', exact: true })).toBeInViewport({
        ratio: 1,
    });
});

test('archive event text lines up with or without an IP address', async ({ page }) => {
    const timestamp = Date.now() - 60_000;
    await page.route(
        url => url.pathname === '/auth/v1/events',
        route =>
            route.fulfill({
                json: [
                    { id: '1', timestamp, level: 'info', typ: 'RauthyStarted' },
                    {
                        id: '2',
                        timestamp: timestamp - 1000,
                        level: 'info',
                        typ: 'TokenIssued',
                        text: 'Example client',
                        ip: '2001:db8::1234:5678',
                    },
                ],
            }),
    );
    await page.goto('/auth/v1/admin/events');
    const archive = page.locator('#archive');
    const started = archive.getByText('Rauthy Started', { exact: true });
    const token = archive.getByText('Token Issued', { exact: true });
    await expect(started).toBeVisible();
    await expect(token).toBeVisible();
    expect((await started.boundingBox())?.x).toBeCloseTo((await token.boundingBox())?.x ?? -1, 0);
});

test('PAM host fields share the same size', async ({ page }) => {
    await page.goto('/auth/v1/admin/pam');
    const navigation = page.getByRole('main').getByRole('navigation');
    await navigation.getByRole('button', { name: 'Hosts', exact: true }).click();
    await navigation.getByRole('button', { name: 'Add New' }).click();
    const dialog = page.getByRole('dialog');
    const hostname = dialog.getByRole('textbox', { name: 'Hostname', exact: true });
    const group = dialog.getByRole('combobox', { name: 'Groupname' });
    await expect(hostname).toBeVisible();
    await expect(group).toBeVisible();
    await expect
        .poll(async () => {
            const [input, select] = await Promise.all([
                hostname.boundingBox(),
                group.boundingBox(),
            ]);
            return Math.abs((input?.height ?? 0) - (select?.height ?? -1));
        })
        .toBeLessThan(0.5);
});

test('Hostname and Groupname line up in an existing PAM host', async ({ page }) => {
    await page.setViewportSize({ width: 1440, height: 900 });
    const id = 'example-host';
    const name = 'demo-host';
    await page.route('**/auth/v1/pam/groups', route =>
        route.fulfill({ json: [{ id: 1, name: 'default', typ: 'host' }] }),
    );
    await page.route('**/auth/v1/pam/hosts', route =>
        route.fulfill({ json: [{ id, name, aliases: [], addresses: [] }] }),
    );
    await page.route(`**/auth/v1/pam/hosts/${id}`, route =>
        route.fulfill({
            json: {
                id,
                hostname: name,
                gid: 1,
                secret: '',
                force_mfa: true,
                local_password_only: false,
                ips: [],
                aliases: [],
            },
        }),
    );
    await page.goto('/auth/v1/admin/pam');
    const navigation = page.getByRole('main').getByRole('navigation');
    await navigation.getByRole('button', { name: 'Hosts', exact: true }).click();
    await navigation.getByRole('button', { name, exact: true }).click();
    const hostname = page.getByRole('textbox', { name: 'Hostname', exact: true });
    const group = page.getByRole('combobox', { name: 'Groupname' });
    await expect(group).toHaveText('default');
    const hostnameBox = await hostname.boundingBox();
    const groupBox = await group.boundingBox();
    expect(groupBox?.y).toBeCloseTo(hostnameBox?.y ?? -1, 0);
    expect(groupBox?.height).toBeCloseTo(hostnameBox?.height ?? -1, 0);
});

test('encryption key status stays separate from the migration target', async ({ page }) => {
    await page.route('**/auth/v1/encryption/keys', route =>
        route.fulfill({ json: { active: 'current', keys: ['current', 'previous'] } }),
    );
    await page.goto('/auth/v1/admin/config/encryption');
    const available = page.getByRole('region', { name: 'Available Keys' });
    const first = available.getByRole('listitem').filter({ hasText: 'current' });
    await expect(first.getByText('Active Key', { exact: true })).toBeVisible();
    const heading = await available.getByRole('heading').boundingBox();
    expect((await first.locator('code').boundingBox())?.x).toBeCloseTo(heading?.x ?? -1, 0);

    const target = page.getByRole('combobox', { name: /Migrate all existing encrypted values/ });
    await chooseOption(page, target, 'previous');
    await expect(target).toHaveText('previous');
    await expect(first.getByText('Active Key', { exact: true })).toBeVisible();
    await expect(available.getByText('Active Key', { exact: true })).toHaveCount(1);
});

test('generated API key commands wrap without overlapping their controls', async ({ page }) => {
    const name = 'demo-key';
    const secret = `${name}$${'a'.repeat(90)}`;
    await page.route('**/auth/v1/api_keys', route =>
        route.fulfill({ json: { keys: [{ name, created: 1795273200, access: [] }] } }),
    );
    await page.route(`**/auth/v1/api_keys/${name}/secret`, route =>
        route.fulfill({ contentType: 'text/plain', body: secret }),
    );
    await page.setViewportSize({ width: 375, height: 667 });
    await page.goto(`/auth/v1/admin/api_keys?kn=${name}`);
    await tab(page, 'Secret').click();
    await page.getByRole('button', { name: 'Generate', exact: true }).click();
    const command = page.getByRole('group', { name: 'curl text command with jq' });
    const code = command.locator('pre code');
    await expect(code).toHaveText(/^[•\s]+$/);
    await command.getByRole('button', { name: 'Show', exact: true }).click();
    await expect(code).toContainText(`Authorization: API-Key ${secret}`);
    await expect(code).toContainText('| jq');
    const pre = command.locator('pre');
    expect(await pre.evaluate(el => el.scrollWidth - el.clientWidth)).toBe(0);
    const actions = await command.getByRole('button', { name: 'Hide', exact: true }).boundingBox();
    expect((await pre.boundingBox())?.y).toBeGreaterThanOrEqual(
        (actions?.y ?? 0) + (actions?.height ?? 0),
    );
});

test('client secret confirmation scrolls into view without rotating the secret', async ({
    page,
}) => {
    await page.setViewportSize({ width: 1200, height: 480 });
    await page.goto('/auth/v1/admin/clients?cid=init_client');
    await tab(page, 'Login').click();
    const generate = page.getByRole('button', { name: 'Generate New Secret', exact: true });
    await expect(generate).toBeVisible();
    let writes = 0;
    page.on('request', request => {
        if (request.method() === 'PUT' && request.url().endsWith('/init_client/secret')) {
            writes += 1;
        }
    });
    await generate.click();
    const cache = page.getByRole('checkbox', { name: 'Client Secret Cache' });
    await expect(page.getByText(/Caution: You should not cache/)).toBeInViewport({ ratio: 1 });
    await expect(cache).toBeInViewport({ ratio: 1 });
    await expect(generate).toBeInViewport({ ratio: 1 });
    await cache.check();
    await expect(page.getByRole('spinbutton', { name: 'Cache Duration (hours)' })).toBeInViewport({
        ratio: 1,
    });
    await expect(generate).toBeInViewport({ ratio: 1 });
    expect(writes).toBe(0);
});
