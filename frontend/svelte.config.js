import adapter from '@sveltejs/adapter-static';

const isDev = process.env.DEV_MODE === 'true';

/** @type {import('@sveltejs/kit').Config} */
const config = {
    kit: {
        // CAUTION: The order of the arguments here is important - adapter must be the last element
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
        csp: isDev ? {} : {
            directives: {
                'default-src': ['none'],
                'connect-src': ['self'],
                'script-src': ['self', 'wasm-unsafe-eval'],
                'style-src': ['self', 'unsafe-inline'],
                'img-src': ['self'],
            },
        },
    },
};

export default config;
