import {getCsrfToken} from "./helpers.js";

const HEADERS = {
    json: {
        'Content-Type': 'application/json',
        'Accept': 'application/json',
    },
    form: {
        'Content-type': 'application/x-www-form-urlencoded',
        'Accept': 'application/json',
    }
}

function getCsrfHeaders() {
    return {
        ...HEADERS.json,
        'csrf-token': getCsrfToken(),
    }
}

export async function authorize(data) {
    const res = await fetch('/auth/v1/oidc/authorize', {
        method: 'POST',
        headers: getCsrfHeaders(),
        body: JSON.stringify(data),
    });

    if (res.status === 202) {
        window.location.replace(res.headers.get('location'));
    }

    return res;
}

export async function authorizeRefresh(data) {
    const res = await fetch('/auth/v1/oidc/authorize/refresh', {
        method: 'POST',
        headers: getCsrfHeaders(),
        body: JSON.stringify(data),
    });

    if (res.status === 202) {
        window.location.replace(res.headers.get('location'));
    }

    return res;
}


export async function getAppVersion() {
    return await fetch('/auth/v1/version', {
        method: 'GET',
        headers: HEADERS.json,
    });
}

export async function checkAdminAccess() {
    return await fetch('/auth/v1/auth_check_admin', {
        method: 'GET',
        headers: HEADERS.json,
    });
}

export async function getToken(data) {
    return await fetch('/auth/v1/oidc/token', {
        method: 'POST',
        headers: HEADERS.form,
        body: data,
    });
}

export async function getSessionInfo() {
    return await fetch('/auth/v1/oidc/sessioninfo', {
        method: 'GET',
        headers: HEADERS.json,
    });
}

export async function getSessionInfoXsrf(accessToken) {
    return await fetch('/auth/v1/oidc/sessioninfo/xsrf', {
        method: 'GET',
        headers: {
            ...HEADERS.json,
            'Authorization': `Bearer ${accessToken}`,
        },
    });
}

export async function logout(data) {
    return await fetch('/auth/v1/oidc/logout', {
        method: 'POST',
        headers: getCsrfHeaders(),
        body: data,
    });
}

export async function getClientLogo(id) {
    return await fetch(`/auth/v1/clients/${id}/logo`, {
        method: 'GET',
    });
}

export async function getPasswordPolicy() {
    return await fetch('/auth/v1/password_policy', {
        method: 'GET',
        headers: HEADERS.json,
    });
}

export async function getProvidersTemplate() {
    return await fetch('/auth/v1/providers/minimal', {
        method: 'GET',
        headers: HEADERS.json,
    });
}

export async function getPow() {
    return await fetch('/auth/v1/pow', {
        method: 'POST',
        headers: HEADERS.json,
    });
}

export async function resetPassword(uid, data, csrf) {
    return await fetch(`/auth/v1/users/${uid}/reset`, {
        method: 'PUT',
        headers: {
            ...HEADERS.json,
            'pwd-csrf-token': csrf,
        },
        body: JSON.stringify(data),
    });
}

export async function postPasswordResetRequest(data) {
    return await fetch('/auth/v1/users/request_reset', {
        method: 'POST',
        headers: getCsrfHeaders(),
        body: JSON.stringify(data),
    });
}

export async function postProviderLogin(data) {
    return await fetch('/auth/v1/providers/login', {
        method: 'POST',
        headers: getCsrfHeaders(),
        body: JSON.stringify(data),
    });
}

export async function postProviderCallback(data) {
    return await fetch('/auth/v1/providers/callback', {
        method: 'POST',
        headers: getCsrfHeaders(),
        body: JSON.stringify(data),
    });
}

export async function postTestEvent() {
    return await fetch('/auth/v1/events/test', {
        method: 'POST',
        headers: getCsrfHeaders(),
    });
}

export async function registerUser(data) {
    return await fetch('/auth/v1/users/register', {
        method: 'POST',
        headers: HEADERS.json,
        body: JSON.stringify(data),
    });
}

export async function getUser(id) {
    return await fetch(`/auth/v1/users/${id}`, {
        method: 'GET',
        headers: HEADERS.json,
    });
}

export async function getUserPasskeys(id) {
    return await fetch(`/auth/v1/users/${id}/webauthn`, {
        method: 'GET',
        headers: getCsrfHeaders(),
    });
}

export async function getUserWebIdData(id) {
    return await fetch(`/auth/v1/users/${id}/webid/data`, {
        method: 'GET',
        headers: getCsrfHeaders(),
    });
}

export async function putUserWebIdData(id, data) {
    return await fetch(`/auth/v1/users/${id}/webid/data`, {
        method: 'PUT',
        headers: getCsrfHeaders(),
        body: JSON.stringify(data),
    });
}

export async function putUserSelf(id, data) {
    return await fetch(`/auth/v1/users/${id}/self`, {
        method: 'PUT',
        headers: getCsrfHeaders(),
        body: JSON.stringify(data),
    });
}

export async function postUserSelfConvertPasskey(id) {
    return await fetch(`/auth/v1/users/${id}/self/convert_passkey`, {
        method: 'POST',
        headers: getCsrfHeaders(),
    });
}

export async function postUpdateUserLanguage() {
    return await fetch('/auth/v1/update_language', {
        method: 'POST',
        headers: getCsrfHeaders(),
    });
}

export async function webauthnRegStart(id, data) {
    return await fetch(`/auth/v1/users/${id}/webauthn/register/start`, {
        method: 'POST',
        headers: getCsrfHeaders(),
        body: JSON.stringify(data),
    });
}

export async function webauthnRegStartAccReset(id, data, csrf) {
    return await fetch(`/auth/v1/users/${id}/webauthn/register/start`, {
        method: 'POST',
        headers: {
            ...HEADERS.json,
            'pwd-csrf-token': csrf,
        },
        body: JSON.stringify(data),
    });
}

export async function webauthnRegFinish(id, data) {
    return await fetch(`/auth/v1/users/${id}/webauthn/register/finish`, {
        method: 'POST',
        headers: getCsrfHeaders(),
        body: JSON.stringify(data),
    });
}

export async function webauthnRegFinishAccReset(id, data, csrf) {
    return await fetch(`/auth/v1/users/${id}/webauthn/register/finish`, {
        method: 'POST',
        headers: {
            ...HEADERS.json,
            'pwd-csrf-token': csrf,
        },
        body: JSON.stringify(data),
    });
}

export async function webauthnAuthStart(id, data) {
    return await fetch(`/auth/v1/users/${id}/webauthn/auth/start`, {
        method: 'POST',
        headers: getCsrfHeaders(),
        body: JSON.stringify(data),
    });
}

export async function webauthnAuthFinish(id, data) {
    return await fetch(`/auth/v1/users/${id}/webauthn/auth/finish`, {
        method: 'POST',
        headers: getCsrfHeaders(),
        body: JSON.stringify(data),
    });
}

export async function webauthnDelete(id, name) {
    return await fetch(`/auth/v1/users/${id}/webauthn/delete/${name}`, {
        method: 'DELETE',
        headers: getCsrfHeaders(),
    });
}
