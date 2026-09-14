// Run with: npm run test:webauthn
// Frontend contract tests only: no backend, accounts, or network access.
import assert from 'node:assert/strict';
import { resolve } from 'node:path';
import { after, before, test } from 'node:test';
import { createServer } from 'vite';
import config from '../svelte.config.js';

let server;
let webauthnAuth;
const storageDescriptor = Object.getOwnPropertyDescriptor(globalThis, 'localStorage');

before(async () => {
    Object.defineProperty(globalThis, 'localStorage', {
        configurable: true,
        value: { getItem: () => null },
    });
    server = await createServer({
        configFile: false,
        server: { middlewareMode: true, watch: null },
        resolve: {
            alias: Object.fromEntries(
                Object.entries(config.kit.alias).map(([key, path]) => [key, resolve(path)]),
            ),
        },
    });
    ({ webauthnAuth } = await server.ssrLoadModule('/src/mfa/webauthn/authentication.ts'));
});

after(async () => {
    await server?.close();
    if (storageDescriptor) {
        Object.defineProperty(globalThis, 'localStorage', storageDescriptor);
    } else {
        delete globalThis.localStorage;
    }
});

test('password reset starts verification for the reset user', async t => {
    const fetch = t.mock.method(globalThis, 'fetch', async (path, options) => {
        assert.equal(path, '/auth/v1/users/reset-user/webauthn/auth/start');
        assert.equal(options.method, 'POST');
        assert.deepEqual(JSON.parse(options.body), { purpose: 'PasswordReset' });
        return Response.json({ message: 'binding rejected' }, { status: 400 });
    });
    t.mock.method(console, 'error', () => {});
    const result = await webauthnAuth('PasswordReset', 'invalid key', 'timeout', 'reset-user');
    assert.equal(result.error, 'binding rejected');
    assert.equal(fetch.mock.callCount(), 1);
});

test('missing reset user fails before any request', async t => {
    const fetch = t.mock.method(globalThis, 'fetch', async () => {
        throw new Error('must not send a reset request without its user');
    });
    for (const userId of [undefined, '']) {
        const result = await webauthnAuth('PasswordReset', 'invalid key', 'timeout', userId);
        assert.ok(result.error);
    }
    assert.equal(fetch.mock.callCount(), 0);
});

test('the reset user is encoded as one path segment', async t => {
    t.mock.method(globalThis, 'fetch', async path => {
        assert.equal(
            path,
            '/auth/v1/users/user%2Fwith%3Freserved%23characters/webauthn/auth/start',
        );
        return Response.json({ message: 'invalid user' }, { status: 400 });
    });
    t.mock.method(console, 'error', () => {});
    await webauthnAuth('PasswordReset', 'invalid key', 'timeout', 'user/with?reserved#characters');
});

test('login and authenticated service purposes keep the generic endpoint', async t => {
    // Generate login context at runtime; never persist it in a fixture or assertion output.
    const login = { Login: crypto.randomUUID() };
    for (const purpose of [login, 'Discover', 'PasswordNew', 'MfaModToken', 'Test']) {
        const fetch = t.mock.method(globalThis, 'fetch', async (path, options) => {
            assert.equal(path, '/auth/v1/users/webauthn_start');
            assert.equal(options.method, 'POST');
            assert.ok(JSON.stringify(JSON.parse(options.body).purpose) === JSON.stringify(purpose));
            return Response.json({ message: 'verification rejected' }, { status: 403 });
        });
        t.mock.method(console, 'error', () => {});
        const result = await webauthnAuth(purpose, 'invalid key', 'timeout', 'ignored-user');
        assert.equal(result.error, 'verification rejected');
        assert.equal(fetch.mock.callCount(), 1);
        t.mock.restoreAll();
    }
});
