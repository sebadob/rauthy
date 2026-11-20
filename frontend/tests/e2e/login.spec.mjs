// The login page (`/auth/v1/oidc/authorize`) as an end user sees it: the two-step e-mail and
// password login, failed logins, the reset request, registration link, passkey login, language,
// theme and keyboard use.
//
// CAUTION: the dev backend blacklists an IP after 3 different failed credentials within 5s
// (`cred_stuff_detection`) and delays logins after failures. This file causes exactly 2 failed
// password logins and waits out the detection window before each of them.
import { test, expect } from './fixtures.mjs';
import { api } from './helpers/api.mjs';
import {
    ACCOUNT_EXPECTED_ERRORS,
    NO_SESSION,
    coolDownCredStuffWindow,
    copyCsrfTokenToRpOrigin,
    createTestUser,
    deleteTestUser,
    emailField,
    fillLoginEmail,
    loginButton,
    loginOnPage,
    loginToAccount,
    nowSecs,
    openAdminPage,
    passkeyLoginButton,
    serveUnderRpOrigin,
    addVirtualAuthenticator,
} from './helpers/account.mjs';
import { waitForMail } from './helpers/mail.mjs';
import { chooseOption } from './helpers/ui.mjs';

test.use({ storageState: NO_SESSION, allowErrors: ACCOUNT_EXPECTED_ERRORS });

/** @type {{ page: import('@playwright/test').Page, close: () => Promise<void> }} */
let admin;
/** @type {Awaited<ReturnType<typeof createTestUser>>} */
let user;

test.beforeEach(async ({ browser, baseURL }) => {
    admin = await openAdminPage(browser, /** @type {string} */ (baseURL));
    user = await createTestUser(admin.page, { prefix: 'e2e-login' });
});

test.afterEach(async () => {
    await deleteTestUser(admin.page, user.id);
    await admin.close();
});

const passwordField = (/** @type {import('@playwright/test').Page} */ page) =>
    page.getByLabel('Password', { exact: true });

