// Every test runs with a guard that fails it on unexpected browser errors: console errors,
// uncaught exceptions, CSP violations and HTTP errors the UI did not ask for on purpose.
import { test as base, expect } from '@playwright/test';

/**
 * Requests that fail by design and are therefore not a UI problem.
 * Keep this list short and explain every entry.
 * @type {{ status: number, method?: string, path: RegExp, why: string }[]}
 */
const EXPECTED_HTTP_ERRORS = [
    { status: 401, path: /\/oidc\/sessioninfo$/, why: 'session probe before login' },
    { status: 404, path: /\/clients\/[^/]+\/logo$/, why: 'no custom client logo uploaded' },
    { status: 404, path: /\/users\/[^/]+\/picture\/?/, why: 'no user picture uploaded' },
];

/**
 * @type {import('@playwright/test').Fixtures<
 *   { allowErrors: RegExp[] },
 *   {},
 *   import('@playwright/test').PlaywrightTestArgs & import('@playwright/test').PlaywrightTestOptions
 * >}
 */
const fixtures = {
    /** Extra allowed errors for a single test: `test.use({ allowErrors: [/regex/] })`. */
    allowErrors: [[], { option: true }],

    page: async ({ page, allowErrors }, use, testInfo) => {
        /** @type {string[]} */
        const problems = [];
        /** @type {Promise<void>[]} */
        const responseChecks = [];
        const allowed = (/** @type {string} */ s) => allowErrors.some(re => re.test(s));

        page.on('pageerror', err => problems.push(`pageerror: ${err.message}`));
        page.on('console', msg => {
            if (msg.type() !== 'error') return;
            const text = msg.text();
            // The matching HTTP response is judged below; the console line only repeats it.
            if (/^Failed to load resource: the server responded with a status of/.test(text)) {
                return;
            }
            if (!allowed(text)) problems.push(`console: ${text}`);
        });
        page.on('response', res => {
            if (res.status() < 400) return;
            const url = new URL(res.url());
            const method = res.request().method();
            const ok = EXPECTED_HTTP_ERRORS.some(
                e =>
                    e.status === res.status() &&
                    (!e.method || e.method === method) &&
                    e.path.test(url.pathname),
            );
            const line = `http ${res.status()} ${method} ${url.pathname}`;
            if (ok || allowed(line)) return;

            if (
                res.status() === 400 &&
                method === 'POST' &&
                url.pathname.endsWith('/oidc/authorize')
            ) {
                responseChecks.push(
                    res
                        .json()
                        .catch(() => undefined)
                        .then(body => {
                            if (
                                body?.error !== 'PasswordRequired' &&
                                body?.error !== 'InvalidCredentials'
                            ) {
                                problems.push(line);
                            }
                        }),
                );
            } else {
                problems.push(line);
            }
        });

        await use(page);
        await Promise.all(responseChecks);

        if (problems.length) {
            await testInfo.attach('browser-problems', { body: problems.join('\n') });
        }
        expect(problems, 'unexpected browser errors').toEqual([]);
    },
};

export const test = base.extend(fixtures);

export { expect };
