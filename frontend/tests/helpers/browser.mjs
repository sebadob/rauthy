// Shared scaffolding for the built-UI contract tests: real Chromium, the prerendered page from
// `templates/html`, the compiled bundle from `static/v1`, and a virtual WebAuthn authenticator.
// Everything the tests assert on is intercepted, so no backend needs to run.
import assert from 'node:assert/strict';
import { readFile } from 'node:fs/promises';
import { fileURLToPath } from 'node:url';
import { chromium } from '@playwright/test';

export const ORIGIN = 'http://localhost';

const BASE_PREFIX_LENGTH = '/auth/v1'.length;

const STATIC_RE =
    /^\/auth\/v1\/_app\/immutable\/(entry|chunks|nodes|assets)\/[\w.-]+\.(js|css|woff2)$/;

const AUTHENTICATOR = {
    protocol: 'ctap2',
    transport: 'internal',
    hasResidentKey: true,
    hasUserVerification: true,
    isUserVerified: true,
    automaticPresenceSimulation: true,
};

// Debug logging for browser traffic would flood the test output, and
// `PLAYWRIGHT_CHROMIUM_EXECUTABLE_PATH` allows using a system binary instead of a downloaded one.
export async function launchChromium() {
    assert.ok(
        !process.env.DEBUG && !process.env.PWDEBUG,
        'Run without browser/protocol debug logging',
    );
    return chromium.launch({
        executablePath: process.env.PLAYWRIGHT_CHROMIUM_EXECUTABLE_PATH,
    });
}

// relPath is relative to `templates/html`, exactly as written by `just build-ui`.
export async function readBuiltHtml(/** @type {string} */ relPath) {
    return readFile(new URL(`../../../templates/html/${relPath}`, import.meta.url), 'utf8');
}

// The prerendered pages only ship the template loop, which just the backend fills. Values are
// handed over as strings, because that is what the UI reads from the DOM.
export function injectTemplates(
    /** @type {string} */ html,
    /** @type {Record<string, string>} */ templates,
) {
    const injected = Object.entries(templates)
        .map(([id, value]) => `<template id="${id}">${value}</template>`)
        .join('');
    return html.replace(/\{%- for tpl in templates -%\}[\s\S]*?\{%- endfor %\}/, injected);
}

// Serves the built UI for a context: the document and the static bundle are the helper's, anything
// else goes to `handle`, which returns the fulfillment it wants for that request. Unclaimed GETs
// become 404, unclaimed other methods 400. Non-GET requests are recorded as `METHOD /path`.
export async function serveUi(
    /** @type {{ context: any, documentPath: string, html: string, requests: string[], handle?: (route: any, request: any, url: any) => any }} */ {
        context,
        documentPath,
        html,
        requests,
        handle,
    },
) {
    await context.route(
        '**/*',
        /** @param {any} route */ async route => {
            const request = route.request();
            const url = new URL(request.url());
            if (url.origin !== ORIGIN) return route.abort();
            if (request.method() === 'GET') {
                if (url.pathname === documentPath) {
                    return route.fulfill({ contentType: 'text/html', body: html });
                }
                if (STATIC_RE.test(url.pathname)) {
                    const rel = url.pathname.slice(BASE_PREFIX_LENGTH);
                    return route.fulfill({
                        path: fileURLToPath(new URL(`../../../static/v1/${rel}`, import.meta.url)),
                    });
                }
            } else {
                requests.push(`${request.method()} ${url.pathname}`);
            }
            const handled = handle ? await handle(route, request, url) : undefined;
            if (handled) {
                return route.fulfill(handled);
            }
            if (request.method() === 'GET') {
                return route.fulfill({ status: 404 });
            }
            return route.fulfill({ status: 400, json: { message: 'Unexpected endpoint' } });
        },
    );
}

// Enables WebAuthn on the page and adds a virtual authenticator, so the browser can answer the
// ceremonies the UI starts. Removing it again is left to the caller via the returned session.
export async function addVirtualAuthenticator(
    /** @type {any} */ context,
    /** @type {any} */ page,
    /** @type {any} */ options = AUTHENTICATOR,
) {
    const cdp = await context.newCDPSession(page);
    await cdp.send('WebAuthn.enable');
    const { authenticatorId } = await cdp.send('WebAuthn.addVirtualAuthenticator', { options });
    return { cdp, authenticatorId };
}

// Creates a resident credential inside the virtual authenticator which a later ceremony can then
// authenticate against, and returns its id.
export async function createPasskeyCredential(
    /** @type {any} */ page,
    /** @type {{ rpName: string, userId: string, residentKey?: string }} */ {
        rpName,
        userId,
        residentKey = 'required',
    },
) {
    return page.evaluate(
        /** @param {{ rpName: string, userId: string, residentKey: string }} p */ async p => {
            const credential = await navigator.credentials.create({
                publicKey: {
                    challenge: crypto.getRandomValues(new Uint8Array(32)),
                    rp: { name: p.rpName, id: 'localhost' },
                    user: {
                        id: crypto.getRandomValues(new Uint8Array(16)),
                        name: p.userId,
                        displayName: p.userId,
                    },
                    pubKeyCredParams: [{ type: 'public-key', alg: -7 }],
                    authenticatorSelection: {
                        residentKey: /** @type {any} */ (p.residentKey),
                        userVerification: 'required',
                    },
                },
            });
            return credential?.id;
        },
        { rpName, userId, residentKey },
    );
}
