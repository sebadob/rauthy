import adapter from '@sveltejs/adapter-static';

const isDev = process.env.DEV_MODE === 'true';

/** @type {import('@sveltejs/kit').Config} */
const config = {
    compilerOptions: {
        runes: true,
    },
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
            $api: 'src/api',
            $css: 'src/css',
            $icons: 'src/icons',
            $lib5: 'src/lib',
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
        },
    },
};

export default config;
