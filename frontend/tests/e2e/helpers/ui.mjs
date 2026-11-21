// Interaction helpers that work on accessible names, so they survive markup and style changes.
import { expect } from '@playwright/test';

/**
 * Opens a combobox and picks `value`. Prefers proper `role=option` entries and falls back to the
 * clickable entries inside the listbox for widgets that do not expose options yet.
 * @param {import('@playwright/test').Page} page
 * @param {import('@playwright/test').Locator} combobox
 * @param {string} value
 */
export async function chooseOption(page, combobox, value) {
    await combobox.click();
    const listbox = page.getByRole('listbox').last();
    await expect(listbox).toBeVisible();
    const option = listbox.getByRole('option', { name: value, exact: true });
    if (await option.count()) {
        await option.first().click();
    } else {
        await listbox.getByRole('button', { name: value, exact: true }).first().click();
    }
    await expect(listbox).toBeHidden();
}

/**
 * Returns to the list when a phone-size admin detail is open.
 * @param {import('@playwright/test').Page} page
 */
export async function showEntryList(page) {
    const main = page.getByRole('main');
    await expect(main.getByRole('heading', { level: 1 })).toBeVisible();
    const back = main.locator('.navSub').getByRole('button', { name: 'Back', exact: true });
    if (await back.isVisible()) await back.click();
}

/**
 * Enter a time through the same keystrokes as a user. The control inserts its own colon.
 * @param {import('@playwright/test').Locator} input
 * @param {string} time
 */
export async function enterTime(input, time) {
    await input.focus();
    await input.clear();
    await expect(input).toHaveValue('');
    await input.pressSequentially(time.replace(':', ''));
    await expect(input).toHaveValue(time);
}
