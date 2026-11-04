// Built-UI contract test for the account page's MFA section: which requests the UI sends to the
// backend around the re-authentication, including the paths where the user interrupts the flow.
// All requests are intercepted; session, user, and tokens only exist in memory.
import assert from 'node:assert/strict';
import { after, before, test } from 'node:test';
import {
    addVirtualAuthenticator,
    createPasskeyCredential,
    injectTemplates,
    launchChromium,
    ORIGIN,
    readBuiltHtml,
    serveUi,
} from './helpers/browser.mjs';

const USER_ID = 'account-user';
const DOCUMENT_PATH = '/auth/v1/account';
const PASSWORD = 'CurrentPassword1!';

// The backend only fills the template loop when it serves a page, so the test injects the values.
const PAGE_TEMPLATES = { tpl_is_otp_enabled: 'true', tpl_otp_length: '6' };

const SESSION = {
    id: 'session-id',
    csrf_token: 'csrf-token',
    user_id: USER_ID,
    roles: '',
    groups: '',
    exp: '1700003600',
    timeout: 'session_long',
    state: 'Auth',
};

const USER = {
    id: USER_ID,
    email: 'account@localhost',
    language: 'en',
    roles: [],
    groups: [],
    enabled: true,
    email_verified: true,
    created_at: 1700000000,
    account_type: 'password',
    webauthn_user_id: undefined,
    user_values: {},
};

const PASSKEY = { name: 'Testkey', registered: 1700000000, last_used: 1700000000 };

const MODAL_PASSWORD =
    'Before you can modify your MFA, you need to re-authenticate with your password.';
const MODAL_PASSKEY = 'Before you can modify your MFA, you need to authenticate with a passkey:';
const ACTIVATION_INPUT = 'Please enter the OTP code that was sent to your email address.';
const PASSWORD_INVALID = 'Password Invalid';
const CREATION_FAILED = 'Error starting the Registration process';
const CAN_MODIFY_FOR = 'MFA can be modified for:';

const OTP_BUTTON = 'Register New OTP';

const MFA_TOKEN = `POST /auth/v1/users/${USER_ID}/mfa_token`;
const OTP_POST = `POST /auth/v1/users/${USER_ID}/otp`;
const OTP_PUT = `PUT /auth/v1/users/${USER_ID}/otp`;
const WEBAUTHN_START = 'POST /auth/v1/users/webauthn_start';
const WEBAUTHN_FINISH = 'POST /auth/v1/users/webauthn_finish';
const PASSKEY_DELETE = `DELETE /auth/v1/users/${USER_ID}/webauthn/delete/Testkey`;

/** @type {any[]} */
const NO_PASSKEY = [];
/** @type {any[]} */
const NO_OTP = [];

