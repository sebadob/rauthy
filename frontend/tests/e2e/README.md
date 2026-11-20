# E2E lifecycle tests

Playwright tests against a real Rauthy backend in `dev_mode` that serves the production UI build.
They cover every admin section, the account dashboard and the public flows end to end.

## Running

1. `just mailcrab-start`, then start Rauthy (`just run`) after `just build-ui`. The backend embeds
   the static assets, so rebuild and restart Rauthy after every UI build. Keep the repository
   bootstrap fixtures enabled for the list and search tests.
2. `npx playwright install chromium webkit` (once).
3. `npm run test:e2e` (all), or `npx playwright test -c playwright.e2e.config.js users`.
4. Report: `npx playwright show-report test-results/e2e-report`.

Environment overrides: `E2E_BASE_URL`, `E2E_ADMIN_EMAIL`, `E2E_ADMIN_PASSWORD`, `E2E_MAILCRAB_URL`,
`E2E_BACKEND_SELF_URL` for provider discovery, and `E2E_WEBAUTHN_ORIGIN` to match the backend's
configured WebAuthn origin. The default WebAuthn origin is `http://localhost:5173`.

## Conventions

- One spec per area (`users.spec.mjs`, `clients.spec.mjs`, …), `test.describe` per feature.
- Import `test`/`expect` from `fixtures.mjs`: the page guard fails a test on console errors,
  uncaught exceptions, CSP violations and unexpected HTTP errors. Allow a deliberate error with
  `test.use({ allowErrors: [/regex/] })` and a comment why.
- Prefer accessible roles and names (`getByRole`, `getByLabel`). Use selectors for layout checks
  that depend on document structure.
- Create data with `unique('prefix')` names and delete it again in the test or `afterEach`.
  Never modify the bootstrap admin, the `rauthy` client or global config without restoring it.
- Arrange or verify state through the REST API (`helpers/api.mjs`) only where the UI is not the
  subject of the test. Verify what the UI shows, and that the backend really changed.
- Mail is read from Mailcrab (`helpers/mail.mjs`); passkeys use the CDP virtual authenticator.
- Tag tests that must also run at phone size with `@mobile`.
- Tag tests that exercise Safari-specific behavior with `@webkit`.
