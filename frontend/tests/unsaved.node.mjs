// Run with: npm run test:unsaved
// The `<UnsavedGuard>` decides "dirty" from a list of watched fields. A form that edits a list
// must not look clean after the edit, so arrays and objects are compared by content.
import assert from 'node:assert/strict';
import { after, before, test } from 'node:test';
import { createServer } from 'vite';

/** @type {any} */
let server;
/** @type {{ hasUnsavedChanges: (fields: { value: unknown, original: unknown }[]) => boolean }} */
let unsaved;

before(async () => {
    server = await createServer({
        configFile: false,
        server: { middlewareMode: true, watch: null },
    });
    unsaved = await server.ssrLoadModule('/src/utils/unsaved.ts');
});

after(() => server.close());

test('an empty watch list is never dirty', () => {
    assert.equal(unsaved.hasUnsavedChanges([]), false);
});

test('scalars are compared by value', () => {
    assert.equal(unsaved.hasUnsavedChanges([{ value: 'a', original: 'a' }]), false);
    assert.equal(unsaved.hasUnsavedChanges([{ value: 'a', original: 'b' }]), true);
    assert.equal(unsaved.hasUnsavedChanges([{ value: 1, original: 1 }]), false);
    assert.equal(unsaved.hasUnsavedChanges([{ value: true, original: false }]), true);
});

test('an edited array is dirty even though it is a list', () => {
    assert.equal(
        unsaved.hasUnsavedChanges([{ value: ['a', 'b'], original: ['a', 'b'] }]),
        false,
        'same content must stay clean',
    );
    assert.equal(
        unsaved.hasUnsavedChanges([{ value: ['a', 'c'], original: ['a', 'b'] }]),
        true,
        'an edited list must be dirty',
    );
    assert.equal(
        unsaved.hasUnsavedChanges([{ value: ['b', 'a'], original: ['a', 'b'] }]),
        true,
        'a reordered list must be dirty',
    );
});

test('nested objects are compared by content', () => {
    const original = { authorizationCode: true, password: false };
    assert.equal(
        unsaved.hasUnsavedChanges([{ value: { ...original }, original }]),
        false,
        'a copied object must stay clean',
    );
    assert.equal(
        unsaved.hasUnsavedChanges([{ value: { ...original, password: true }, original }]),
        true,
        'a flipped flag inside an object must be dirty',
    );
});

test('one dirty field out of many is enough', () => {
    const fields = [
        { value: 'name', original: 'name' },
        { value: ['a'], original: ['a'] },
        { value: false, original: true },
    ];
    assert.equal(unsaved.hasUnsavedChanges(fields), true);
});

test('absent and empty are kept apart', () => {
    assert.equal(unsaved.hasUnsavedChanges([{ value: '', original: undefined }]), true);
    assert.equal(unsaved.hasUnsavedChanges([{ value: undefined, original: undefined }]), false);
    assert.equal(unsaved.hasUnsavedChanges([{ value: null, original: undefined }]), true);
});