const SCENARIOS = [
    {
        // The regular case for this issue: the form only appears after the request was sent, and
        // the activation the user triggers by typing the code runs exactly once.
        name: 'password',
        webauthnUserId: undefined,
        passkeys: NO_PASSKEY,
        otps: NO_OTP,
        authenticator: false,
        expected: [MFA_TOKEN, OTP_POST, OTP_PUT],
        async run(/** @type {any} */ page, /** @type {any} */ s) {
            await page.getByRole('button', { name: OTP_BUTTON, exact: true }).click();
            await page.getByText(MODAL_PASSWORD).waitFor();
            await authenticateWithPassword(page, PASSWORD);
            await page.getByText(ACTIVATION_INPUT).waitFor();
            await page.locator('input[name="otp"]').fill('123456');
            await page.getByText('Registered OTPs').waitFor();
        },
    },
    {
        name: 'passkey-ceremony',
        webauthnUserId: 'webauthn-user-id',
        passkeys: [PASSKEY],
        otps: NO_OTP,
        authenticator: true,
        expected: [WEBAUTHN_START, WEBAUTHN_FINISH, MFA_TOKEN, OTP_POST],
        async run(/** @type {any} */ page, /** @type {any} */ s) {
            await page.getByRole('button', { name: OTP_BUTTON, exact: true }).click();
            await page.getByText(MODAL_PASSKEY).waitFor();
            await page.getByRole('button', { name: 'Authenticate', exact: true }).click();
            await page.getByText(ACTIVATION_INPUT).waitFor();
        },
    },
    {
        // A rejected re-authentication must not have created anything, and the next attempt has
        // to resume the registration the user had clicked before.
        name: 'password-rejected',
        webauthnUserId: undefined,
        passkeys: NO_PASSKEY,
        otps: NO_OTP,
        authenticator: false,
        expected: [MFA_TOKEN, MFA_TOKEN, OTP_POST],
        async run(/** @type {any} */ page, /** @type {any} */ s) {
            await page.getByRole('button', { name: OTP_BUTTON, exact: true }).click();
            await authenticateWithPassword(page, 'WrongPassword1!');
            await page.getByText(PASSWORD_INVALID).first().waitFor();
            assert.deepEqual(s.requests, [MFA_TOKEN]);
            await authenticateWithPassword(page, PASSWORD);
            await page.getByText(ACTIVATION_INPUT).waitFor();
        },
    },
    {
        name: 'cancel-then-retry',
        webauthnUserId: undefined,
        passkeys: NO_PASSKEY,
        otps: NO_OTP,
        authenticator: false,
        expected: [MFA_TOKEN, OTP_POST],
        async run(/** @type {any} */ page, /** @type {any} */ s) {
            await page.getByRole('button', { name: OTP_BUTTON, exact: true }).click();
            await page.getByText(MODAL_PASSWORD).waitFor();
            await page.getByRole('button', { name: 'Close', exact: true }).click();
            await page.getByRole('button', { name: OTP_BUTTON, exact: true }).waitFor();
            await page.getByRole('button', { name: OTP_BUTTON, exact: true }).click();
            await authenticateWithPassword(page, PASSWORD);
            await page.getByText(ACTIVATION_INPUT).waitFor();
        },
    },
    {
        // The interrupted registration must not be resumed by a later token fetch that belongs to
        // another action, and that action still has to work on its own.
        name: 'cancel-then-unrelated-action',
        webauthnUserId: 'webauthn-user-id',
        passkeys: [PASSKEY],
        otps: NO_OTP,
        authenticator: true,
        expected: [WEBAUTHN_START, WEBAUTHN_FINISH, MFA_TOKEN, PASSKEY_DELETE],
        async run(/** @type {any} */ page, /** @type {any} */ s) {
            await page.getByRole('button', { name: OTP_BUTTON, exact: true }).click();
            await page.getByText(MODAL_PASSKEY).waitFor();
            await page.getByRole('button', { name: 'Close', exact: true }).click();
            await page.getByRole('button', { name: OTP_BUTTON, exact: true }).waitFor();
            await page.getByRole('button', { name: 'Delete', exact: true }).click();
            await page.getByRole('button', { name: 'Authenticate', exact: true }).click();
            await page.getByText(CAN_MODIFY_FOR).first().waitFor();
            // The token is valid now, so this second click performs the deletion right away, and
            // the vanished passkey is the proof that its request was the last one.
            await page.getByRole('button', { name: 'Delete', exact: true }).click();
            await page.getByText('Testkey').waitFor({ state: 'detached' });
        },
    },
    {
        // The token is still valid, so the second click must not ask again, and a failed creation
        // shows the error instead of the form.
        name: 'create-failed-then-retry',
        webauthnUserId: undefined,
        passkeys: NO_PASSKEY,
        otps: NO_OTP,
        authenticator: false,
        // The backend rejects the first creation, the second one succeeds.
        createRejected: true,
        expected: [MFA_TOKEN, OTP_POST, OTP_POST],
        async run(/** @type {any} */ page, /** @type {any} */ s) {
            await page.getByRole('button', { name: OTP_BUTTON, exact: true }).click();
            await authenticateWithPassword(page, PASSWORD);
            await page.getByText(CREATION_FAILED).first().waitFor();
            assert.equal(await page.getByText(ACTIVATION_INPUT).count(), 0);
            await page.getByRole('button', { name: OTP_BUTTON, exact: true }).click();
            await page.getByText(ACTIVATION_INPUT).waitFor();
        },
    },
    {
        // A second creation while the token is still valid must not fetch a new one.
        name: 'existing-token-reuse',
        webauthnUserId: undefined,
        passkeys: NO_PASSKEY,
        otps: NO_OTP,
        authenticator: false,
        expected: [MFA_TOKEN, OTP_POST, OTP_POST],
        async run(/** @type {any} */ page, /** @type {any} */ s) {
            await page.getByRole('button', { name: OTP_BUTTON, exact: true }).click();
            await authenticateWithPassword(page, PASSWORD);
            await page.getByText(ACTIVATION_INPUT).waitFor();
            await page.getByRole('button', { name: 'Cancel', exact: true }).click();
            await page.getByRole('button', { name: OTP_BUTTON, exact: true }).waitFor();
            await page.getByRole('button', { name: OTP_BUTTON, exact: true }).click();
            await page.getByText(ACTIVATION_INPUT).waitFor();
        },
    },
    {
        // A federated-only account gets the token directly, without a modal, and only once.
        name: 'federated-only',
        accountType: 'federated',
        webauthnUserId: undefined,
        passkeys: NO_PASSKEY,
        otps: NO_OTP,
        authenticator: false,
        expected: [MFA_TOKEN, OTP_POST],
        async run(/** @type {any} */ page, /** @type {any} */ s) {
            await page.getByRole('button', { name: OTP_BUTTON, exact: true }).click();
            await page.getByText(ACTIVATION_INPUT).waitFor();
            assert.equal(await page.getByText(MODAL_PASSWORD).count(), 0);
        },
    },
];

