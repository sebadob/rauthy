import { test, expect } from './fixtures.mjs';
import { api } from './helpers/api.mjs';
import { enterTime, chooseOption, showEntryList } from './helpers/ui.mjs';
import {
    apiRaw,
    berlinTs,
    createUser,
    dateInput,
    fmtTs,
    policyRequest,
    recordRequests,
} from './helpers/config.mjs';

/** @param {import('@playwright/test').Page} page */
const main = page => page.getByRole('main');
/** @param {import('@playwright/test').Page} page */
const subNav = page => main(page).getByRole('navigation');

/**
 * Returns to the section list when a phone-size detail is open.
 * @param {import('@playwright/test').Page} page
 * @param {import('@playwright/test').Locator} link
 */
async function openSubNav(page, link) {
    if (await link.isVisible()) return;
    await showEntryList(page);
    await expect(link).toBeVisible();
}

// Config sub pages: link name in the sub navigation, path and something only that page shows.
const PAGES = [
    ['Password Policy', 'policy', 'Rauthy Password Policy'],
    ['JWKS', 'jwks', 'Rauthy JWKS'],
    ['Password Hashing', 'argon2', 'Rauthy Password Hashing'],
    ['Encryption', 'encryption', 'Rauthy Encryption'],
    ['Backups', 'backups', 'Rauthy Backups'],
    ['ToS', 'tos', 'Rauthy Terms of Service'],
];
/** @type {Record<string, (page: import('@playwright/test').Page) => import('@playwright/test').Locator>} */
const MARKER = {
    policy: p => main(p).getByRole('heading', { name: 'Password Policy' }),
    jwks: p => main(p).getByRole('heading', { name: 'Json Web Keys' }),
    argon2: p => main(p).getByRole('heading', { name: 'Argon2ID - Password Hashing' }),
    encryption: p => main(p).getByRole('heading', { name: 'Encryption Keys' }),
    backups: p => main(p).getByRole('heading', { name: 'Backups', exact: true }),
    tos: p => main(p).getByRole('heading', { name: 'Terms of Service' }),
};

test.describe('config sub-navigation', () => {
    test('every config page is reachable from the sub-navigation @mobile', async ({
        page,
        isMobile,
    }) => {
        await page.goto('/auth/v1/admin/config/policy');
        for (const [name, path, title] of PAGES) {
            const link = subNav(page).getByRole('link', { name, exact: true });
            if (isMobile) await openSubNav(page, link);
            await link.click();
            await expect(page).toHaveURL(new RegExp(`/auth/v1/admin/config/${path}$`));
            await expect(page).toHaveTitle(title);
            await expect(MARKER[path](page)).toBeVisible();
            if (!isMobile) {
                await expect(link).toHaveAttribute('aria-current', 'page');
            }
        }
    });

    test('deep links load every config page', async ({ page }) => {
        for (const [, path, title] of PAGES) {
            const response = await page.goto(`/auth/v1/admin/config/${path}`);
            expect(response?.status()).toBe(200);
            await expect(page).toHaveTitle(title);
            await expect(MARKER[path](page)).toBeVisible();
        }
    });
});