test.describe('login page', () => {
    test('e-mail step, then password step, then redirect to the account @mobile', async ({
        page,
    }) => {
        await page.goto('/auth/v1/account');
        await expect(page).toHaveURL(/\/auth\/v1\/oidc\/authorize\?client_id=rauthy/);
        await expect(page).toHaveTitle('Login: Rauthy');
        await expect(page.getByRole('heading', { name: 'Rauthy', level: 2 })).toBeVisible();
        await expect(page.getByRole('link', { name: 'Client Home Page' })).toBeVisible();

        // first step: only the e-mail, the password prompt comes from the backend's answer
        await expect(loginButton(page)).toBeDisabled();
        await expect(passwordField(page)).toHaveCount(0);
        await fillLoginEmail(page, user.email);
        await loginButton(page).click();

        await expect(passwordField(page)).toBeVisible();
        await expect(passwordField(page)).toBeFocused();
        await expect(page.getByRole('button', { name: 'Show' })).toBeVisible();
        await passwordField(page).fill(user.password);
        // the eye button reveals and hides the typed password
        await page.getByRole('button', { name: 'Show' }).click();
        await expect(passwordField(page)).toHaveAttribute('type', 'text');
        await page.getByRole('button', { name: 'Hide' }).click();
        await expect(passwordField(page)).toHaveAttribute('type', 'password');

        await loginButton(page).click();
        await expect(page).toHaveURL(/\/auth\/v1\/account$/, { timeout: 30_000 });
        await expect(page.getByText(user.email, { exact: true }).first()).toBeVisible();

        const u = await api(admin.page, 'GET', `users/${user.id}`);
        expect(u.last_login).toBeGreaterThan(nowSecs() - 120);
    });

    test('a changed e-mail drops the password step again', async ({ page }) => {
        await page.goto('/auth/v1/account');
        await fillLoginEmail(page, user.email);
        await emailField(page).press('Enter');
        await expect(passwordField(page)).toBeVisible();

        await emailField(page).fill(`other-${user.email}`);
        await expect(passwordField(page)).toHaveCount(0);
        await emailField(page).fill(user.email);
        await expect(passwordField(page)).toHaveCount(0);
    });

    test('empty and invalid e-mail show a validation error', async ({ page }) => {
        await page.goto('/auth/v1/account');
        await expect(loginButton(page)).toBeDisabled();

        await fillLoginEmail(page, 'not-an-email');
        await emailField(page).blur();
        await expect(page.getByText('Provide valid E-Mail address')).toBeVisible();
        await expect(emailField(page)).toHaveAttribute('aria-invalid', 'true');

        await emailField(page).fill('');
        await expect(loginButton(page)).toBeDisabled();

        await emailField(page).fill(user.email);
        await expect(emailField(page)).toHaveAttribute('aria-invalid', 'false');
        await expect(page.getByText('Provide valid E-Mail address')).toBeHidden();
    });

    test.describe(() => {
        // the request this test is about
        test.use({
            allowErrors: [
                ...ACCOUNT_EXPECTED_ERRORS,
                /^http 400 POST \/auth\/v1\/oidc\/authorize$/,
            ],
        });

        test('an invalid e-mail is not sent to the backend', async ({ page }) => {
            // UPSTREAM BUG: the Login button calls `onSubmit()` from its `onclick` as well, which
            // skips the form validation. An invalid address is posted, and the raw backend
            // message "Payload validation error: ValidationErrors(...)" is shown to the user.
            // `routes/oidc/authorize/+page.svelte` (the submit `Button` with `onclick`).
            test.fail();
            /** @type {string[]} */
            const posts = [];
            page.on('request', r => {
                if (r.method() === 'POST' && r.url().includes('/oidc/authorize'))
                    posts.push(r.url());
            });
            await page.goto('/auth/v1/account');
            await fillLoginEmail(page, 'not-an-email');
            await loginButton(page).click();
            await expect(page.getByText('Provide valid E-Mail address')).toBeVisible();
            await page.waitForTimeout(1_500);
            await expect(page.getByText(/ValidationErrors/)).toHaveCount(0);
            expect(posts).toEqual([]);
        });
    });

    test('wrong password, then a reset request from the login page, then the right password', async ({
        page,
    }) => {
        // 1 of 2 failed logins in this file
        await coolDownCredStuffWindow(page);
        await page.goto('/auth/v1/account');
        await fillLoginEmail(page, user.email);
        await emailField(page).press('Enter');
        await expect(page.getByRole('button', { name: 'Password forgotten?' })).toHaveCount(0);
        await passwordField(page).fill('Wrong-Password-123');
        await passwordField(page).press('Enter');
        await expect(page.getByText('Invalid credentials')).toBeVisible({ timeout: 30_000 });

        // the failed login offers a password reset
        const since = nowSecs();
        await page.getByRole('button', { name: 'Password forgotten?' }).click();
        await expect(passwordField(page)).toHaveCount(0);
        await expect(loginButton(page)).toHaveCount(0);
        await page.getByRole('button', { name: 'Request', exact: true }).click();
        await expect(
            page.getByText('If your E-Mail exists, a request has been sent'),
        ).toBeVisible();
        const mail = await waitForMail(page.request, { to: user.email, subject: /reset/i, since });
        expect(`${mail.text}`).toContain('/auth/v1/users/');

        // after a few seconds the form returns to the login with the password step
        await expect(page.getByText('If your E-Mail exists, a request has been sent')).toBeHidden({
            timeout: 10_000,
        });
        await expect(passwordField(page)).toBeVisible();
        await passwordField(page).fill(user.password);
        await passwordField(page).press('Enter');
        await expect(page).toHaveURL(/\/auth\/v1\/account$/, { timeout: 30_000 });
    });

    test('an unknown user gets the same password step and the same error', async ({ page }) => {
        // 2 of 2 failed logins in this file
        await coolDownCredStuffWindow(page);
        await page.goto('/auth/v1/account');
        await fillLoginEmail(page, `nobody-${user.email}`);
        await emailField(page).press('Enter');
        // no user enumeration: the password is asked for as for an existing user
        await expect(passwordField(page)).toBeVisible();
        await passwordField(page).fill('Some-Password-123');
        await passwordField(page).press('Enter');
        await expect(page.getByText('Invalid credentials')).toBeVisible({ timeout: 30_000 });

        // a successful login afterwards resets the failed login counter of this IP
        await emailField(page).fill(user.email);
        await expect(passwordField(page)).toHaveCount(0);
        await expect(page.getByText('Invalid credentials')).toBeHidden();
        await emailField(page).press('Enter');
        await passwordField(page).fill(user.password);
        await passwordField(page).press('Enter');
        await expect(page).toHaveURL(/\/auth\/v1\/account$/, { timeout: 30_000 });
    });

    test('the registration link opens the registration form in a new tab', async ({ page }) => {
        await page.goto('/auth/v1/account');
        const link = page.getByRole('link', { name: 'User Registration' });
        await expect(link).toHaveAttribute('href', /^\/auth\/v1\/users\/register/);
        const [popup] = await Promise.all([page.waitForEvent('popup'), link.click()]);
        await expect(popup.getByRole('heading', { name: 'User Registration' })).toBeVisible();
        await expect(popup.getByRole('button', { name: 'Register' })).toBeVisible();
        await popup.close();
    });

    test('theme switch toggles and persists on the login page', async ({ page }) => {
        await page.goto('/auth/v1/account');
        const toggle = page.getByRole('button', { name: /^Change Theme - Active: / });
        const before = /** @type {string} */ (await toggle.getAttribute('aria-label'));
        await toggle.click();
        await expect(toggle).not.toHaveAttribute('aria-label', before);
        const after = /** @type {string} */ (await toggle.getAttribute('aria-label'));
        await page.reload();
        await expect(
            page.getByRole('button', { name: /^Change Theme - Active: / }),
        ).toHaveAttribute('aria-label', after);
    });

    test('language selector translates the login page', async ({ page }) => {
        await page.goto('/auth/v1/account');
        await chooseOption(page, page.getByRole('combobox', { name: 'Select Language' }), 'de');
        await expect(page.getByRole('link', { name: 'Benutzer Registrierung' })).toBeVisible();
        await expect(page.getByRole('combobox', { name: 'Sprache wählen' })).toBeVisible();
        const cookies = await page.context().cookies();
        expect(cookies.find(c => c.name === 'locale')?.value).toBe('de');

        await chooseOption(page, page.getByRole('combobox', { name: 'Sprache wählen' }), 'en');
        await expect(page.getByRole('link', { name: 'User Registration' })).toBeVisible();
    });

    test('keyboard only: Tab to the e-mail, Enter, password, Enter', async ({ page }) => {
        await page.goto('/auth/v1/account');
        await expect(loginButton(page)).toBeDisabled();
        // wait for hydration, so that typing reaches the component state
        await page.waitForLoadState('networkidle');
        await page.locator('body').focus();
        for (let i = 0; i < 10; i++) {
            await page.keyboard.press('Tab');
            if (await emailField(page).evaluate(el => el === document.activeElement)) break;
        }
        await expect(emailField(page)).toBeFocused();
        await page.keyboard.type(user.email);
        await page.keyboard.press('Enter');

        await expect(passwordField(page)).toBeFocused();
        await page.keyboard.type(user.password);
        await page.keyboard.press('Enter');
        await expect(page).toHaveURL(/\/auth\/v1\/account$/, { timeout: 30_000 });
    });

    test('login_hint prefills the e-mail', async ({ page }) => {
        await page.goto('/auth/v1/account');
        await expect(page).toHaveURL(/\/oidc\/authorize/);
        const url = new URL(page.url());
        url.searchParams.set('login_hint', user.email);
        await page.goto(url.toString());
        await expect(emailField(page)).toHaveValue(user.email);
        await expect(loginButton(page)).toBeEnabled();
    });
});

