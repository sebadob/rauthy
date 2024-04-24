import {getCsrfToken} from "./helpers.js";
import {sleepAwait} from "$lib/utils/helpers.js";

const HEADERS = {
    'Content-Type': 'application/json',
    'Accept': 'application/json',
}

function getHeaders() {
    return {
        // TOOD change to session with lifetime observation
        'csrf-token': getCsrfToken(),
        'Content-Type': 'application/json',
        'Accept': 'application/json',
    }
    // 'Authorization': `Bearer ${getAccessToken()}`
}

async function checkRedirectForbidden(res) {
    if (res.status === 403) {
        let body = await res.json();

        if (body.error === 'MfaRequired') {
            console.error('MFA is required for a rauthy admin account');
            await sleepAwait(3000);
            window.location.reload();
        }
    }

    if (res.status === 401 || res.status === 403) {
        window.location.reload();
    }
    return res;
}

export async function getCerts() {
    return await fetch(`/auth/v1/oidc/certs`, {
        method: 'GET',
        headers: HEADERS,
    });
}

export async function getClients() {
    const res = await fetch(`/auth/v1/clients`, {
        method: 'GET',
        headers: HEADERS,
    });
    return await checkRedirectForbidden(res);
}

export async function postClient(client) {
    const res = await fetch(`/auth/v1/clients`, {
        method: 'POST',
        headers: getHeaders(),
        body: JSON.stringify(client),
    });
    return await checkRedirectForbidden(res);
}

export async function putClient(client) {
    const res = await fetch(`/auth/v1/clients/${client.id}`, {
        method: 'PUT',
        headers: getHeaders(),
        body: JSON.stringify(client),
    });
    return await checkRedirectForbidden(res);
}

export async function deleteClient(id) {
    const res = await fetch(`/auth/v1/clients/${id}`, {
        method: 'DELETE',
        headers: getHeaders(),
    });
    return await checkRedirectForbidden(res);
}

export async function getClientColors(id) {
    const res = await fetch(`/auth/v1/clients/${id}/colors`, {
        method: 'GET',
        headers: HEADERS,
    });
    return await checkRedirectForbidden(res);
}

export async function putClientColors(id, colors) {
    const res = await fetch(`/auth/v1/clients/${id}/colors`, {
        method: 'PUT',
        headers: getHeaders(),
        body: JSON.stringify(colors),
    });
    return await checkRedirectForbidden(res);
}

export async function deleteClientColors(id) {
    const res = await fetch(`/auth/v1/clients/${id}/colors`, {
        method: 'DELETE',
        headers: getHeaders(),
    });
    return await checkRedirectForbidden(res);
}

export async function putClientLogo(id, data) {
    const formData = new FormData();
    formData.append("logo", data);

    const res = await fetch(`/auth/v1/clients/${id}/logo`, {
        method: 'PUT',
        headers: {
            'csrf-token': getCsrfToken(),
        },
        body: formData,
    });
    return await checkRedirectForbidden(res);
}

export async function deleteClientLogo(id) {
    const res = await fetch(`/auth/v1/clients/${id}/logo`, {
        method: 'DELETE',
        headers: getHeaders(),
    });
    return await checkRedirectForbidden(res);
}

export async function getClientSecret(id) {
    const res = await fetch(`/auth/v1/clients/${id}/secret`, {
        method: 'GET',
        headers: HEADERS,
    });
    return await checkRedirectForbidden(res);
}

export async function putClientSecret(id) {
    const res = await fetch(`/auth/v1/clients/${id}/secret`, {
        method: 'PUT',
        headers: getHeaders(),
    });
    return await checkRedirectForbidden(res);
}

export async function getEncKeys() {
    const res = await fetch('/auth/v1/encryption/keys', {
        method: 'GET',
        headers: HEADERS,
    });
    return await checkRedirectForbidden(res);
}

export async function postEncMigrate(data) {
    const res = await fetch('/auth/v1/encryption/migrate', {
        method: 'POST',
        headers: getHeaders(),
        body: JSON.stringify(data),
    });
    return await checkRedirectForbidden(res);
}

export async function postEvents(data) {
    const res = await fetch('/auth/v1/events', {
        method: 'POST',
        headers: getHeaders(),
        body: JSON.stringify(data),
    });
    return await checkRedirectForbidden(res);
}

export async function getGroups() {
    const res = await fetch('/auth/v1/groups', {
        method: 'GET',
        headers: HEADERS,
    });
    return await checkRedirectForbidden(res);
}

export async function postGroup(group) {
    const res = await fetch('/auth/v1/groups', {
        method: 'POST',
        headers: getHeaders(),
        body: JSON.stringify(group),
    });
    return await checkRedirectForbidden(res);
}

export async function putGroup(id, group) {
    const res = await fetch(`/auth/v1/groups/${id}`, {
        method: 'PUT',
        headers: getHeaders(),
        body: JSON.stringify(group),
    });
    return await checkRedirectForbidden(res);
}