test.describe('password policy', () => {
    const FIELDS = /** @type {const} */ ([
        ['Length min', 'length_min'],
        ['Length max', 'length_max'],
        ['Lowercase letters min', 'include_lower_case'],
        ['Uppercase letters min', 'include_upper_case'],
        ['Digits min', 'include_digits'],
        ['Special characters min', 'include_special'],
        ['Not one of last recent passwords', 'not_recently_used'],
        ['Valid For Days', 'valid_days'],
    ]);
    /** @param {import('@playwright/test').Page} page @param {string} name */
    const field = (page, name) => main(page).getByRole('spinbutton', { name, exact: true });
    /** @param {import('@playwright/test').Page} page */
    const save = async page => {
        const res = page.waitForResponse(
            r => r.url().endsWith('/auth/v1/password_policy') && r.request().method() === 'PUT',
        );
        await main(page).getByRole('button', { name: 'Save' }).click();
        return res;
    };

    /** @type {any} */
    let original;
    test.beforeEach(async ({ page }) => {
        await page.goto('/auth/v1/admin/config/policy');
        original = await api(page, 'GET', 'password_policy');
    });
    test.afterEach(async ({ page }) => {
        // Always put the shared policy back, whatever the test did.
        await api(page, 'PUT', 'password_policy', policyRequest(original));
    });

    test('shows the current policy of the backend @mobile', async ({ page }) => {
        await expect(main(page).getByRole('heading', { name: 'Password Policy' })).toBeVisible();
        for (const [name, key] of FIELDS) {
            await expect(field(page, name)).toHaveValue(String(original[key] ?? 0));
        }
        await expect(field(page, 'Length min')).toHaveAttribute('aria-required', 'true');
        await expect(field(page, 'Length max')).toHaveAttribute('aria-required', 'true');
    });

    test('edits every field, persists it and updates the backend', async ({ page }) => {
        const next = {
            length_min: 16,
            length_max: 100,
            include_lower_case: 2,
            include_upper_case: 3,
            include_digits: 2,
            include_special: 1,
            not_recently_used: 5,
            valid_days: 90,
        };
        for (const [name, key] of FIELDS) {
            await field(page, name).fill(String(next[key]));
        }
        const res = await save(page);
        expect(res.status()).toBe(200);
        await expect(main(page).getByText(/cannot be lower|does not fit/)).toHaveCount(0);

        await page.reload();
        for (const [name, key] of FIELDS) {
            await expect(field(page, name)).toHaveValue(String(next[key]));
        }
        expect(await api(page, 'GET', 'password_policy')).toMatchObject(next);
    });

    test('a value of 0 removes an optional requirement', async ({ page }) => {
        await field(page, 'Special characters min').fill('2');
        await field(page, 'Valid For Days').fill('30');
        await save(page);
        expect(await api(page, 'GET', 'password_policy')).toMatchObject({
            include_special: 2,
            valid_days: 30,
        });

        for (const name of [
            'Lowercase letters min',
            'Uppercase letters min',
            'Digits min',
            'Special characters min',
            'Not one of last recent passwords',
            'Valid For Days',
        ]) {
            await field(page, name).fill('0');
        }
        expect((await save(page)).status()).toBe(200);
        const stored = await api(page, 'GET', 'password_policy');
        for (const key of [
            'include_lower_case',
            'include_upper_case',
            'include_digits',
            'include_special',
            'not_recently_used',
            'valid_days',
        ]) {
            expect(stored[key] ?? 0, key).toBe(0);
        }
        await page.reload();
        await expect(field(page, 'Valid For Days')).toHaveValue('0');
        await expect(field(page, 'Special characters min')).toHaveValue('0');
    });

    test('invalid input shows an error and blocks saving', async ({ page }) => {
        const puts = recordRequests(page, 'PUT', /\/auth\/v1\/password_policy$/);
        const saveBtn = main(page).getByRole('button', { name: 'Save' });

        // max below min
        await field(page, 'Length min').fill('20');
        await field(page, 'Length max').fill('16');
        await saveBtn.click();
        await expect(
            main(page).getByText('Max Length cannot be lower than Min Length'),
        ).toBeVisible();

        // the required character classes do not fit into max length
        await field(page, 'Length min').fill('8');
        await field(page, 'Length max').fill('8');
        await field(page, 'Lowercase letters min').fill('3');
        await field(page, 'Uppercase letters min').fill('3');
        await field(page, 'Digits min').fill('3');
        await saveBtn.click();
        await expect(
            main(page).getByText('The sum of all includes does not fit into Max Length'),
        ).toBeVisible();

        // below the allowed minimum length: native constraint, flagged on the field itself
        await field(page, 'Length max').fill('128');
        await field(page, 'Length min').fill('5');
        await expect(field(page, 'Length min')).toHaveAttribute('aria-invalid', 'true');
        await expect(main(page).getByText('Invalid input')).toBeVisible();
        await saveBtn.click();

        // validity window above 10 years
        await field(page, 'Length min').fill('14');
        await expect(field(page, 'Length min')).toHaveAttribute('aria-invalid', 'false');
        await field(page, 'Valid For Days').fill('4000');
        await expect(field(page, 'Valid For Days')).toHaveAttribute('aria-invalid', 'true');
        await saveBtn.click();

        // an empty required field
        await field(page, 'Valid For Days').fill('90');
        await field(page, 'Length max').fill('');
        await expect(field(page, 'Length max')).toHaveAttribute('aria-invalid', 'true');
        await saveBtn.click();

        expect(puts, 'no invalid policy may be sent').toEqual([]);
        expect(await api(page, 'GET', 'password_policy')).toEqual(original);
    });

    test.describe(() => {
        // The UI lets "not recently used" go up to 32, the backend only accepts 1..=10.
        test.use({ allowErrors: [/http 400 PUT \/auth\/v1\/password_policy/] });
        test('values the backend rejects show its error and change nothing', async ({ page }) => {
            // FINDING: PasswordPolicy.svelte allows max=32 for "not recently used", the API
            // validates 1..=10, so the user only learns about the limit from a server error.
            await field(page, 'Not one of last recent passwords').fill('11');
            const res = await save(page).catch(() => null);
            if (res) {
                expect(res.status()).toBe(400);
                await expect(main(page).getByText(/not_recently_used/)).toBeVisible();
            } else {
                await expect(field(page, 'Not one of last recent passwords')).toHaveAttribute(
                    'aria-invalid',
                    'true',
                );
            }
            expect(await api(page, 'GET', 'password_policy')).toEqual(original);
        });
    });

    test('the saved policy is enforced for new passwords', async ({ page }) => {
        const { user } = await createUser(page);
        const setPassword = (/** @type {string} */ password) =>
            // the same request the admin UI sends from the user's Password tab
            apiRaw(page, 'PATCH', `users/${user.id}`, {
                put: [{ key: 'password', value: password }],
                del: [],
            });
        try {
            await field(page, 'Length min').fill('24');
            await field(page, 'Special characters min').fill('2');
            expect((await save(page)).status()).toBe(200);

            // too short and without special characters
            const short = await setPassword('Short1Password2Abc');
            expect(short.status).toBe(400);
            // long enough, but only one special character
            const oneSpecial = await setPassword('AVeryLong1Password2ForE2e!');
            expect(oneSpecial.status).toBe(400);
            // fulfils the new policy
            const ok = await setPassword('AVeryLong1Password2ForE2e!?');
            expect(ok.status, ok.text).toBe(200);
        } finally {
            await apiRaw(page, 'DELETE', `users/${user.id}`);
        }
    });
});

