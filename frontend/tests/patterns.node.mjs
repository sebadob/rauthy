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

test('claim mapping paths and values match the backend rules', () => {
    const path = asBrowser(/** @type {string} */ (patterns.PATTERN_JSON_PATH));
    for (const ok of [
        '$',
        '$.roles',
        '$.roles.*',
        '$.groups[*]',
        "$..['x-groups']",
        '$.foo.bar[0]',
        '$.foo..bar',
    ]) {
        assert.ok(path.test(ok), ok);
    }
    for (const bad of ['', 'x'.repeat(257), '$.groups\t', '$.groups\0']) {
        assert.ok(!path.test(bad), bad);
    }

    const value = asBrowser(/** @type {string} */ (patterns.PATTERN_CLAIM_VALUE));
    for (const ok of ['rauthy_admin', '^tgchat:', '~^admin$', '^^literal', 'a b']) {
        assert.ok(value.test(ok), ok);
    }
    for (const bad of ['', 'x'.repeat(129), 'a\tb']) {
        assert.ok(!value.test(bad), bad);
    }

    // A target lands in the comma-separated role / group CSV of a user, so a `,` is out. The role
    // name rule has no space class and allows a `.`, the group rule has the space and no `.`.
    const roleTarget = asBrowser(/** @type {string} */ (patterns.PATTERN_CLAIM_TARGET_ROLE));
    const groupTarget = asBrowser(/** @type {string} */ (patterns.PATTERN_CLAIM_TARGET_GROUP));
    for (const ok of ['tgchat:admin', 'cm_test_role']) {
        assert.ok(roleTarget.test(ok), ok);
        assert.ok(groupTarget.test(ok), ok);
    }
    assert.ok(roleTarget.test('dev.read'), 'the role rule allows a `.`');
    assert.ok(groupTarget.test('My Group 1'), 'the group rule allows spaces');
    for (const bad of ['', 'a,b', 'x'.repeat(65)]) {
        assert.ok(!roleTarget.test(bad), bad);
        assert.ok(!groupTarget.test(bad), bad);
    }
    assert.ok(!roleTarget.test('My Role'), 'the role rule has no space class');
    assert.ok(!groupTarget.test('dev.read'), 'the group rule has no `.` class');
});
