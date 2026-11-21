// Users: list, search, details, add, edit, attributes, password, reset mail, roles, force logout
// and delete, checked in the UI and against the backend.
import { test, expect } from './fixtures.mjs';
import { api } from './helpers/api.mjs';
import { unique } from './helpers/env.mjs';
import { loginWithPassword } from './helpers/login.mjs';
import { chooseOption } from './helpers/ui.mjs';
import {
    createUser,
    deleteUser,
    detailTab,
    findUser,
    mailIdsTo,
    openSelectList,
    closeSelectList,
    strongPassword,
    trackRequests,
    userEntries,
    userEntry,
    waitForNewMail,
} from './helpers/users.mjs';

const USERS = '/auth/v1/admin/users';

/** @type {string[]} */
let created = [];

test.afterEach(async ({ page }) => {
    for (const id of created) await deleteUser(page, id);
    created = [];
});

/** Creates a user through the API and remembers it for the cleanup. */
async function newUser(/** @type {import('@playwright/test').Page} */ page, data = {}) {
    await page.goto(USERS);
    const user = await createUser(page, { given_name: unique('Given'), ...data });
    created.push(user.id);
    return user;
}

/**
 * Opens the details of `user` by deep link and waits for its header.
 * @param {import('@playwright/test').Page} page
 * @param {{ id: string, given_name: string }} user
 */
async function openUser(page, user) {
    await page.goto(`${USERS}?uid=${user.id}`);
    await expect(
        page.getByRole('main').getByRole('heading', { name: user.given_name }),
    ).toBeVisible();
}

/**
 * Opens the Password tab. A user who never set a password first shows a short notice with a
 * "Manual Initialization" button in front of the form.
 */
async function openPasswordForm(/** @type {import('@playwright/test').Page} */ page) {
    await detailTab(page, 'Password').click();
    const init = page.getByRole('button', { name: 'Manual Initialization' });
    const form = page.getByPlaceholder('New Password');
    await expect(init.or(form).first()).toBeVisible();
    if (await init.isVisible()) await init.click();
    await expect(form).toBeVisible();
}

/** The final action button on a confirmation tab: the last button with that visible label. */
function tabAction(
    /** @type {import('@playwright/test').Page} */ page,
    /** @type {string} */ name,
) {
    return page
        .getByRole('main')
        .getByRole('button', { name, exact: true })
        .filter({ hasText: name })
        .last();
}

