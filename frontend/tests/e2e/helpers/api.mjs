// Direct REST access with the logged-in admin session, used to arrange and clean up test data
// without going through the UI under test. The CSRF token is read the same way the UI reads it.

/** @param {import('@playwright/test').Page} page */
export async function csrfToken(page) {
    const stored = await page.evaluate(() => localStorage.getItem('sct') || '');
    // `utils/helpers.ts` stores it as 'o/' + 39 random chars + token + 43 random chars.
    return stored.slice(2 + 39, stored.length - 43);
}

/**
 * @param {import('@playwright/test').Page} page
 * @param {'GET'|'POST'|'PUT'|'PATCH'|'DELETE'} method
 * @param {string} path relative to `/auth/v1/`
 * @param {any} [data]
 */
export async function api(page, method, path, data) {
    if (!page.url().startsWith('http')) await page.goto('/auth/v1/account');
    const res = await page.request.fetch(`/auth/v1/${path.replace(/^\//, '')}`, {
        method,
        headers: { 'x-csrf-token': await csrfToken(page), 'content-type': 'application/json' },
        data: data === undefined ? undefined : JSON.stringify(data),
    });
    const text = await res.text();
    if (!res.ok()) throw new Error(`${method} ${path} -> ${res.status()}: ${text}`);
    try {
        return text ? JSON.parse(text) : null;
    } catch {
        return text;
    }
}
