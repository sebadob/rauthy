// Run with: npm run test:redirect-uri
// Mirrors the backend `test_validate_redirect_uri_shape()` in `src/data/src/entity/clients.rs`.
import assert from 'node:assert/strict';
import { after, before, test } from 'node:test';
import { createServer } from 'vite';

let server;
let validateRedirectUriShape;
let RESERVED_REDIRECT_QUERY_KEYS;

before(async () => {
    server = await createServer({
        configFile: false,
        server: { middlewareMode: true, watch: null },
    });
    ({ validateRedirectUriShape, RESERVED_REDIRECT_QUERY_KEYS } = await server.ssrLoadModule(
        '/src/utils/redirectUri.ts',
    ));
});

after(async () => {
    await server?.close();
});

test('valid redirect URIs are accepted', () => {
    for (const uri of [
        'https://app.example.com/cb',
        'https://app.example.com/cb?foo=bar&x=y',
        'https://app.example.com/*',
        'http://localhost:*/cb?foo=bar',
        // keys only, not values or substrings
        'https://app.example.com/cb?foo=iss&issuer=x&code_x=1',
        'https://app.example.com/cb?x=1;issuer=y',
        'https://app.example.com/cb?issx=1',
    ]) {
        assert.equal(validateRedirectUriShape(uri), undefined, uri);
    }
});

test('reserved query keys are rejected', () => {
    assert.deepEqual(RESERVED_REDIRECT_QUERY_KEYS, [
        'code',
        'state',
        'error',
        'error_description',
        'error_uri',
        'iss',
    ]);
    for (const key of RESERVED_REDIRECT_QUERY_KEYS) {
        for (const uri of [
            `https://app.example.com/cb?${key}=x`,
            `https://app.example.com/cb?foo=bar&${key}=x`,
            `https://app.example.com/*?${key}=x`,
            `http://localhost:*/cb?${key}=x`,
        ]) {
            assert.deepEqual(validateRedirectUriShape(uri), { kind: 'reservedKey', key }, uri);
        }
    }

    for (const uri of [
        'https://app.example.com/cb?%69ss=x',
        'https://app.example.com/cb?x=1;iss=https://evil.example',
        'https://app.example.com/cb?x=1;%69ss=x',
        'https://app.example.com/*?x=1;state=x',
        'http://localhost:*/cb?%69ss=x',
        'http://localhost:*/cb?x=1;iss=x',
        'https://app.example.com/cb?+iss=x',
        'https://app.example.com/cb?%20iss=x',
    ]) {
        assert.equal(validateRedirectUriShape(uri)?.kind, 'reservedKey', uri);
    }
});

test('keys folded into a reserved key by common parsers are rejected', () => {
    for (const [uri, key] of [
        ['https://app.example.com/cb?iss', 'iss'],
        ['https://app.example.com/cb?x=1&code', 'code'],
        ['https://app.example.com/cb?x=1&code=', 'code'],
        ['https://app.example.com/cb?ISS=x', 'iss'],
        ['https://app.example.com/cb?State=x', 'state'],
        ['https://app.example.com/cb?code%5B%5D=x', 'code'],
        ['https://app.example.com/cb?iss%5B0%5D=x', 'iss'],
        ['https://app.example.com/cb?state%5Bx%5D=y', 'state'],
        ['https://app.example.com/*?code%5B%5D=x', 'code'],
        ['https://app.example.com/cb?error.description=x', 'error_description'],
        ['https://app.example.com/cb?error+description=x', 'error_description'],
        ['https://app.example.com/cb?error%20description=x', 'error_description'],
        ['https://app.example.com/cb?error.description%5B%5D=x', 'error_description'],
        ['https://app.example.com/cb?iss.x=y', 'iss'],
        ['https://app.example.com/cb?error_uri=https://evil.example', 'error_uri'],
        ['https://app.example.com/cb?error.uri=x', 'error_uri'],
        ['https://app.example.com/cb?%20iss%20=x', 'iss'],
        // Unicode whitespace is trimmed as well
        ['https://app.example.com/cb?%E2%80%83iss=x', 'iss'],
    ]) {
        assert.deepEqual(validateRedirectUriShape(uri), { kind: 'reservedKey', key }, uri);
    }
});

test('control characters in a decoded key are rejected', () => {
    for (const uri of [
        'https://app.example.com/cb?%09iss=x',
        'https://app.example.com/cb?iss%00=x',
        'https://app.example.com/cb?x%0Ay=1',
        'https://app.example.com/cb?%7Fcode=x',
    ]) {
        assert.deepEqual(validateRedirectUriShape(uri), { kind: 'controlChar' }, uri);
    }
});

test('keys that only look similar are accepted', () => {
    for (const uri of [
        'https://app.example.com/cb?issuer=x',
        'https://app.example.com/cb?codes=x',
        'https://app.example.com/cb?state_x=1',
        'https://app.example.com/cb?state%20x=1',
        'https://app.example.com/cb?errors%5B%5D=1',
        'https://app.example.com/cb?error_uris=1',
        'https://app.example.com/cb?x%5Biss%5D=1',
        'https://app.example.com/cb?x.iss=1',
        'https://app.example.com/cb?x=code%5B%5D',
        'https://app.example.com/cb?',
        'https://app.example.com/cb?&&;',
        // a BOM is not whitespace, and invalid UTF-8 is replaced
        'https://app.example.com/cb?%EF%BB%BFiss=x',
        'https://app.example.com/cb?%A0iss=x',
        'https://app.example.com/cb?%69ss%=x',
    ]) {
        assert.equal(validateRedirectUriShape(uri), undefined, uri);
    }
});

test('fragments are rejected', () => {
    for (const uri of [
        'https://app.example.com/#/callback',
        'https://app.example.com/cb#',
        'https://app.example.com/cb?foo=bar#/route',
        'https://app.example.com/cb#/callback?iss=https%3A%2F%2Fattacker.example%2F',
        'https://app.example.com/#/*',
        'http://localhost:*/cb#x',
    ]) {
        assert.deepEqual(validateRedirectUriShape(uri), { kind: 'fragment' }, uri);
    }
});

test('commas are rejected', () => {
    for (const uri of [
        'https://app.example.com/cb?x=,https://evil.example/cb',
        'https://app.example.com/cb?a=,iss=x',
        'https://app.example.com/cb,https://app.example.com/cb?iss=x',
        'https://app.example.com/*,https://evil.example/*',
        'http://localhost:*/cb?x=a,b',
    ]) {
        assert.deepEqual(validateRedirectUriShape(uri), { kind: 'comma' }, uri);
    }
});