test.describe('users list', () => {
    test('searches for a name and ignores a late username response after clearing', async ({
        page,
    }) => {
        const username = unique('lookup').replaceAll('-', '');
        const target = await newUser(page, { preferred_username: username });
        const other = await newUser(page);
        await page.goto(USERS);

        const search = page.getByRole('searchbox', { name: 'Search' });
        const options = page.getByRole('combobox', { name: 'Search Options' });
        await chooseOption(page, options, 'Name');
        await search.fill(target.given_name);
        await expect(userEntry(page, target.email)).toBeVisible();
        await expect(userEntry(page, other.email)).toHaveCount(0);

        await chooseOption(page, options, 'Username');
        const url = (/** @type {URL} */ u) =>
            u.pathname === '/auth/v1/search' &&
            u.searchParams.get('idx') === 'username' &&
            u.searchParams.get('q') === username;
        const started = Promise.withResolvers();
        const release = Promise.withResolvers();
        await page.route(url, async route => {
            try {
                const response = await route.fetch();
                expect(response.ok()).toBe(true);
                expect(await response.json()).toEqual(
                    expect.arrayContaining([expect.objectContaining({ id: target.id })]),
                );
                started.resolve(undefined);
                await release.promise;
                await route.fulfill({ response });
            } catch (err) {
                started.reject(err);
            }
        });

        await search.fill(username);
        try {
            await started.promise;
            await search.clear();
            await expect(userEntry(page, other.email)).toBeVisible();
            const delivered = page.waitForResponse(response => url(new URL(response.url())));
            release.resolve(undefined);
            await (await delivered).finished();
            await expect(userEntry(page, other.email)).toBeVisible();
        } finally {
            release.resolve(undefined);
        }
    });

    test('changes page size and moves between pages of users', async ({ page }) => {
        for (let index = 0; index < 6; index++) await newUser(page);
        await page.goto(USERS);
        const pagination = page.getByRole('navigation', { name: 'Pagination', exact: true });
        const size = pagination.getByRole('combobox', { name: 'Show count' });
        const next = pagination.getByRole('button', { name: 'Go to next page' });
        const previous = pagination.getByRole('button', { name: 'Go to previous page' });
        await size.selectOption('5');
        await expect(size).toHaveValue('5');
        await expect(userEntries(page)).toHaveCount(5);
        const firstPage = await userEntries(page).allTextContents();
        await expect(previous).toBeDisabled();
        await next.click();
        await expect(pagination.getByRole('listitem', { name: 'Go to page 2' })).toHaveAttribute(
            'aria-current',
            'page',
        );
        const secondPage = await userEntries(page).allTextContents();
        expect(secondPage).not.toEqual(firstPage);
        if (await next.isEnabled()) {
            await next.click();
            await expect(
                pagination.getByRole('listitem', { name: 'Go to page 3' }),
            ).toHaveAttribute('aria-current', 'page');
            await previous.click();
            await expect.poll(() => userEntries(page).allTextContents()).toEqual(secondPage);
        }
        await previous.click();
        await expect.poll(() => userEntries(page).allTextContents()).toEqual(firstPage);
        await page.reload();
        await expect(size).toHaveValue('5');
        await expect(userEntries(page)).toHaveCount(5);
    });

    test('shows users with their count and finds them by e-mail', async ({ page }) => {
        const user = await newUser(page);
        await page.goto(USERS);
        await expect(userEntry(page, user.email)).toBeVisible();

        const total = (await api(page, 'GET', 'users')).length;
        await expect(page.getByRole('main').getByRole('navigation').first()).toContainText(
            String(total),
        );

        const search = page.getByRole('searchbox', { name: 'Search' });
        await search.fill(user.email.split('@')[0]);
        await expect(userEntries(page)).toHaveCount(1);
        await expect(userEntry(page, user.email)).toBeVisible();

        await search.fill('');
        await expect.poll(() => userEntries(page).count()).toBeGreaterThan(1);
    });

    test('selecting a user opens its details and puts it in the URL', async ({ page }) => {
        const user = await newUser(page);
        await page.goto(USERS);
        await userEntry(page, user.email).click();
        await expect(page).toHaveURL(new RegExp(`uid=${user.id}`));
        const main = page.getByRole('main');
        await expect(main.getByRole('heading', { name: user.given_name })).toBeVisible();
        await expect(main.getByRole('textbox', { name: 'E-Mail' })).toHaveValue(user.email);
    });
});

test.describe('add new user', () => {
    test('creates a user from the dialog and selects it', async ({ page }) => {
        await page.goto(USERS);
        const email = `${unique('e2e-new')}@example.com`;
        await page.getByRole('button', { name: 'Add New' }).click();
        const dialog = page.getByRole('dialog');
        await expect(dialog).toBeVisible();
        await dialog.getByRole('textbox', { name: 'E-Mail' }).fill(email);
        await dialog.getByRole('textbox', { name: 'Given Name' }).fill('New');
        await dialog.getByRole('textbox', { name: 'Family Name' }).fill('Person');
        await dialog.getByRole('button', { name: 'Save' }).click();

        await expect(dialog).toBeHidden();
        await expect(page).toHaveURL(/uid=/);
        const id = new URL(page.url()).searchParams.get('uid') || '';
        created.push(id);
        await expect(userEntry(page, email)).toBeVisible();
        const user = await api(page, 'GET', `users/${id}`);
        expect(user).toMatchObject({ email, given_name: 'New', family_name: 'Person' });
    });

    test('does not send an incomplete form', async ({ page }) => {
        await page.goto(USERS);
        const posts = trackRequests(page, 'POST', /^\/auth\/v1\/users$/);
        await page.getByRole('button', { name: 'Add New' }).click();
        const dialog = page.getByRole('dialog');
        await dialog.getByRole('textbox', { name: 'Given Name' }).fill('No Mail');
        await dialog.getByRole('button', { name: 'Save' }).click();
        await expect(dialog).toBeVisible();
        await expect(dialog.getByRole('textbox', { name: 'E-Mail' })).toHaveJSProperty(
            'validity.valid',
            false,
        );
        expect(posts).toHaveLength(0);
    });
});