export async function deleteGroup(id) {
    const res = await fetch(`/auth/v1/groups/${id}`, {
        method: 'DELETE',
        headers: getHeaders(),
    });
    return await checkRedirectForbidden(res);
}

export async function getLoginTime() {
    const res = await fetch(`/auth/v1/login_time`, {
        method: 'GET',
        headers: HEADERS,
    });
    return await checkRedirectForbidden(res);
}

export async function postPasswordHashTimes(data) {
    const res = await fetch(`/auth/v1/password_hash_times`, {
        method: 'POST',
        headers: getHeaders(),
        body: JSON.stringify(data),
    });
    return await checkRedirectForbidden(res);
}

export async function putPasswordPolicy(data) {
    const res = await fetch('/auth/v1/password_policy', {
        method: 'PUT',
        headers: getHeaders(),
        body: JSON.stringify(data),
    });
    return await checkRedirectForbidden(res);
}

export async function postPasswordResetRequest(data) {
    const res = await fetch('/auth/v1/users/request_reset', {
        method: 'POST',
        headers: getHeaders(),
        body: JSON.stringify(data),
    });
    return await checkRedirectForbidden(res);
}

export async function postProviders() {
    return await fetch('/auth/v1/providers', {
        method: 'POST',
        headers: getHeaders(),
    });
}

export async function postProvider(data) {
    return await fetch('/auth/v1/providers/create', {
        method: 'POST',
        headers: getHeaders(),
        body: JSON.stringify(data),
    });
}

export async function putProvider(id, data) {
    return await fetch(`/auth/v1/providers/${id}`, {
        method: 'PUT',
        headers: getHeaders(),
        body: JSON.stringify(data),
    });
}

export async function deleteProvider(id) {
    return await fetch(`/auth/v1/providers/${id}`, {
        method: 'DELETE',
        headers: getHeaders(),
    });
}

export async function getProviderDeleteSafe(id) {
    return await fetch(`/auth/v1/providers/${id}/delete_safe`, {
        method: 'GET',
        headers: getHeaders(),
    });
}

export async function putProviderLogo(id, data) {
    const formData = new FormData();
    formData.append("logo", data);

    const res = await fetch(`/auth/v1/providers/${id}/img`, {
        method: 'PUT',
        headers: {
            'csrf-token': getCsrfToken(),
        },
        body: formData,
    });
    return await checkRedirectForbidden(res);
}

export async function postProviderLookup(data) {
    return await fetch('/auth/v1/providers/lookup', {
        method: 'POST',
        headers: getHeaders(),
        body: JSON.stringify(data),
    });
}

export async function postRotateJwk() {
    const res = await fetch(`/auth/v1/oidc/rotateJwk`, {
        method: 'POST',
        headers: getHeaders(),
    });
    return await checkRedirectForbidden(res);
}

export async function getSessions() {
    const res = await fetch('/auth/v1/sessions', {
        method: 'GET',
        headers: HEADERS,
    });
    return await checkRedirectForbidden(res);
}

export async function deleteAllSessions() {
    const res = await fetch('/auth/v1/sessions', {
        method: 'DELETE',
        headers: getHeaders(),
    });
    return await checkRedirectForbidden(res);
}

export async function deleteSessionsForUser(uid) {
    const res = await fetch(`/auth/v1/sessions/${uid}`, {
        method: 'DELETE',
        headers: getHeaders(),
    });
    return await checkRedirectForbidden(res);
}

export async function getUsers() {
    const res = await fetch('/auth/v1/users', {
        method: 'GET',
        headers: HEADERS,
    });
    return await checkRedirectForbidden(res);
}

export async function getUsersSsp(pageSize, offset, continuationToken, backwards) {
    let url = `/auth/v1/users?page_size=${pageSize}`;
    if (offset) {
        url = `${url}&offset=${offset}`;
    }
    if (backwards) {
        url = `${url}&backwards=${backwards}`;
    }
    if (continuationToken) {
        url = `${url}&continuation_token=${continuationToken}`;
    }
    const res = await fetch(
        url, {
            method: 'GET',
            headers: HEADERS,
        });
    return await checkRedirectForbidden(res);
}

export async function postUser(data) {
    const res = await fetch('/auth/v1/users', {
        method: 'POST',
        headers: getHeaders(),
        body: JSON.stringify(data),
    });
    return await checkRedirectForbidden(res);
}

export async function putUser(uid, data) {
    const res = await fetch(`/auth/v1/users/${uid}`, {
        method: 'PUT',
        headers: getHeaders(),
        body: JSON.stringify(data),
    });
    return await checkRedirectForbidden(res);
}

export async function deleteUser(uid) {
    const res = await fetch(`/auth/v1/users/${uid}`, {
        method: 'DELETE',
        headers: getHeaders(),
    });
    return await checkRedirectForbidden(res);
}

export async function getAttr() {
    const res = await fetch('/auth/v1/users/attr', {
        method: 'GET',
        headers: HEADERS,
    });
    return await checkRedirectForbidden(res);
}

