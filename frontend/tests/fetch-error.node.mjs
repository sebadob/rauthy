// Run with: npm run test:fetch-error
// Frontend contract tests only: no backend or network access.
// Non-JSON error bodies (reverse-proxy HTML, empty 5xx, plain text) and network failures must
// come back as an `IResponse` with `error` set instead of a rejected promise.
import assert from 'node:assert/strict';
import { resolve } from 'node:path';
import { after, before, test } from 'node:test';
import { createServer } from 'vite';

import { loadConfig } from '@sveltejs/load-config';

/** @type {any} */
let server;
/** @type {any} */
let fetchGet;

before(async () => {
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
    ({ fetchGet } = await server.ssrLoadModule('/src/api/fetch.ts'));
});

after(async () => {
    await server?.close();
});

test('JSON error bodies are passed through unchanged', async t => {
    t.mock.method(globalThis, 'fetch', async () =>
        Response.json({ timestamp: 1, error: 'BadRequest', message: 'nope' }, { status: 400 }),
    );
    const res = await fetchGet('/auth/v1/x');
    assert.equal(res.status, 400);
    assert.equal(res.body, undefined);
    assert.deepEqual(res.error, { timestamp: 1, error: 'BadRequest', message: 'nope' });
});

test('non-JSON error bodies become an ErrorResponse instead of a rejection', async t => {
    /** @type {[number, string, Record<string, string>][]} */
    const cases = [
        [502, '<html>Bad Gateway</html>', { 'content-type': 'text/html' }],
        [500, '', {}],
        [413, 'Payload Too Large', { 'content-type': 'text/plain' }],
    ];
    for (const [status, body, headers] of cases) {
        t.mock.method(
            globalThis,
            'fetch',
            async () => new Response(body, { status, statusText: '', headers }),
        );
        const res = await fetchGet('/auth/v1/x');
        assert.equal(res.status, status, `status ${status}`);
        assert.equal(res.body, undefined);
        assert.equal(typeof res.error?.error, 'string', `error.error for ${status}`);
        assert.ok(res.error?.message, `error.message for ${status}`);
        assert.equal(typeof res.error?.timestamp, 'number');
        t.mock.restoreAll();
    }
});

test('a failed fetch becomes a status 0 ErrorResponse', async t => {
    t.mock.method(globalThis, 'fetch', async () => {
        throw new TypeError('Failed to fetch');
    });
    const res = await fetchGet('/auth/v1/x');
    assert.equal(res.status, 0);
    assert.equal(res.body, undefined);
    assert.equal(typeof res.error?.error, 'string');
    assert.ok(res.error?.message);
});
