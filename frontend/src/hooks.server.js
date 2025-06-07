// @ts-ignore
import {IS_DEV} from "$utils/constants.js";

const langDefault = 'en';
const themeDefault = 'body{--text:208 10 40;--text-high:208 20 20;--bg:228 2 98;--bg-high:228 8 84;--action:34 100 59;--accent:246 60 53;--error:15 100 37;--btn-text:white;--theme-sun:hsla(var(--action), .7);--theme-moon:hsla(var(--accent), .85);--border-radius:5px;}.theme-dark{--text:228 2 70;--text-high:228 8 90;--bg:208 90 4;--bg-high:208 30 19;--action:34 100 59;--accent:246 60 53;--error:15 100 37;--btn-text:hsl(var(--bg));--theme-sun:hsla(var(--action), .7);--theme-moon:hsla(var(--accent), .85);}@media (prefers-color-scheme: dark){body{--text:228 2 70;--text-high:228 8 90;--bg:208 90 4;--bg-high:208 30 19;--action:34 100 59;--accent:246 60 53;--error:15 100 37;--btn-text:hsl(var(--bg));--theme-sun:hsla(var(--action), .7);--theme-moon:hsla(var(--accent), .85);}.theme-light{--text:208 10 40;--text-high:208 20 20;--bg:228 2 98;--bg-high:228 8 84;--action:34 100 59;--accent:246 60 53;--error:15 100 37;--btn-text:white;--theme-sun:hsla(var(--action), .7);--theme-moon:hsla(var(--accent), .85);}}';

/**
 * This hook only runs during local dev and compiling into static HTML.
 * Proxy values in `vite.config.js` have a higher priority.
 *
 * We can use it to fix "error" popping up because of for instance invalid CSS
 * values inside `app.html`, which will be used in production for SSR.
 *
 * @type {import('@sveltejs/kit').Handle}
 */
export async function handle({event, resolve}) {
    let path = event.url.pathname;

    if (path.startsWith('/auth/v1/theme/%7B%7Bclient_id%7D%7D')) {
        return new Response('{}');
    }

    if (path === '/auth/v1/i18n_email/%7B%7Blang%7D%7D') {
        // TODO insert EN template here if the whole setup works
        // This, for now, only fixes UI compilation.
        return new Response('{}');
    }

    return resolve(event, {
        transformPageChunk: ({html}) => {
            if (IS_DEV) {
                let locale = event.cookies.get('locale');
                return html
                    .replace('%lang%', locale || langDefault)
                    .replace('{{theme_ts}}', new Date().getTime().toString());
            } else {
                return html.replace('%lang%', '{{lang}}');
            }
        }
    });
}
