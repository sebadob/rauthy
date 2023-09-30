import { sveltekit } from '@sveltejs/kit/vite';

/** @type {import('vite').UserConfig} */
const config = {
	plugins: [sveltekit()],
	server: {
		proxy: {
			'/auth/v1/_app': 'http://127.0.0.1:8080',
			'/auth/v1/book/': 'http://127.0.0.1:8080',
			'/auth/v1/auth_check_admin': 'http://127.0.0.1:8080',
			'/auth/v1/i18n': 'http://127.0.0.1:8080',
			// '/auth/v1/oidc/authorize': 'http://127.0.0.1:8080',
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
			'/auth/v1/roles': 'http://127.0.0.1:8080',
			'/auth/v1/groups': 'http://127.0.0.1:8080',
			'/auth/v1/password_hash_times': 'http://127.0.0.1:8080',
			'/auth/v1/password_policy': 'http://127.0.0.1:8080',
			'/auth/v1/scopes': 'http://127.0.0.1:8080',
			'/auth/v1/sessions': 'http://127.0.0.1:8080',
			'/auth/v1/update_language': 'http://127.0.0.1:8080',
			'/docs/v1/': 'http://127.0.0.1:8080',
		}
	}
};

export default config;