test.describe('passkey login', () => {
    test('a resident key logs in with the Passkey button, without e-mail', async ({
        page,
        baseURL,
    }) => {
        const authenticator = await addVirtualAuthenticator(page);
        const rp = await serveUnderRpOrigin(page.context(), /** @type {string} */ (baseURL));

        // arrange: register a resident key through the account dashboard
        await loginToAccount(page, user);
        await copyCsrfTokenToRpOrigin(page);
        await page.goto(rp('/auth/v1/account'));
        await page.getByRole('button', { name: 'MFA', exact: true }).click();
        await page.getByRole('button', { name: 'Register New Passkey' }).click();
        const dialog = page.getByRole('dialog');
        await dialog.getByLabel('Password').fill(user.password);
        await dialog.getByRole('button', { name: 'Authenticate' }).click();
        await chooseOption(
            page,
            page.getByRole('combobox', { name: 'Passkey Type' }),
            'Resident Key',
        );
        await page.getByRole('textbox', { name: 'Passkey Name' }).fill('e2e-resident');
        await page.getByRole('button', { name: 'Register', exact: true }).click();
        const passkeyCard = page.getByRole('article').filter({ hasText: 'e2e-resident' });
        await expect(passkeyCard.getByText('Last Used')).toBeVisible();
        await expect(page.getByRole('button', { name: 'Test passkey' })).toBeVisible();
        expect(await authenticator.credentials()).toHaveLength(1);

        await admin.page.goto(`/auth/v1/admin/users?uid=${user.id}&userTab=mfa`);
        const adminCard = admin.page.getByRole('article').filter({ hasText: 'e2e-resident' });
        await expect(adminCard.getByText('Last Used')).toBeVisible();
        await adminCard.getByRole('button', { name: 'Delete', exact: true }).click();
        await expect(admin.page.getByRole('dialog').getByText('e2e-resident')).toBeVisible();
        await admin.page.getByRole('button', { name: 'Cancel', exact: true }).click();

        // log out by dropping the session, then log in with the Passkey button only
        // Leave the dashboard before clearing its session so in-flight requests cannot redirect.
        await page.goto('about:blank');
        await page.context().clearCookies();
        await page.goto('/auth/v1/account');
        await expect(page).toHaveURL(/\/oidc\/authorize/);
        await page.goto(page.url().replace(new URL(page.url()).origin, rp('')));
        await expect(page.getByText('or login with')).toBeVisible();
        await expect(passkeyLoginButton(page)).toBeVisible();
        await passkeyLoginButton(page).click();
        await expect(page.getByText('Expecting Passkey Authentication')).toBeVisible();
        await expect(page).toHaveURL(/\/auth\/v1\/account$/, { timeout: 30_000 });
        await expect(page.getByText(user.email, { exact: true }).first()).toBeVisible();

        const keys = await api(admin.page, 'GET', `users/${user.id}/webauthn`);
        expect(keys).toHaveLength(1);
        expect(keys[0]).toMatchObject({ name: 'e2e-resident', resident_key: true });
    });

    test('a passkey of a password account is asked for after the password', async ({
        page,
        baseURL,
    }) => {
        await addVirtualAuthenticator(page);
        const rp = await serveUnderRpOrigin(page.context(), /** @type {string} */ (baseURL));
        await loginToAccount(page, user);
        await copyCsrfTokenToRpOrigin(page);
        await page.goto(rp('/auth/v1/account'));
        await page.getByRole('button', { name: 'MFA', exact: true }).click();
        await page.getByRole('button', { name: 'Register New Passkey' }).click();
        await page.getByRole('dialog').getByLabel('Password').fill(user.password);
        await page.getByRole('dialog').getByRole('button', { name: 'Authenticate' }).click();
        await page.getByRole('textbox', { name: 'Passkey Name' }).fill('e2e-default');
        await page.getByRole('button', { name: 'Register', exact: true }).click();
        await expect(page.getByText('e2e-default')).toBeVisible();

        // Leave the dashboard before clearing its session so in-flight requests cannot redirect.
        await page.goto('about:blank');
        await page.context().clearCookies();
        await page.goto('/auth/v1/account');
        await expect(page).toHaveURL(/\/oidc\/authorize/);
        await page.goto(page.url().replace(new URL(page.url()).origin, rp('')));
        // the virtual authenticator answers at once, so the "Expecting Passkey Authentication"
        // step can be gone before it renders: the finished WebAuthn request proves it was asked
        const webauthn = page.waitForResponse(r =>
            r.url().endsWith('/auth/v1/users/webauthn_finish'),
        );
        await loginOnPage(page, user.email, user.password);
        expect((await webauthn).ok()).toBe(true);
        await expect(page).toHaveURL(/\/auth\/v1\/account$/, { timeout: 30_000 });
    });
});
