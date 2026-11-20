// Mailcrab (https://github.com/tweedegolf/mailcrab) catches every mail of the dev backend.
import { expect } from '@playwright/test';
import { MAILCRAB_URL } from './env.mjs';

/**
 * Waits for the newest mail to `to` (optionally matching `subject`) that arrived after `since`
 * (unix seconds) and returns it with its HTML and text body.
 * @param {import('@playwright/test').APIRequestContext} request
 * @param {{ to: string, subject?: RegExp, since: number }} opts
 */
export async function waitForMail(request, { to, subject, since }) {
    /** @type {any} */
    let found;
    await expect
        .poll(
            async () => {
                const list = await (await request.get(`${MAILCRAB_URL}/api/messages`)).json();
                found = list
                    .filter(
                        (/** @type {any} */ m) =>
                            m.time >= since &&
                            m.to.some((/** @type {any} */ t) => t.email === to) &&
                            (!subject || subject.test(m.subject)),
                    )
                    .sort((/** @type {any} */ a, /** @type {any} */ b) => b.time - a.time)[0];
                return !!found;
            },
            { timeout: 20_000, message: `mail to ${to}` },
        )
        .toBe(true);
    return (await request.get(`${MAILCRAB_URL}/api/message/${found.id}`)).json();
}

/** Extracts the first link pointing to the Rauthy instance from a mail body. */
export function firstRauthyLink(/** @type {any} */ mail) {
    const body = `${mail.html || ''} ${mail.text || ''}`;
    const m = body.match(/https?:\/\/[^\s"'<>]+\/auth\/v1\/[^\s"'<>]+/);
    if (!m) throw new Error('no Rauthy link in mail');
    return m[0].replaceAll('&amp;', '&');
}