/** @type {any} */
let browser;
/** @type {any} */
let html;
before(async () => {
    html = injectTemplates(await readBuiltHtml('account.html'), PAGE_TEMPLATES);
    browser = await launchChromium();
});
after(async () => browser?.close());

for (const scenario of SCENARIOS) {
    test(`account mfa: ${scenario.name}`, async t => {
        const context = await browser.newContext({ serviceWorkers: 'block' });
        t.after(() => context.close());
        const page = await context.newPage();
        const s = {
            scenario,
            requests: [],
            code: undefined,
            mfaCode: undefined,
            tokenId: undefined,
            deleted: false,
        };
        await serveUi({
            context,
            documentPath: DOCUMENT_PATH,
            html,
            requests: s.requests,
            handle: (route, request, url) => handle(s, route, request, url),
        });
        await page.goto(`${ORIGIN}${DOCUMENT_PATH}`);
        await page.getByRole('button', { name: OTP_BUTTON, exact: true }).waitFor();
        if (scenario.authenticator) {
            await addVirtualAuthenticator(context, page);
            await createPasskeyCredential(page, { rpName: 'Rauthy', userId: USER_ID });
        }
        await scenario.run(page, s);
        assert.deepEqual(s.requests, scenario.expected);
    });
}

/**
 * @param {any} page
 * @param {string} password
 */
async function authenticateWithPassword(page, password) {
    await page.locator('input[name="password"]').fill(password);
    await page.getByRole('button', { name: 'Authenticate', exact: true }).click();
}

/**
 * @param {any} s
 * @param {any} route
 * @param {any} request
 * @param {any} url
 */
async function handle(s, route, request, url) {
    const scenario = s.scenario;
    const method = request.method();
    if (method === 'GET') {
        if (url.pathname === '/auth/v1/oidc/sessioninfo') {
            return { json: SESSION };
        }
        if (url.pathname === `/auth/v1/users/${USER_ID}`) {
            return {
                json: {
                    ...USER,
                    account_type: scenario.accountType ?? 'password',
                    webauthn_user_id: scenario.webauthnUserId,
                },
            };
        }
        if (url.pathname === `/auth/v1/users/${USER_ID}/webauthn`) {
            return { json: s.deleted ? [] : scenario.passkeys };
        }
        if (url.pathname === `/auth/v1/users/${USER_ID}/otp`) {
            return { json: scenario.otps };
        }
        return;
    }

    if (url.pathname === `/auth/v1/users/${USER_ID}/mfa_token`) {
        const payload = request.postDataJSON();
        if ('password' in payload && payload.password !== PASSWORD) {
            return { status: 400, json: { message: 'Wrong credentials' } };
        }
        if (s.mfaCode && 'mfa_code' in payload) {
            assert.equal(payload.mfa_code, s.mfaCode, 'the UI must forward the ceremony code');
        } else if (!('password' in payload) && !s.mfaCode) {
            assert.deepEqual(
                payload,
                {},
                'a federated-only account proves itself with an empty body',
            );
        }
        s.tokenId = crypto.randomUUID();
        return {
            json: { id: s.tokenId, user_id: USER_ID, exp: 1800000000, ip: '127.0.0.1' },
        };
    }

    if (url.pathname === `/auth/v1/users/${USER_ID}/otp` && method === 'POST') {
        assert.equal(request.postDataJSON().mfa_mod_token_id, s.tokenId);
        if (scenario.createRejected && !s.rejected) {
            s.rejected = true;
            return { status: 400, json: { message: 'OTP creation rejected' } };
        }
        return {
            status: 201,
            json: {
                id: 'otp-id',
                name: 'account@localhost',
                last_used: 1700000000,
                kind: 'email',
                is_active: false,
            },
        };
    }

    if (url.pathname === `/auth/v1/users/${USER_ID}/otp` && method === 'PUT') {
        const payload = request.postDataJSON();
        assert.equal(payload.otp_code, '123456');
        assert.equal(payload.mfa_mod_token_id, s.tokenId);
        return { status: 200 };
    }

    if (url.pathname === '/auth/v1/users/webauthn_start') {
        assert.deepEqual(request.postDataJSON(), { purpose: 'MfaModToken' });
        s.code = crypto.randomUUID();
        return {
            json: {
                code: s.code,
                exp: 60,
                rcr: {
                    publicKey: {
                        challenge: btoa(crypto.randomUUID()),
                        rpId: 'localhost',
                        userVerification: 'required',
                    },
                },
            },
        };
    }

    if (url.pathname === '/auth/v1/users/webauthn_finish') {
        assert.equal(request.postDataJSON().code, s.code);
        s.mfaCode = crypto.randomUUID();
        return { status: 202, json: { code: s.mfaCode, user_id: USER_ID } };
    }

    if (url.pathname === `/auth/v1/users/${USER_ID}/webauthn/delete/Testkey`) {
        assert.equal(request.postDataJSON().mfa_mod_token_id, s.tokenId);
        s.deleted = true;
        return { status: 200 };
    }
}