test.describe('password hashing (argon2)', () => {
    test('shows the current parameters of the backend', async ({ page }) => {
        await page.goto('/auth/v1/admin/config/argon2');
        const lt = await api(page, 'GET', 'login_time');
        await expect(main(page).getByRole('heading', { name: 'Current values' })).toBeVisible();
        await expect(main(page).getByText(`${lt.argon2_params.m_cost} kB`)).toBeVisible();
        await expect(main(page).getByText(`${lt.argon2_params.p_cost} threads`)).toBeVisible();
        await expect(main(page).getByText(/Login Time:\s*\d+ ms/)).toBeVisible();
        await expect(
            main(page).locator('.stats > div').filter({ hasText: 'Rauthy has access to' }),
        ).toContainText(String(lt.num_cpus));
        // the calculation form starts with the current values
        await expect(main(page).getByRole('spinbutton', { name: 'Target Time (ms)' })).toHaveValue(
            '1000',
        );
        await expect(main(page).getByRole('spinbutton', { name: 'm_cost (kB)' })).toHaveValue(
            String(lt.argon2_params.m_cost),
        );
        await expect(main(page).getByRole('spinbutton', { name: 'p_cost' })).toHaveValue(
            String(lt.argon2_params.p_cost),
        );
    });

    test('calculates parameters for a target time and lists the results @mobile', async ({
        page,
    }) => {
        await page.goto('/auth/v1/admin/config/argon2');
        await main(page).getByRole('spinbutton', { name: 'Target Time (ms)' }).fill('500');
        await main(page).getByRole('spinbutton', { name: 'm_cost (kB)' }).fill('32768');
        await main(page).getByRole('spinbutton', { name: 'p_cost' }).fill('2');
        const resP = page.waitForResponse(r => r.url().endsWith('/auth/v1/password_hash_times'));
        await main(page).getByRole('button', { name: 'Calculate' }).click();
        const res = await resP;
        expect(res.request().postDataJSON()).toEqual({
            target_time: 500,
            m_cost: 32768,
            p_cost: 2,
        });
        const { results } = await res.json();
        expect(results.length).toBeGreaterThan(0);

        const table = main(page).getByRole('table');
        await expect(table.getByRole('columnheader', { name: 'ms', exact: true })).toBeVisible();
        for (const r of results) {
            await expect(
                table.getByRole('cell', { name: String(r.time_taken), exact: true }).first(),
            ).toBeVisible();
        }
        // the longest run reached the target
        expect(results[0].time_taken).toBeGreaterThanOrEqual(500);
        await expect(main(page).getByRole('button', { name: 'Calculate' })).toBeEnabled();
    });

    test('invalid parameters are flagged and not sent', async ({ page }) => {
        await page.goto('/auth/v1/admin/config/argon2');
        const posts = recordRequests(page, 'POST', /\/auth\/v1\/password_hash_times$/);
        const target = main(page).getByRole('spinbutton', { name: 'Target Time (ms)' });
        const mCost = main(page).getByRole('spinbutton', { name: 'm_cost (kB)' });

        await target.fill('100');
        await expect(target).toHaveAttribute('aria-invalid', 'true');
        await main(page).getByRole('button', { name: 'Calculate' }).click();

        await target.fill('600');
        await expect(target).toHaveAttribute('aria-invalid', 'false');
        await mCost.fill('1024');
        await expect(mCost).toHaveAttribute('aria-invalid', 'true');
        await main(page).getByRole('button', { name: 'Calculate' }).click();

        await target.fill('');
        await mCost.fill('32768');
        await expect(target).toHaveAttribute('aria-invalid', 'true');
        await main(page).getByRole('button', { name: 'Calculate' }).click();

        expect(posts).toEqual([]);
    });
});

