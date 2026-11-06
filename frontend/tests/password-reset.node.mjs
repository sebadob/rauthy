// Built-UI contract tests, not backend authorization tests. All requests are intercepted;
// credentials and reset context exist only in memory, without traces or a mail server.
import assert from 'node:assert/strict';
import { after, before, test } from 'node:test';
import {
    addVirtualAuthenticator,
    injectTemplates,
    launchChromium,
    readBuiltHtml,
    serveUi,
} from './helpers/browser.mjs';

/** @type {any} */
let browser;
/** @type {any} */
let html;
before(async () => {
    html = await readBuiltHtml('users/{id}/reset/reset.html');
    browser = await launchChromium();
});
after(async () => browser?.close());

for (const scenario of [
    'password-only',
    'passkey',
    'non-resident-key',
    'start-rejected',
    'finish-rejected',
    'put-rejected',
    'authenticator-timeout',
]) {
    test(`reset page: ${scenario}`, async t => {
        const context = await browser.newContext({ serviceWorkers: 'block' });
        t.after(() => context.close());
        const page = await context.newPage();
        const tpl = {
            user_id: 'reset-user',
            magic_link_id: crypto.randomUUID(),
            csrf_token: crypto.randomUUID(),
            needs_mfa: scenario !== 'password-only',
            password_policy: {
                length_min: 12,
                length_max: 128,
                include_lower_case: 1,
                include_upper_case: 1,
                include_digits: 1,
                include_special: 1,
            },
        };
        /** @type {any} */
        let challengeCode;
        /** @type {any} */
        let mfaCode;
        /** @type {any} */
        let createdCredId;
        let startRejected = false;
        let finishRejected = false;
        let putRejected = false;
        /** @type {string[]} */
        const requests = [];
        const documentPath = '/auth/v1/users/reset-user/reset/reset';
        /**
         * @param {any} route
         * @param {any} request
         * @param {any} url
         */
        const handle = async (route, request, url) => {
            if (url.pathname === '/auth/v1/users/reset-user/webauthn/auth/start') {
                assert.deepEqual(request.postDataJSON(), { purpose: 'PasswordReset' });
                if (scenario === 'start-rejected' && !startRejected) {
                    startRejected = true;
                    return {
                        status: 400,
                        json: { message: 'Reset binding rejected' },
                    };
                }
                challengeCode = crypto.randomUUID();
                /** @type {any} */
                const publicKey = {
                    challenge: btoa(crypto.randomUUID()),
                    rpId: 'localhost',
                    userVerification: 'required',
                };
                if (createdCredId) {
                    publicKey.allowCredentials = [{ id: createdCredId, type: 'public-key' }];
                }
                return {
                    // Short expiry for the timeout scenario so the test does not
                    // wait out the full WebAuthn timeout with no authenticator present.
                    json: {
                        code: challengeCode,
                        exp: scenario === 'authenticator-timeout' ? 6 : 60,
                        rcr: { publicKey },
                    },
                };
            }
            if (url.pathname === '/auth/v1/users/webauthn_finish') {
                assert.ok(request.postDataJSON().code === challengeCode);
                if (scenario === 'finish-rejected' && !finishRejected) {
                    finishRejected = true;
                    return {
                        status: 403,
                        json: { message: 'Verification rejected' },
                    };
                }
                mfaCode = crypto.randomUUID();
                return {
                    status: 202,
                    json: { code: mfaCode, user_id: tpl.user_id },
                };
            }
            if (url.pathname === '/auth/v1/users/reset-user/reset') {
                const body = request.postDataJSON();
                assert.equal(request.method(), 'PUT');
                assert.ok(body.magic_link_id === tpl.magic_link_id);
                assert.ok(request.headers()['x-pwd-csrf-token'] === tpl.csrf_token);
                assert.ok(body.mfa_code === (tpl.needs_mfa ? mfaCode : undefined));
                assert.ok(typeof body.password === 'string' && body.password.length >= 12);
                if (scenario === 'put-rejected' && !putRejected) {
                    putRejected = true;
                    return { status: 400, json: { message: 'Reset rejected' } };
                }
                return { status: 202 };
            }
            return { status: 400, json: { message: 'Unexpected endpoint' } };
        };
        await serveUi({
            context,
            documentPath,
            html: injectTemplates(html, { tpl_password_reset: JSON.stringify(tpl) }),
            requests,
            handle,
        });
        await page.goto(`http://localhost${documentPath}`);
        await page.locator('input[type=password]').first().waitFor();
        const { cdp, authenticatorId } = await addVirtualAuthenticator(context, page);
        if (scenario === 'authenticator-timeout') {
            // No authenticator is available, so the prompt times out and
            // nothing must be sent beyond the already-issued start request.
            await cdp.send('WebAuthn.removeVirtualAuthenticator', { authenticatorId });
            await page.evaluate(() => {
                const password = `${crypto.randomUUID()}aA1!`;
                for (const elem of document.querySelectorAll('input[type=password]')) {
                    /** @type {any} */
                    const input = elem;
                    input.value = password;
                    input.dispatchEvent(new Event('input', { bubbles: true }));
                }
            });
        } else {
            createdCredId = await page.evaluate(
                /** @param {{createCredential: boolean, residentKey: ResidentKeyRequirement}} p */ async ({
                    createCredential,
                    residentKey,
                }) => {
                    let credentialId;
                    if (createCredential) {
                        const credential = await navigator.credentials.create({
                            publicKey: {
                                challenge: crypto.getRandomValues(new Uint8Array(32)),
                                rp: { name: 'Reset test', id: 'localhost' },
                                user: {
                                    id: crypto.getRandomValues(new Uint8Array(16)),
                                    name: 'reset-user',
                                    displayName: 'Reset user',
                                },
                                pubKeyCredParams: [{ type: 'public-key', alg: -7 }],
                                authenticatorSelection: {
                                    residentKey,
                                    userVerification: 'required',
                                },
                            },
                        });
                        credentialId = credential?.id;
                    }
                    const password = `${crypto.randomUUID()}aA1!`;
                    for (const elem of document.querySelectorAll('input[type=password]')) {
                        /** @type {any} */
                        const input = elem;
                        input.value = password;
                        input.dispatchEvent(new Event('input', { bubbles: true }));
                    }
                    return credentialId;
                },
                {
                    createCredential: tpl.needs_mfa,
                    residentKey: scenario === 'non-resident-key' ? 'discouraged' : 'required',
                },
            );
        }
        await page.getByRole('button', { name: 'Save', exact: true }).click();
        const startReq = 'POST /auth/v1/users/reset-user/webauthn/auth/start';
        const finishReq = 'POST /auth/v1/users/webauthn_finish';
        const putReq = 'PUT /auth/v1/users/reset-user/reset';
        if (scenario === 'authenticator-timeout') {
            await page.locator('.err').first().waitFor();
            assert.deepEqual(requests, [startReq]);
            return;
        }
        if (scenario === 'start-rejected' || scenario === 'finish-rejected') {
            await page
                .getByText(
                    scenario === 'start-rejected'
                        ? 'Reset binding rejected'
                        : 'Verification rejected',
                    {
                        exact: true,
                    },
                )
                .first()
                .waitFor();
            assert.deepEqual(
                requests,
                scenario === 'start-rejected' ? [startReq] : [startReq, finishReq],
            );
            await page.getByRole('button', { name: 'Save', exact: true }).click();
            await page.getByRole('link', { name: 'Account', exact: true }).waitFor();
            assert.deepEqual(requests.slice(-3), [startReq, finishReq, putReq]);
            return;
        }
        if (scenario === 'put-rejected') {
            await page.getByText('Reset rejected', { exact: true }).first().waitFor();
            assert.ok(
                (await page.getByRole('link', { name: 'Account', exact: true }).count()) === 0,
            );
            assert.deepEqual(requests, [startReq, finishReq, putReq]);
            // A rejected update consumes the MFA proof: retry needs a fresh ceremony.
            await page.getByRole('button', { name: 'Save', exact: true }).click();
            await page.getByRole('link', { name: 'Account', exact: true }).waitFor();
            assert.deepEqual(requests.slice(-3), [startReq, finishReq, putReq]);
            return;
        }
        await page.getByRole('link', { name: 'Account', exact: true }).waitFor();
        const expected = tpl.needs_mfa ? [startReq, finishReq, putReq] : [putReq];
        assert.deepEqual(requests, expected);
    });
}
