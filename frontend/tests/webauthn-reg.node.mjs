// Run with: npm run test:webauthn-reg
// Frontend contract tests only: no backend, accounts, or network access.
import assert from 'node:assert/strict';
import { resolve } from 'node:path';
import { after, before, test } from 'node:test';
import { createServer } from 'vite';

import { loadConfig } from '@sveltejs/load-config';

/** @type {any} */
let server;
/** @type {any} */
let webauthnReg;
const storageDescriptor = Object.getOwnPropertyDescriptor(globalThis, 'localStorage');
const navigatorDescriptor = Object.getOwnPropertyDescriptor(globalThis, 'navigator');

before(async () => {
    Object.defineProperty(globalThis, 'localStorage', {
        configurable: true,
        value: { getItem: () => null },
    });
    /** @type {any} */
    const loaded = await loadConfig('.');
    if (!loaded || loaded.error) {
        throw new Error('could not load vite/svelte config');
    }
    server = await createServer({
        configFile: false,
        server: { middlewareMode: true, watch: null },
        resolve: {
            alias: Object.fromEntries(
                Object.entries(loaded.config.alias).map(
                    /** @param {[string, any]} entry */ entry => [entry[0], resolve(entry[1])],
                ),
            ),
        },
    });
    ({ webauthnReg } = await server.ssrLoadModule('/src/mfa/webauthn/registration.ts'));
});

after(async () => {
    await server?.close();
    /** @type {[string, PropertyDescriptor | undefined][]} */
    const globals = [
        ['localStorage', storageDescriptor],
        ['navigator', navigatorDescriptor],
    ];
    for (const [name, descriptor] of globals) {
        if (descriptor) {
            Object.defineProperty(globalThis, name, descriptor);
        } else {
            Reflect.deleteProperty(globalThis, name);
        }
    }
});

/** @param {string} userId */
const reg = userId => webauthnReg(userId, 'key', 'invalid key', 'timeout', undefined, 'csrf');

test('a rejected registration start returns the server message', async t => {
    /** @param {string} path */
    const fetchImpl = async path => {
        assert.equal(path, '/auth/v1/users/reg-user/webauthn/register/start');
        return Response.json(
            { timestamp: 1, error: 'BadRequest', message: 'This Passkey Type is not allowed' },
            { status: 400 },
        );
    };
    const fetch = t.mock.method(globalThis, 'fetch', fetchImpl);
    const result = await reg('reg-user');
    assert.deepEqual(result, { error: 'This Passkey Type is not allowed' });
    assert.equal(fetch.mock.callCount(), 1);
});

test('a rejected registration finish returns the server message', async t => {
    const buf = new Uint8Array([1, 2, 3]).buffer;
    Object.defineProperty(globalThis, 'navigator', {
        configurable: true,
        value: {
            credentials: {
                create: async () => ({
                    id: 'cred',
                    rawId: buf,
                    response: { attestationObject: buf, clientDataJSON: buf },
                    getClientExtensionResults: () => ({}),
                    type: 'public-key',
                }),
            },
        },
    });
    /** @type {[number, string][]} */
    const cases = [
        [400, 'User Presence only is not allowed - Verification is needed'],
        [406, 'FIDO Authenticator certification level too low'],
    ];
    for (const [status, message] of cases) {
        /** @param {string} path */
        const fetchImpl = async path => {
            if (path.endsWith('/register/start')) {
                return Response.json({
                    publicKey: { challenge: 'AQID', user: { id: 'AQID' }, timeout: 60000 },
                });
            }
            assert.equal(path, '/auth/v1/users/reg-user/webauthn/register/finish');
            return Response.json({ timestamp: 1, error: 'BadRequest', message }, { status });
        };
        t.mock.method(globalThis, 'fetch', fetchImpl);
        assert.deepEqual(await reg('reg-user'), { error: message });
        t.mock.restoreAll();
    }
});
