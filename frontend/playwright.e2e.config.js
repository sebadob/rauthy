// End-to-end lifecycle tests against a real Rauthy backend in `dev_mode`.
//
// The backend must serve the production UI build (`just build-ui`, then restart Rauthy) on
// `E2E_BASE_URL` (default `http://localhost:8080`), with the bootstrap data from `config.toml`
// and a Mailcrab container for outgoing mail (`just mailcrab-start`). Tests create their own data
// with unique names, so they can run against a database that was not reset, but a fresh one
// (`just test-backend` state) gives the most reliable results.
import { defineConfig, devices } from '@playwright/test';

const baseURL = process.env.E2E_BASE_URL || 'http://localhost:8080';

export default defineConfig({
    testDir: './tests/e2e',
    testMatch: /.*\.spec\.mjs$/,
    outputDir: './test-results/e2e',
    fullyParallel: false,
    workers: 1,
    retries: 0,
    timeout: 60_000,
    expect: { timeout: 10_000 },
    reporter: [
        ['list'],
        ['html', { outputFolder: './test-results/e2e-report', open: 'never' }],
        ['json', { outputFile: './test-results/e2e-results.json' }],
    ],
    use: {
        baseURL,
        locale: 'en-US',
        timezoneId: 'Europe/Berlin',
        trace: 'retain-on-failure',
        screenshot: 'only-on-failure',
        serviceWorkers: 'block',
    },
    projects: [
        { name: 'setup', testMatch: /global\.setup\.mjs$/ },
        {
            name: 'desktop',
            dependencies: ['setup'],
            grepInvert: /@mobile-only/,
            use: {
                ...devices['Desktop Chrome'],
                viewport: { width: 1440, height: 900 },
                storageState: './test-results/.auth/admin.json',
            },
        },
        {
            name: 'mobile',
            dependencies: ['setup'],
            grep: /@mobile/,
            use: {
                ...devices['Pixel 7'],
                storageState: './test-results/.auth/admin.json',
            },
        },
    ],
});