export async function postAttr(data) {
    const res = await fetch('/auth/v1/users/attr', {
        method: 'POST',
        headers: getHeaders(),
        body: JSON.stringify(data),
    });
    return await checkRedirectForbidden(res);
}

export async function putAttr(name, data) {
    const res = await fetch(`/auth/v1/users/attr/${name}`, {
        method: 'PUT',
        headers: getHeaders(),
        body: JSON.stringify(data),
    });
    return await checkRedirectForbidden(res);
}

export async function deleteAttr(name) {
    const res = await fetch(`/auth/v1/users/attr/${name}`, {
        method: 'DELETE',
        headers: getHeaders(),
    });
    return await checkRedirectForbidden(res);
}

export async function getUserAttr(uid) {
    const res = await fetch(`/auth/v1/users/${uid}/attr`, {
        method: 'GET',
        headers: HEADERS,
    });
    return await checkRedirectForbidden(res);
}

export async function updateUserAttr(uid, data) {
    const res = await fetch(`/auth/v1/users/${uid}/attr`, {
        method: 'PUT',
        headers: getHeaders(),
        body: JSON.stringify(data),
    });
    return await checkRedirectForbidden(res);
}


export async function getApiKeys() {
    const res = await fetch('/auth/v1/api_keys', {
        method: 'GET',
        headers: HEADERS,
    });
    return await checkRedirectForbidden(res);
}

export async function postApiKey(data) {
    const res = await fetch('/auth/v1/api_keys', {
        method: 'POST',
        headers: getHeaders(),
        body: JSON.stringify(data),
    });
    return await checkRedirectForbidden(res);
}

export async function putApiKey(data) {
    const res = await fetch(`/auth/v1/api_keys/${data.name}`, {
        method: 'PUT',
        headers: getHeaders(),
        body: JSON.stringify(data),
    });
    return await checkRedirectForbidden(res);
}

export async function deleteApiKey(name) {
    const res = await fetch(`/auth/v1/api_keys/${name}`, {
        method: 'DELETE',
        headers: getHeaders(),
    });
    return await checkRedirectForbidden(res);
}


export async function putApiKeySecret(name) {
    const res = await fetch(`/auth/v1/api_keys/${name}/secret`, {
        method: 'PUT',
        headers: getHeaders(),
    });
    return await checkRedirectForbidden(res);
}

export async function getBlacklist() {
    const res = await fetch('/auth/v1/blacklist', {
        method: 'GET',
        headers: HEADERS,
    });
    return await checkRedirectForbidden(res);
}

export async function postBlacklist(data) {
    const res = await fetch('/auth/v1/blacklist', {
        method: 'POST',
        headers: getHeaders(),
        body: JSON.stringify(data),
    });
    return await checkRedirectForbidden(res);
}

export async function deleteBlacklistedIp(ip) {
    const res = await fetch(`/auth/v1/blacklist/${ip}`, {
        method: 'DELETE',
        headers: getHeaders(),
    });
    return await checkRedirectForbidden(res);
}

export async function getRoles() {
    const res = await fetch('/auth/v1/roles', {
        method: 'GET',
        headers: HEADERS,
    });
    return await checkRedirectForbidden(res);
}

export async function postRole(role) {
    const res = await fetch('/auth/v1/roles', {
        method: 'POST',
        headers: getHeaders(),
        body: JSON.stringify(role),
    });
    return await checkRedirectForbidden(res);
}

export async function putRole(id, role) {
    const res = await fetch(`/auth/v1/roles/${id}`, {
        method: 'PUT',
        headers: getHeaders(),
        body: JSON.stringify(role),
    });
    return await checkRedirectForbidden(res);
}

export async function deleteRole(id) {
    const res = await fetch(`/auth/v1/roles/${id}`, {
        method: 'DELETE',
        headers: getHeaders(),
    });
    return await checkRedirectForbidden(res);
}

export async function getScopes() {
    const res = await fetch('/auth/v1/scopes', {
        method: 'GET',
        headers: HEADERS,
    });
    return await checkRedirectForbidden(res);
}

export async function postScope(scope) {
    const res = await fetch('/auth/v1/scopes', {
        method: 'POST',
        headers: getHeaders(),
        body: JSON.stringify(scope),
    });
    return await checkRedirectForbidden(res);
}

export async function putScope(id, scope) {
    const res = await fetch(`/auth/v1/scopes/${id}`, {
        method: 'PUT',
        headers: getHeaders(),
        body: JSON.stringify(scope),
    });
    return await checkRedirectForbidden(res);
}

export async function deleteScope(id) {
    const res = await fetch(`/auth/v1/scopes/${id}`, {
        method: 'DELETE',
        headers: getHeaders(),
    });
    return await checkRedirectForbidden(res);
}

export async function getSearch(ty, idx, q) {
    const res = await fetch(`/auth/v1/search?ty=${ty}&idx=${idx}&q=${q}`, {
        method: 'GET',
        headers: HEADERS,
    });
    return await checkRedirectForbidden(res);
}
