import {sveltekit} from '@sveltejs/kit/vite';
import wasm from "vite-plugin-wasm";
// import fs from 'fs';

const backend = 'http://127.0.0.1:8080';

/** @type {import('vite').UserConfig} */
const config = {
    build: {
        target: 'esnext',
    },
    plugins: [wasm(), sveltekit()],
    server: {
        // If you want to run with dev TLS certificates, for instance when you use a remote host, uncomment the following
        // lines. Do not forget to adjust the `PUB_URL` in `rauthy.cfg` accordingly to allow the `redirect_uri`, for instance:
        // `PUB_URL=your_remote_host_ip:8443`
        // https: {
        //     key: fs.readFileSync(`${__dirname}/../tls/key.pem`),
        //     cert: fs.readFileSync(`${__dirname}/../tls/cert-chain.pem`)
        // },
        proxy: {
            '/.well-known/': backend,
            '/auth/v1/dev': backend,
            '/auth/v1/_app': backend,
            '/auth/v1/book/': backend,
            '/auth/v1/api_keys': backend,
            '/auth/v1/auth_check_admin': backend,
            '/auth/v1/backup': backend,
            '/auth/v1/blacklist': backend,
            '/auth/v1/docs': backend,
            '/auth/v1/events': backend,
            '/auth/v1/fed_cm/': backend,
            '/auth/v1/i18n': backend,
            // '/auth/v1/oidc/authorize': backend,
            '/auth/v1/oidc/authorize/refresh': backend,
            // '/auth/v1/oidc/callback': backend,
            '/auth/v1/oidc/certs': backend,
            '/auth/v1/oidc/device': backend,
            // '/auth/v1/oidc/logout': backend,
            '/auth/v1/oidc/rotate_jwk': backend,
            '/auth/v1/oidc/session': backend,
            '/auth/v1/oidc/sessioninfo': backend,
            '/auth/v1/oidc/token': backend,
            '/auth/v1/clients': backend,
            '/auth/v1/encryption/keys': backend,
            '/auth/v1/encryption/migrate': backend,
            '/auth/v1/login_time': backend,
            '^^(?!\/auth\/v1\/users\/(password_reset|register|%7Bid%7D))\/auth\/v1\/users.*': backend,
            '/auth/v1/pow': backend,
            '/auth/v1/roles': backend,
            '/auth/v1/groups': backend,
            '/auth/v1/password_hash_times': backend,
            '/auth/v1/password_policy': backend,
            '^^(?!\/auth\/v1\/providers\/callback)\/auth\/v1\/providers.*': backend,
            // '/auth/v1/providers': backend,
            '/auth/v1/scopes': backend,
            '/auth/v1/search': backend,
            '/auth/v1/sessions': backend,
            '/auth/v1/template': backend,
            '/auth/v1/theme': backend,
            '/auth/v1/update_language': backend,
            '/auth/v1/version': backend,
            '/docs/v1/': backend,
        },
    },
    worker: {
        format: 'es',
        plugins: () => [wasm()],
    }
};

export default config;