test.describe('user details', () => {
    test('edits and saves the profile', async ({ page }) => {
        const user = await newUser(page);
        await openUser(page, user);
        const main = page.getByRole('main');
        const given = unique('Edited');
        await main.getByRole('textbox', { name: 'Given Name' }).fill(given);
        await main.getByRole('textbox', { name: 'Family Name' }).fill('Changed');
        await main.getByRole('textbox', { name: 'City' }).fill('Gotham');
        await main.getByRole('checkbox', { name: 'Enabled' }).uncheck();
        await main.getByRole('button', { name: 'Save', exact: true }).click();

        await expect
            .poll(async () => (await api(page, 'GET', `users/${user.id}`)).given_name)
            .toBe(given);
        const saved = await api(page, 'GET', `users/${user.id}`);
        expect(saved.family_name).toBe('Changed');
        expect(saved.enabled).toBe(false);
        expect(saved.user_values?.city).toBe('Gotham');
        await expect(userEntry(page, user.email)).toContainText(given);
    });

    test('saves custom attributes', async ({ page }) => {
        const user = await newUser(page);
        const name = unique('profile_attr').replaceAll('-', '_');
        const value = unique('attr');
        await api(page, 'POST', 'users/attr', { name });
        try {
            await openUser(page, user);
            await detailTab(page, 'Attributes').click();
            const main = page.getByRole('main');
            await main.getByRole('textbox', { name, exact: true }).fill(value);
            await main.getByRole('button', { name: 'Save', exact: true }).click();
            await expect
                .poll(async () => JSON.stringify(await api(page, 'GET', `users/${user.id}/attr`)))
                .toContain(value);
        } finally {
            await api(page, 'DELETE', `users/attr/${name}`);
        }
    });

    test('sets a password manually, after which the user can log in', async ({
        page,
        browser,
        baseURL,
    }) => {
        const user = await newUser(page);
        await openUser(page, user);
        await openPasswordForm(page);
        const pwd = strongPassword();
        await page.getByPlaceholder('New Password').fill(pwd);
        await page.getByPlaceholder('Confirm Password').fill(pwd);
        const saved = page.waitForResponse(
            r => r.request().method() === 'PATCH' && r.url().endsWith(`/users/${user.id}`),
        );
        await page.getByRole('button', { name: 'Save Password' }).click();
        expect((await saved).ok()).toBe(true);

        // an explicit empty state: new contexts would otherwise inherit the admin session of the config
        const ctx = await browser.newContext({
            baseURL,
            storageState: { cookies: [], origins: [] },
        });
        const p2 = await ctx.newPage();
        await loginWithPassword(p2, {
            email: user.email,
            password: pwd,
            startPath: '/auth/v1/account',
        });
        await expect(p2).toHaveURL(/\/auth\/v1\/account/, { timeout: 20_000 });
        await expect(p2.getByText(user.email).first()).toBeVisible();
        await ctx.close();
    });

    test('does not send mismatching passwords', async ({ page }) => {
        const user = await newUser(page);
        await openUser(page, user);
        await openPasswordForm(page);
        const patches = trackRequests(page, 'PATCH', new RegExp(`/users/${user.id}$`));
        await page.getByPlaceholder('New Password').fill(strongPassword());
        await page.getByPlaceholder('Confirm Password').fill(strongPassword());
        await page.getByRole('button', { name: 'Save Password' }).click();
        await expect(page.getByRole('button', { name: 'Save Password' })).toBeVisible();
        await page.waitForTimeout(500);
        expect(patches).toHaveLength(0);
    });

    test('sends a password reset e-mail', async ({ page, request }) => {
        const user = await newUser(page);
        // creating a user already sends the welcome mail, wait for it first
        await waitForNewMail(request, { to: user.email, known: [] });
        const known = await mailIdsTo(request, user.email);
        await openUser(page, user);
        await detailTab(page, 'Password').click();
        const sent = page.waitForResponse(r => r.url().endsWith('/users/request_reset'));
        await page.getByRole('button', { name: 'Send Reset E-Mail' }).click();
        expect((await sent).ok()).toBe(true);
        const mail = await waitForNewMail(request, { to: user.email, known });
        expect(JSON.stringify(mail)).toContain('/auth/v1/users/');
    });

    test('assigns a role and saves it', async ({ page }) => {
        const user = await newUser(page);
        const role = await api(page, 'POST', 'roles', { role: unique('profile-role') });
        try {
            await openUser(page, user);
            const dialog = await openSelectList(page.getByRole('main'), 'Roles');
            await dialog.getByRole('checkbox', { name: role.name, exact: true }).check();
            await closeSelectList(dialog);
            await page.getByRole('main').getByRole('button', { name: 'Save', exact: true }).click();
            await expect
                .poll(async () => (await api(page, 'GET', `users/${user.id}`)).roles)
                .toContain(role.name);
        } finally {
            await api(page, 'DELETE', `roles/${role.id}`);
        }
    });

    test('force logout invalidates the sessions of the user', async ({ page }) => {
        const user = await newUser(page);
        await openUser(page, user);
        await detailTab(page, 'Logout').click();
        const done = page.waitForResponse(
            r => r.request().method() === 'DELETE' && r.url().endsWith(`/sessions/${user.id}`),
        );
        await tabAction(page, 'Logout').click();
        expect((await done).ok()).toBe(true);
    });

    test('keeps the selected user section on reload without focusing Info on arrival', async ({
        page,
    }) => {
        const user = await newUser(page);
        await openUser(page, user);
        await expect(detailTab(page, 'Info')).not.toBeFocused();
        await detailTab(page, 'Password').click();
        await expect(detailTab(page, 'Password')).toHaveAttribute('aria-current', 'step');
        await page.reload();
        await expect(detailTab(page, 'Password')).toHaveAttribute('aria-current', 'step');
    });

    test('confirms user deletion from the entity header', async ({ page }) => {
        const user = await newUser(page);
        await openUser(page, user);
        const action = page.locator('.admin-entity-header').getByRole('button', { name: 'Delete' });
        const dialog = page.getByRole('dialog', { name: 'Delete' });
        await action.click();
        await expect(dialog.getByText(user.email, { exact: true })).toBeVisible();
        const cancel = dialog.getByRole('button', { name: 'Cancel' });
        await expect(cancel).toBeFocused();
        await cancel.click();
        await expect(dialog).toBeHidden();
        await expect(action).toBeFocused();
        expect(await findUser(page, user.id)).toBeDefined();

        await action.click();
        await dialog.getByRole('button', { name: 'Delete' }).click();
        await expect(userEntry(page, user.email)).toHaveCount(0);
        await expect.poll(() => findUser(page, user.id)).toBeUndefined();
    });
});

