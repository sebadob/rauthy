// Built-UI test for the markdown editor toolbar, used in the admin "Send E-Mail" dialog: a button
// must apply its markdown to the text the user selected before clicking it.
// All requests are intercepted; no backend needs to run.
import assert from 'node:assert/strict';
import { expect } from '@playwright/test';
import { after, before, test } from 'node:test';
import {
    injectTemplates,
    launchChromium,
    ORIGIN,
    readBuiltHtml,
    serveUi,
} from './helpers/browser.mjs';

const DOCUMENT_PATH = '/auth/v1/admin/users';

const SESSION = {
    id: 'session-id',
    csrf_token: 'csrf-token',
    user_id: 'admin-id',
    roles: 'rauthy_admin',
    groups: '',
    exp: '1700003600',
    timeout: 'session_long',
    state: 'Auth',
};

/** @type {Record<string, any>} */
const EMPTY_LISTS = {
    '/auth/v1/users': [],
    '/auth/v1/roles': [],
    '/auth/v1/groups': [],
    '/auth/v1/email': [],
    '/auth/v1/providers/minimal': [],
};

const TOOLS = [
    ['Heading 1', '# hello'],
    ['Bold', '**hello**'],
    ['Italic', '*hello*'],
    ['List', '- hello'],
    ['Code', '`hello`'],
];

/** @type {any} */
let browser;
/** @type {string} */
let html;

before(async () => {
    browser = await launchChromium();
    html = injectTemplates(await readBuiltHtml('admin/users.html'), {});
});

after(async () => {
    await browser?.close();
});

test('toolbar buttons apply their markdown to the selection', async () => {
    const context = await browser.newContext({ serviceWorkers: 'block' });
    await serveUi({
        context,
        documentPath: DOCUMENT_PATH,
        html,
        requests: [],
        handle: (/** @type {any} */ _route, /** @type {any} */ request, /** @type {URL} */ url) => {
            if (url.pathname === '/auth/v1/oidc/sessioninfo') return { json: SESSION };
            if (url.pathname === '/auth/v1/auth_check_admin') return { status: 200, body: '' };
            if (request.method() === 'GET' && url.pathname in EMPTY_LISTS) {
                return { json: EMPTY_LISTS[url.pathname] };
            }
        },
    });

    try {
        const page = await context.newPage();
        await page.goto(`${ORIGIN}${DOCUMENT_PATH}`);
        await page.getByRole('button', { name: 'Send E-Mail' }).click();
        const dialog = page.getByRole('dialog');
        const editor = dialog.getByRole('textbox', { name: 'Edit Text' });

        for (const [name, expected] of TOOLS) {
            await editor.fill('hello');
            await editor.press('ControlOrMeta+a');
            await dialog.getByRole('button', { name, exact: true }).click();
            assert.equal(await editor.innerText(), expected, name);
        }

        await page.setViewportSize({ width: 1280, height: 900 });
        const delivery = dialog.getByRole('combobox', { name: 'Delivery', exact: true });
        await delivery.selectOption('scheduled');
        const controls = [
            dialog.getByRole('combobox', { name: 'User Filter', exact: true }),
            delivery,
            dialog.locator('input[type="date"]'),
            dialog.getByRole('textbox', { name: 'Time', exact: true }),
        ];
        const subject = dialog.getByRole('textbox', { name: 'Subject' });
        await expect
            .poll(async () => {
                const boxes = await Promise.all(controls.map(control => control.boundingBox()));
                const first = boxes[0];
                return (
                    first &&
                    boxes.every(
                        box =>
                            box &&
                            Math.abs(box.y - first.y) <= 1 &&
                            Math.abs(box.height - first.height) <= 1,
                    )
                );
            })
            .toBe(true);
        const dates = await controls[2].boundingBox();
        const subjectBox = await subject.boundingBox();
        assert.ok(dates && subjectBox && subjectBox.y >= dates.y + dates.height);
        await delivery.selectOption('immediate');
        await editor.fill('Draft before expanding');
        const expand = dialog.getByRole('button', { name: 'Expand content' });
        const normal = await editor.boundingBox();
        await expand.click();
        await expect(expand).toHaveAttribute('aria-pressed', 'true');
        const expanded = await editor.boundingBox();
        assert.ok(normal && expanded && expanded.width > normal.width);
        assert.ok(normal && expanded && expanded.height > normal.height);
        await expect(editor).toHaveText('Draft before expanding');
        await expand.click();
        await expect(expand).toHaveAttribute('aria-pressed', 'false');

        const format = dialog.getByRole('radiogroup', { name: 'Format', exact: true });
        const markdown = format.getByRole('radio', { name: 'Markdown', exact: true });
        const plainText = format.getByRole('radio', { name: 'Plain text', exact: true });
        await expect(markdown).toBeChecked({ timeout: 3000 });
        await markdown.focus();
        await markdown.press('ArrowRight');
        await expect(plainText).toBeChecked();
        await expect(plainText).toBeFocused();
        await expect(dialog.getByRole('button', { name: 'Preview', exact: true })).toHaveCount(0);
        await expect(editor).toHaveText('Draft before expanding');
        await plainText.press('ArrowLeft');
        await expect(markdown).toBeChecked();
        await expect(markdown).toBeFocused();
        await expect(dialog.getByRole('button', { name: 'Preview', exact: true })).toBeVisible();
        await expect(editor).toHaveText('Draft before expanding');

        await page.setViewportSize({ width: 390, height: 520 });
        await dialog
            .getByRole('combobox', { name: 'Delivery', exact: true })
            .selectOption('scheduled');
        await expect(dialog.locator('.fields')).toHaveCSS('scrollbar-width', 'none');
        await editor.fill('Paragraph\n'.repeat(100));
        await editor.evaluate((/** @type {HTMLElement} */ element) => {
            element.scrollTop = element.scrollHeight;
        });
        await expect(dialog.getByRole('button', { name: 'Bold', exact: true })).toBeInViewport();
        await expect(dialog.getByRole('button', { name: 'Send', exact: true })).toBeInViewport({
            ratio: 1,
        });

        for (const [key, expected] of [
            ['b', '**shortcut**'],
            ['i', '*shortcut*'],
            ['k', '[shortcut](https://...)'],
        ]) {
            await editor.fill('shortcut');
            await editor.press('ControlOrMeta+a');
            await editor.press(`ControlOrMeta+${key}`);
            await expect(editor).toHaveText(expected);
        }
    } finally {
        await context.close();
    }
});