test.describe('encryption keys', () => {
    test('lists the available keys, the active key and migrates to the active key', async ({
        page,
    }) => {
        await page.goto('/auth/v1/admin/config/encryption');
        const { active, keys } = await api(page, 'GET', 'encryption/keys');

        await expect(main(page).getByText(`Active Key: ${active}`)).toBeVisible();
        const list = main(page).getByRole('list');
        await expect(list.getByRole('listitem')).toHaveCount(keys.length);
        for (const k of keys) {
            await expect(list.getByRole('listitem').filter({ hasText: k })).toBeVisible();
        }

        if (keys.length < 2) {
            await expect(main(page).getByRole('button', { name: 'Migrate' })).toHaveCount(0);
            return;
        }

        // Migrating to the already active key is idempotent: only values that still use an
        // older key are re-encrypted (and the JWKs get rotated). Never migrate to another key.
        await page.route(/\/auth\/v1\/encryption\/migrate$/, route => {
            const body = route.request().postDataJSON();
            if (body.key_id === active) return route.continue();
            return route.abort();
        });
        const select = main(page).getByRole('combobox', {
            name: 'Migrate all existing encrypted values to the following key',
        });
        await expect(select).toHaveText(active);
        const other = keys.find((/** @type {string} */ k) => k !== active);
        await chooseOption(page, select, other);
        await expect(select).toHaveText(other);
        await chooseOption(page, select, active);
        await expect(select).toHaveText(active);

        // re-encrypting every value and rotating the JWKs (RSA key generation) takes about four
        // minutes on an unoptimized debug build
        test.setTimeout(360_000);
        const res = page.waitForResponse(r => r.url().endsWith('/auth/v1/encryption/migrate'), {
            timeout: 330_000,
        });
        await main(page).getByRole('button', { name: 'Migrate' }).click();
        expect((await res).status()).toBe(200);
        expect((await res).request().postDataJSON()).toEqual({ key_id: active });
        // FINDING: success is only a check icon without text or live region.
        await expect(main(page).getByRole('button', { name: 'Migrate' })).toBeEnabled();
        expect((await api(page, 'GET', 'encryption/keys')).active).toBe(active);
    });
});

