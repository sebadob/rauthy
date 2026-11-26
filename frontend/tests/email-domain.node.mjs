// Run with: npm run test:email-domain
// The registration form must apply `[user_registration].domain_restriction` the way the backend does:
// an exact, case-insensitive match on the part after the last `@`. A suffix match accepted
// `me@sub.my-domain.com` and `me@my-domain.com.evil.tld`, both domains of somebody else.
import assert from 'node:assert/strict';
import { after, before, test } from 'node:test';
import { createServer } from 'vite';

/** @type {any} */
let server;
/** @type {typeof import('../src/utils/helpers.ts')} */
let helpers;

before(async () => {
    server = await createServer({
        configFile: false,
        server: { middlewareMode: true, watch: null },
    });
    helpers = await server.ssrLoadModule('/src/utils/helpers.ts');
});

after(async () => await server?.close());

test('the restricted domain is matched exactly and case-insensitive', () => {
    const { emailDomainMatches } = helpers;
    assert.equal(emailDomainMatches('me@my-domain.com', 'my-domain.com'), true);
    assert.equal(emailDomainMatches('me@MY-DOMAIN.com', 'my-domain.com'), true);
    assert.equal(emailDomainMatches('me@my-domain.com', 'MY-DOMAIN.com'), true);
});

test('a suffix of the restricted domain is not the restricted domain', () => {
    const { emailDomainMatches } = helpers;
    for (const email of [
        'me@sub.my-domain.com',
        'me@my-domain.com.evil.tld',
        'me@evil-my-domain.com',
        'me@sub.my-domain.com.evil.tld',
        'me',
    ]) {
        assert.equal(emailDomainMatches(email, 'my-domain.com'), false, email);
    }
});
