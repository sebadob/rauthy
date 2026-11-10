// Run with: npm run test:patterns
// Browsers compile an input's `pattern` as `^(?:<pattern>)$` with the `v` flag. If that throws,
// the attribute is ignored and the field accepts anything, with only a console warning.
import assert from 'node:assert/strict';
import { after, before, test } from 'node:test';
import { createServer } from 'vite';

/** @type {any} */
let server;
/** @type {Record<string, unknown>} */
let patterns;

before(async () => {
    server = await createServer({
        configFile: false,
        server: { middlewareMode: true, watch: null },
    });
    patterns = await server.ssrLoadModule('/src/utils/patterns.ts');
});

after(async () => {
    await server?.close();
});

/** @param {string} pattern */
const asBrowser = pattern => new RegExp(`^(?:${pattern})$`, 'v');

test('every pattern compiles the way the browser does', () => {
    for (const [name, pattern] of Object.entries(patterns)) {
        assert.equal(typeof pattern, 'string', name);
        assert.doesNotThrow(() => asBrowser(/** @type {string} */ (pattern)), name);
    }
});

test('client contacts still accept the same values', () => {
    const contact = asBrowser(/** @type {string} */ (patterns.PATTERN_CONTACT));
    for (const ok of ['admin@example.com', 'mailto:admin@example.com', 'https://x.io/a-b', '+49']) {
        assert.ok(contact.test(ok), ok);
    }
    for (const bad of ['two words', 'a,b', 'x'.repeat(49)]) {
        assert.ok(!contact.test(bad), bad);
    }
});