test.describe('JWKS', () => {
    /** @param {import('@playwright/test').Page} page */
    const certs = async page => (await (await page.request.get('/auth/v1/oidc/certs')).json()).keys;

    test('lists every public key of the certs endpoint and shows its details', async ({ page }) => {
        await page.goto('/auth/v1/admin/config/jwks');
        const keys = await certs(page);
        for (const k of keys) {
            await expect(main(page).getByText(`${k.alg} / ${k.kid}`)).toBeVisible();
        }
        // FINDING: every toggle is named "Expand content" without saying which key it opens.
        const toggles = main(page).getByRole('button', { name: 'Expand content' });
        await expect(toggles).toHaveCount(keys.length);

        const first = keys[0];
        await toggles.first().click();
        // FINDING: labels and values of LabeledValue are not associated; `title` is the only
        // programmatic link between them.
        await expect(main(page).getByTitle('Key ID')).toHaveText(first.kid);
        await expect(main(page).getByTitle('Type', { exact: true })).toHaveText(first.kty);
        await expect(main(page).getByTitle('Algorithm', { exact: true })).toHaveText(first.alg);
        if (first.n) await expect(main(page).getByTitle('n', { exact: true })).toHaveText(first.n);
        if (first.e) await expect(main(page).getByTitle('e', { exact: true })).toHaveText(first.e);
        if (first.x) await expect(main(page).getByTitle('x', { exact: true })).toHaveText(first.x);

        await toggles.first().click();
        await expect(main(page).getByTitle('Key ID')).toHaveCount(0);
    });

    test('the expand toggle exposes its state', async ({ page }) => {
        // upstream set `aria-expanded` only on the wrapping div; the admin UI puts it on the
        // toggle button as well
        await page.goto('/auth/v1/admin/config/jwks');
        const toggle = main(page).getByRole('button', { name: 'Expand content' }).first();
        await expect(toggle).toHaveAttribute('aria-expanded', 'false', { timeout: 2000 });
        await toggle.click();
        await expect(toggle).toHaveAttribute('aria-expanded', 'true', { timeout: 2000 });
    });

    test('rotating adds new keys to the UI and the certs endpoint', async ({ page }) => {
        // RSA key generation is slow on an unoptimized debug build (minutes)
        test.setTimeout(360_000);
        await page.goto('/auth/v1/admin/config/jwks');
        const before = new Set((await certs(page)).map((/** @type {any} */ k) => k.kid));
        await expect(main(page).getByRole('button', { name: 'Expand content' })).toHaveCount(
            before.size,
        );

        const res = page.waitForResponse(r => r.url().endsWith('/auth/v1/oidc/rotate_jwk'), {
            timeout: 330_000,
        });
        await main(page).getByRole('button', { name: 'Rotate Keys' }).click();
        expect((await res).status()).toBe(200);

        const after = await certs(page);
        const added = after.filter((/** @type {any} */ k) => !before.has(k.kid));
        // one new key for every signing algorithm
        const algs = [...new Set(after.map((/** @type {any} */ k) => k.alg))].sort();
        // a rotation still running from an earlier migration may add a second set of keys
        expect([...new Set(added.map((/** @type {any} */ k) => k.alg))].sort()).toEqual(algs);
        for (const k of added) {
            await expect(main(page).getByText(`${k.alg} / ${k.kid}`)).toBeVisible();
        }
        await expect(main(page).getByRole('button', { name: 'Expand content' })).toHaveCount(
            after.length,
        );
    });
});

test.describe('backups', () => {
    test('creates a backup and lists it with a download link', async ({ page }) => {
        test.setTimeout(90_000);
        await page.goto('/auth/v1/admin/config/backups');
        const listing = await apiRaw(page, 'GET', 'backup');
        if (listing.status === 404) {
            // not a Hiqlite deployment: backups are disabled
            await expect(main(page).getByRole('button', { name: 'Create Backup' })).toHaveCount(0);
            return;
        }
        const before = new Set(
            JSON.parse(listing.text).local.map((/** @type {any} */ b) => b.name),
        );
        if (before.size === 0) {
            await expect(main(page).getByText('No Entries')).toBeVisible();
        }

        const res = page.waitForResponse(
            r => r.url().endsWith('/auth/v1/backup') && r.request().method() === 'POST',
            { timeout: 60_000 },
        );
        await main(page).getByRole('button', { name: 'Create Backup' }).click();
        expect((await res).ok()).toBe(true);

        /** @type {any} */
        let created;
        await expect
            .poll(async () => {
                const { local } = await api(page, 'GET', 'backup');
                created = local.find((/** @type {any} */ b) => !before.has(b.name));
                return !!created;
            })
            .toBe(true);

        // the page reloads the listing on its own shortly after a backup was triggered
        await expect(main(page).getByRole('heading', { name: 'Local' })).toBeVisible();
        await expect(main(page).getByRole('heading', { name: 'S3' })).toBeVisible();
        const link = main(page).getByRole('link', { name: created.name });
        await expect(link).toBeVisible();
        await expect(link).toHaveAttribute('href', `/auth/v1/backup/local/${created.name}`);

        const reload = main(page).getByRole('button', { name: 'Refresh', exact: true });
        const refetch = page.waitForResponse(
            r => r.url().endsWith('/auth/v1/backup') && r.request().method() === 'GET',
        );
        await reload.click();
        expect((await refetch).status()).toBe(200);
        await expect(link).toBeVisible();
    });
});