test.describe('users at phone size', () => {
    test('the list opens first and retains its filter when returning from a user @mobile', async ({
        page,
        isMobile,
    }) => {
        const user = await newUser(page);
        await page.goto(USERS);
        const main = page.getByRole('main');
        const list = main.getByRole('navigation', { name: '', exact: true });
        const search = list.getByRole('searchbox', { name: 'Search' });
        const back = main.getByRole('button', { name: 'Back', exact: true });
        const email = main.getByRole('textbox', { name: 'E-Mail' });
        await expect(search).toBeVisible();
        await expect(email).toBeHidden();
        await search.fill(user.email);
        await userEntry(page, user.email).click();
        await expect(email).toHaveValue(user.email);

        if (isMobile) {
            await expect(list).toBeHidden();
            await expect(back).toBeVisible();
            await page.goBack();
            await expect(search).toBeVisible();
            await expect(search).toHaveValue(user.email);
            await page.goForward();
            await expect(email).toHaveValue(user.email);

            await back.click();
            await expect(search).toHaveValue(user.email);
            await expect(email).toBeHidden();
            await expect(page).toHaveURL(url => !url.searchParams.has('uid'));

            await openUser(page, user);
            await expect(list).toBeHidden();
            await back.click();
            await expect(userEntry(page, user.email)).toBeVisible();
        } else {
            await expect(list).toBeVisible();
            await expect(back).toBeHidden();
        }

        const overflow = await page.evaluate(
            () => document.documentElement.scrollWidth - window.innerWidth,
        );
        expect(overflow).toBeLessThanOrEqual(1);
    });
});
