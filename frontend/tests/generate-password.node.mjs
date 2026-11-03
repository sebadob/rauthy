// Run with: npm run test:generate-password
// Frontend-only: `generatePassword()` must honor the policy the backend enforces in
// `User::apply_password_rules()` (`src/data/src/entity/users.rs`) and use a CSPRNG.
import assert from 'node:assert/strict';
import { after, before, test } from 'node:test';
import { createServer } from 'vite';

/** @type {any} */
let server;
/** @type {any} */
let generatePassword;

before(async () => {
    server = await createServer({
        configFile: false,
        server: { middlewareMode: true, watch: null },
    });
    ({ generatePassword } = await server.ssrLoadModule('/src/utils/helpers.ts'));
});

after(async () => {
    await server?.close();
});

/** @param {string} pwd */
function counts(pwd) {
    let c = { lower: 0, upper: 0, digit: 0, special: 0 };
    for (const ch of pwd) {
        if (/[a-z]/.test(ch)) c.lower++;
        else if (/[A-Z]/.test(ch)) c.upper++;
        else if (/[0-9]/.test(ch)) c.digit++;
        else c.special++;
    }
    return c;
}

test('uses the whole lowercase alphabet', () => {
    const seen = new Set();
    for (let i = 0; i < 200; i++) {
        for (const ch of generatePassword({ length_min: 8, length_max: 128 })) seen.add(ch);
    }
    for (const ch of 'kz09AZ') assert.ok(seen.has(ch), `never generated '${ch}'`);
});

test('length stays within [length_min, length_max]', () => {
    assert.equal(generatePassword({ length_min: 8, length_max: 16 }).length, 16);
    assert.equal(generatePassword({ length_min: 8, length_max: 8 }).length, 8);
    assert.equal(generatePassword({ length_min: 30, length_max: 128 }).length, 30);
    assert.equal(generatePassword({ length_min: 8, length_max: 128 }).length, 20);
});

test('required counts per class are met, including include_special', () => {
    const policy = {
        length_min: 8,
        length_max: 128,
        include_lower_case: 5,
        include_upper_case: 6,
        include_digits: 7,
        include_special: 8,
    };
    for (let i = 0; i < 50; i++) {
        const c = counts(generatePassword(policy));
        assert.ok(
            c.lower >= 5 && c.upper >= 6 && c.digit >= 7 && c.special >= 8,
            JSON.stringify(c),
        );
    }
});

test('required counts above length_min raise the length', () => {
    const pwd = generatePassword({ length_min: 8, length_max: 128, include_lower_case: 32 });
    assert.ok(pwd.length >= 32);
    assert.ok(counts(pwd).lower >= 32);
});

test('disabled classes (-1) are not required', () => {
    // 1 lower + 1 upper (defaults) + 8 digits fill all 10 slots, so no special char may appear
    const c = counts(
        generatePassword({ length_min: 8, length_max: 10, include_special: -1, include_digits: 8 }),
    );
    assert.deepEqual(c, { lower: 1, upper: 1, digit: 8, special: 0 });
});

test('impossible policy returns immediately instead of looping', () => {
    const pwd = generatePassword({ length_min: 8, length_max: 10, include_lower_case: 21 });
    assert.equal(pwd, '');
});
