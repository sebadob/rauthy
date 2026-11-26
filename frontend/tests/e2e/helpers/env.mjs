// Values of the `dev_mode` bootstrap from `config.toml`. They only exist on a local test backend.
export const ADMIN_EMAIL = process.env.E2E_ADMIN_EMAIL || 'admin@localhost';
export const ADMIN_PASSWORD = process.env.E2E_ADMIN_PASSWORD || '123SuperSafe';
export const MAILCRAB_URL = process.env.E2E_MAILCRAB_URL || 'http://localhost:1080';
export const AUTH_STATE = './test-results/.auth/admin.json';

/** A name that is unique per run, so tests never collide with existing or older data. */
export function unique(/** @type {string} */ prefix) {
    return `${prefix}-${Date.now().toString(36)}-${Math.random().toString(36).slice(2, 6)}`;
}
