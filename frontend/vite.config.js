import {sveltekit} from '@sveltejs/kit/vite';
import wasm from "vite-plugin-wasm";
import topLevelAwait from "vite-plugin-top-level-await";
import fs from 'fs';

/** @type {import('vite').UserConfig} */
const config = {
    plugins: [wasm(), topLevelAwait(), sveltekit()],
    server: {
        // If you want to run with dev TLS certificates, for instance when you use a remote host, uncomment the following
        // lines. Do not forget to adjust the `PUB_URL` in `rauthy.cfg` accordingly to allow the `redirect_uri`, for instance:
        // `PUB_URL=your_remote_host_ip:8443`
        // https: {
        //     key: fs.readFileSync(`${__dirname}/../tls/key.pem`),
        //     cert: fs.readFileSync(`${__dirname}/../tls/cert-chain.pem`)
        // },
        proxy: {
            '/auth/v1/_app': 'http://127.0.0.1:8080',
            '/auth/v1/book/': 'http://127.0.0.1:8080',
            '/auth/v1/api_keys': 'http://127.0.0.1:8080',
            '/auth/v1/auth_check_admin': 'http://127.0.0.1:8080',
            '/auth/v1/blacklist': 'http://127.0.0.1:8080',
            '/auth/v1/events': 'http://127.0.0.1:8080',
            '/auth/v1/i18n': 'http://127.0.0.1:8080',
            '/auth/v1/oidc/authorize': 'http://127.0.0.1:8080',
            '/auth/v1/oidc/callback': 'http://127.0.0.1:8080',
            '/auth/v1/oidc/certs': 'http://127.0.0.1:8080',
            '/auth/v1/oidc/logout': 'http://127.0.0.1:8080',
            '/auth/v1/oidc/rotateJwk': 'http://127.0.0.1:8080',
            '/auth/v1/oidc/sessioninfo': 'http://127.0.0.1:8080',
            '/auth/v1/oidc/token': 'http://127.0.0.1:8080',
            '/auth/v1/clients': 'http://127.0.0.1:8080',
            '/auth/v1/encryption/keys': 'http://127.0.0.1:8080',
            '/auth/v1/encryption/migrate': 'http://127.0.0.1:8080',
            '/auth/v1/login_time': 'http://127.0.0.1:8080',
            '/auth/v1/users': 'http://127.0.0.1:8080',
            '/auth/v1/pow': 'http://127.0.0.1:8080',
            '/auth/v1/roles': 'http://127.0.0.1:8080',
            '/auth/v1/groups': 'http://127.0.0.1:8080',
            '/auth/v1/password_hash_times': 'http://127.0.0.1:8080',
            '/auth/v1/password_policy': 'http://127.0.0.1:8080',
            '/auth/v1/scopes': 'http://127.0.0.1:8080',
            '/auth/v1/sessions': 'http://127.0.0.1:8080',
            '/auth/v1/update_language': 'http://127.0.0.1:8080',
            '/auth/v1/version': 'http://127.0.0.1:8080',
            '/auth/webid/': 'http://127.0.0.1:8080',
            '/docs/v1/': 'http://127.0.0.1:8080',
        }
    }
};

export default config;
