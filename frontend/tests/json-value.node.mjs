import assert from 'node:assert/strict';
import { after, before, test } from 'node:test';
import { createServer } from 'vite';

/** @type {Awaited<ReturnType<typeof createServer>>} */
let server;
/** @type {typeof import('../src/utils/jsonValue.ts')} */
let jsonValue;

before(async () => {
    server = await createServer({
        configFile: false,
        server: { middlewareMode: true, watch: null },
    });
    jsonValue = /** @type {typeof jsonValue} */ (
        await server.ssrLoadModule('/src/utils/jsonValue.ts')
    );
});

after(async () => server?.close());

test('metadata parses full JSON values and preserves unquoted text', () => {
    const { parseJsonValue } = jsonValue;
    assert.equal(parseJsonValue(''), undefined);
    assert.equal(parseJsonValue('1.25'), 1.25);
    assert.equal(parseJsonValue('12 monkeys'), '12 monkeys');
    assert.equal(parseJsonValue('2024-01-01'), '2024-01-01');
});

test('metadata can be saved again without changing its value or type', () => {
    const { parseJsonValue, stringifyJsonValue } = jsonValue;
    for (const value of [undefined, null, false, 0, '0', 'false', 'null', '', ' a ', ['42', 0]]) {
        assert.deepEqual(parseJsonValue(stringifyJsonValue(value)), value);
    }
});
