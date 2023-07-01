export async function checkAuth() {
	const res = await fetch('/api/check_auth', {
		method: 'GET',
		headers: getTokenHeaders(),
	});

	if (res.status >= 400) {
		window.location.href = '/login';
	}
}