test.describe('terms of service', () => {
    // A stored ToS is immutable and every user (the shared admin included) would have to accept
    // it at the next login, which breaks every other login test. The writes are therefore
    // answered by a mock; the UI is still driven end to end and the payload is verified.
    /** @type {any[]} */
    let tosList;
    /** @type {any[]} */
    let posted;

    test.beforeEach(async ({ page }) => {
        tosList = [];
        posted = [];
        await page.route(/\/auth\/v1\/tos$/, async route => {
            if (route.request().method() === 'POST') {
                const body = route.request().postDataJSON();
                posted.push(body);
                tosList.push({
                    ts: Math.floor(Date.now() / 1000),
                    author: 'admin@localhost',
                    ...body,
                });
                return route.fulfill({ status: 201, body: '' });
            }
            return route.fulfill({ json: tosList });
        });
    });

    /** @param {import('@playwright/test').Page} page */
    const openToS = async page => {
        await page.goto('/auth/v1/admin/config/tos');
        await expect(MARKER.tos(page)).toBeVisible();
    };

    test('without any ToS only "New ToS" is offered', async ({ page }) => {
        await openToS(page);
        await expect(
            main(page).getByRole('button', { name: 'New ToS', exact: true }),
        ).toBeVisible();
        await expect(main(page).getByRole('button', { name: 'Check user status' })).toHaveCount(0);
        await expect(main(page).getByRole('button', { name: 'New ToS from selected' })).toHaveCount(
            0,
        );
    });

    test('the empty state explains that no ToS exist yet', async ({ page }) => {
        await openToS(page);
        await expect(main(page).getByText('No Terms of Service have been added yet.')).toBeVisible({
            timeout: 3000,
        });
    });

    test('create dialog: validation, transition time and cancel', async ({ page }) => {
        await openToS(page);
        await main(page).getByRole('button', { name: 'New ToS', exact: true }).click();
        const dialog = page.getByRole('dialog');
        await expect(dialog.getByRole('heading', { name: 'New ToS' })).toBeVisible();
        await expect(dialog.getByText(/immutable and cannot be\s+deleted/)).toBeVisible();

        const save = dialog.getByRole('button', { name: 'Save' });
        const editor = dialog.getByRole('textbox', { name: 'Edit Text' });
        await expect(save).toBeDisabled();
        await editor.fill('   ');
        await expect(save).toBeDisabled();
        await editor.fill('Some terms');
        await expect(save).toBeEnabled();

        // transition time is optional and reveals date and time inputs
        const optUntil = dialog.getByRole('checkbox', { name: 'Enable Transition Time' });
        await expect(optUntil).not.toBeChecked();
        await expect(dialog.getByText(/accepting updated ToS is optional/)).toHaveCount(0);
        await optUntil.check();
        await expect(dialog.getByText(/accepting updated ToS is optional/)).toBeVisible();
        await expect(dialog.locator('input[type="date"]')).toHaveValue(dateInput(0));
        await optUntil.uncheck();
        await expect(dialog.getByText(/accepting updated ToS is optional/)).toHaveCount(0);

        await dialog.getByRole('button', { name: 'Cancel' }).click();
        await expect(dialog).toBeHidden();

        // the close icon closes as well
        await main(page).getByRole('button', { name: 'New ToS', exact: true }).click();
        await expect(page.getByRole('dialog')).toBeVisible();
        await page.getByRole('dialog').getByRole('button', { name: 'Close' }).click();
        await expect(page.getByRole('dialog')).toBeHidden();
        expect(posted).toEqual([]);
    });

    test('creates a markdown ToS with transition time and renders it', async ({ page }) => {
        await openToS(page);
        await main(page).getByRole('button', { name: 'New ToS', exact: true }).click();
        const dialog = page.getByRole('dialog');

        const content = '# E2E Terms\n\nYou **must** agree.';
        await dialog.getByRole('textbox', { name: 'Edit Text' }).fill(content);
        const preview = dialog.getByRole('button', { name: 'Preview', exact: true });
        const edit = dialog.getByRole('button', { name: 'Edit', exact: true });
        await expect(edit).toHaveAttribute('aria-pressed', 'true');
        await preview.click();
        await expect(preview).toHaveAttribute('aria-pressed', 'true');
        await expect(edit).toHaveAttribute('aria-pressed', 'false');
        await expect(dialog.getByRole('heading', { name: 'E2E Terms' })).toBeVisible();

        await dialog.getByRole('checkbox', { name: 'Enable Transition Time' }).check();
        const date = dateInput(30);
        await dialog.locator('input[type="date"]').fill(date);
        const time = dialog.locator('input[name="time"]');
        await enterTime(time, '12:30');
        await time.blur();

        await dialog.getByRole('button', { name: 'Save' }).click();
        await expect(dialog).toBeHidden();
        expect(posted).toHaveLength(1);
        // the editor stores an extra blank line after the heading (upstream, harmless for
        // markdown rendering), so blank lines are compared collapsed
        expect({
            ...posted[0],
            content: posted[0].content.replace(/\n{2,}/g, '\n\n'),
        }).toEqual({
            is_html: true,
            content,
            opt_until: berlinTs(date, '12:30'),
        });

        // the listing shows the new version, its dates and the rendered content
        const tos = tosList[0];
        await expect(main(page).getByRole('heading', { name: 'E2E Terms' })).toBeVisible();
        await expect(main(page).getByText('You must agree.')).toBeVisible();
        await expect(main(page).getByTitle('Added', { exact: true })).toHaveText(fmtTs(tos.ts));
        await expect(main(page).getByTitle('End of Transition Time')).toHaveText(
            fmtTs(tos.opt_until),
        );
        await expect(main(page).getByRole('button', { name: 'Check user status' })).toBeVisible();
        await expect(
            main(page).getByRole('button', { name: 'New ToS from selected' }),
        ).toBeVisible();
        // a single version needs no version selector
        await expect(main(page).getByRole('combobox', { name: 'Select ToS' })).toHaveCount(0);
    });

    test('creates a plain text ToS without transition time', async ({ page }) => {
        await openToS(page);
        await main(page).getByRole('button', { name: 'New ToS', exact: true }).click();
        const dialog = page.getByRole('dialog');
        await dialog.getByRole('button', { name: 'Text', exact: true }).click();
        await expect(dialog.getByRole('button', { name: 'Text', exact: true })).toHaveAttribute(
            'aria-current',
            'step',
        );
        await dialog.getByRole('textbox', { name: 'Edit Text' }).fill('Plain *terms*');
        await dialog.getByRole('button', { name: 'Save' }).click();
        await expect(dialog).toBeHidden();
        expect(posted).toEqual([{ is_html: false, content: 'Plain *terms*' }]);
        // plain text is shown verbatim, not rendered
        await expect(main(page).getByText('Plain *terms*')).toBeVisible();
    });

    test('versions: select an older one, start a new one from it, check user status', async ({
        page,
    }) => {
        const now = Math.floor(Date.now() / 1000);
        tosList.push(
            { ts: now - 86_400 * 10, author: 'a', is_html: true, content: '# Old Terms' },
            {
                ts: now - 3600,
                author: 'a',
                is_html: true,
                opt_until: now + 86_400 * 5,
                content: '# New Terms',
            },
        );
        const { user, email } = await createUser(page);
        await page.route(new RegExp(`/auth/v1/tos/user/${user.id}$`), route =>
            route.fulfill({
                json: [
                    {
                        user_id: user.id,
                        tos_ts: tosList[0].ts,
                        accept_ts: tosList[0].ts + 60,
                        location: 'E2E Town',
                    },
                ],
            }),
        );
        try {
            await openToS(page);
            // the latest version is selected by default
            const select = main(page).getByRole('combobox', { name: 'Select ToS' });
            await expect(select).toHaveText(`${fmtTs(tosList[1].ts)} (Latest)`);
            await expect(main(page).getByRole('heading', { name: 'New Terms' })).toBeVisible();
            await expect(main(page).getByTitle('End of Transition Time')).toBeVisible();

            await chooseOption(page, select, fmtTs(tosList[0].ts));
            await expect(main(page).getByRole('heading', { name: 'Old Terms' })).toBeVisible();
            await expect(main(page).getByTitle('Added', { exact: true })).toHaveText(
                fmtTs(tosList[0].ts),
            );
            await expect(main(page).getByTitle('End of Transition Time')).toHaveCount(0);

            // "New ToS from selected" pre-fills the editor with the selected version
            await main(page).getByRole('button', { name: 'New ToS from selected' }).click();
            const dialog = page.getByRole('dialog');
            await expect(dialog.getByRole('textbox', { name: 'Edit Text' })).toHaveText(
                '# Old Terms',
            );
            await dialog.getByRole('button', { name: 'Cancel' }).click();
            await expect(dialog).toBeHidden();

            // user status: search a user, it is selected automatically when it is the only hit
            await main(page).getByRole('button', { name: 'Check user status' }).click();
            const status = page.getByRole('dialog');
            await expect(status.getByRole('heading', { name: 'Check user status' })).toBeVisible();
            await status.getByRole('searchbox', { name: 'Search' }).fill(email);
            await expect(status.getByRole('button', { name: `<${email}>` })).toBeVisible();
            await expect(status.getByRole('heading', { name: email })).toBeVisible();
            await expect(status.getByText(`${fmtTs(tosList[0].ts + 60)} - E2E Town`)).toBeVisible();
            await expect(status.getByText(fmtTs(tosList[1].ts))).toBeVisible();
            await status.getByRole('button', { name: 'Close' }).last().click();
            await expect(status).toBeHidden();
            expect(posted).toEqual([]);
        } finally {
            await apiRaw(page, 'DELETE', `users/${user.id}`);
        }
    });

    test('ToS editor works at phone size @mobile', async ({ page, isMobile }) => {
        test.skip(!isMobile, 'phone layout only');
        await page.goto('/auth/v1/admin/config/policy');
        const tosLink = subNav(page).getByRole('link', { name: 'ToS', exact: true });
        await openSubNav(page, tosLink);
        await tosLink.click();
        await main(page).getByRole('button', { name: 'New ToS', exact: true }).click();
        const dialog = page.getByRole('dialog');
        await dialog.getByRole('textbox', { name: 'Edit Text' }).fill('# Phone Terms');
        // below 1440px the preview is a toggle inside the editor
        await dialog.getByRole('button', { name: 'Preview', exact: true }).click();
        await expect(dialog.getByRole('heading', { name: 'Phone Terms' })).toBeVisible();
        await dialog.getByRole('button', { name: 'Save' }).click();
        await expect(dialog).toBeHidden();
        expect(posted).toEqual([{ is_html: true, content: '# Phone Terms' }]);
        await expect(main(page).getByRole('heading', { name: 'Phone Terms' })).toBeVisible();
    });
});

// Creating a real ToS cannot be undone and forces every user to accept it at the next login, so
// it is opt-in for disposable backends only: E2E_ALLOW_PERMANENT_TOS=1.
test.describe('terms of service (persistent, opt-in)', () => {
    test.skip(!process.env.E2E_ALLOW_PERMANENT_TOS, 'creates an immutable ToS for all users');

    test('stores a new ToS in the backend', async ({ page }) => {
        await page.goto('/auth/v1/admin/config/policy');
        await subNav(page).getByRole('link', { name: 'ToS', exact: true }).click();
        const content = `# Terms ${Date.now()}`;
        await main(page).getByRole('button', { name: 'New ToS', exact: true }).click();
        await page.getByRole('dialog').getByRole('textbox', { name: 'Edit Text' }).fill(content);
        await page.getByRole('dialog').getByRole('button', { name: 'Save' }).click();
        await expect(page.getByRole('dialog')).toBeHidden();
        await expect
            .poll(async () =>
                (await api(page, 'GET', 'tos')).map((/** @type {any} */ t) => t.content),
            )
            .toContain(content);
        await expect(main(page).getByRole('heading', { name: content.slice(2) })).toBeVisible();
    });
});
