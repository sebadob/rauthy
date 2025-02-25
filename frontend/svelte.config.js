import adapter from '@sveltejs/adapter-static';

const isDev = process.env.DEV_MODE === 'true';

/** @type {import('@sveltejs/kit').Config} */
const config = {
    kit: {
        // CAUTION: The order of the arguments here is important - paths must come before adapter
        paths: {
            base: '/auth/v1',
        },
        adapter: adapter({
            fallback: null,
            pages: '../templates/html',
            assets: '../static/v1',
            precompress: true,
            strict: true,
        }),
        alias: {
            // Contains all the API request and response types
            // (as soon as Svelte 5 + TS migration is done)
            $api: 'src/api',
            $css: 'src/css',
            $icons: 'src/icons5',
            // The `lib5` is a temp lib which will exist until all core components
            // have been migrated to Svelte5 + TS. Migration all dependencies will
            // be a lot safer when changing each component to `lib5` first.
            // Later on, the old component can be removed from `lib` and via compiler
            // warnings, we can be 100% sure, that we have not forgotten any migration.
            // After this is done and no errors are thrown, we can do a very easy
            // switch to the original `lib` via global search and replace.
            $lib5: 'src/lib_svelte5',
            $state: 'src/global_state',
            $types: 'src/types',
            $utils: 'src/utils',
            $webauthn: 'src/webauthn',
        },
        csp: isDev ? {} : {
            directives: {
                'default-src': ['none'],
                'connect-src': ['self'],
                'script-src': ['self', 'wasm-unsafe-eval'],
                'style-src': ['self', 'unsafe-inline'],
                'img-src': ['self'],
            },
        },
        env: {
            publicPrefix: 'DEV_',
        }
    },
};

export default config;
