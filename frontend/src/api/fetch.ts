import {CSRF_TOKEN} from "../utils/constants";
import type {ErrorResponse} from "$api/response/common/errorResponse.ts";

export interface IResponse<T> {
    body: undefined | T,
    text: undefined | string,
    error: undefined | ErrorResponse,
}

function buildHeaders(
    method: 'GET' | 'POST' | 'PUT' | 'DELETE',
    payload: 'json' | 'form',
): HeadersInit {
    let headers: any;
    if (payload === 'json') {
        headers = {
            'Content-Type': 'application/json',
            'Accept': 'application/json',
        };
    } else {
        headers = {
            'Content-type': 'application/x-www-form-urlencoded',
            'Accept': 'application/json',
        };
    }

    if (method !== 'GET') {
        headers['csrf-token'] = localStorage.getItem(CSRF_TOKEN);
    }

    return headers;
}

export async function fetchGet<T>(uri: string, typ: 'json' | 'form' = 'json'): Promise<IResponse<T>> {
    return fetchWithoutBody('GET', uri, typ);
}

export async function fetchPost<T>(
    uri: string,
    body?: Object,
    typ: 'json' | 'form' = 'json',
): Promise<IResponse<T>> {
    if (body) {
        return fetchWithBody('POST', uri, typ, body);
    } else {
        return fetchWithoutBody('POST', uri, typ);
    }
}

// export async function fetchPostMultipart(
//     uri: string,
//     body?: Object,
//     typ: 'json' | 'form' = 'json',
// ) {
//     console.warn('TODO fetchPostMultipart');
// }

export async function fetchPut<T>(
    uri: string,
    body?: Object,
    typ: 'json' | 'form' = 'json',
): Promise<IResponse<T>> {
    if (body) {
        return fetchWithBody('PUT', uri, typ, body);
    } else {
        return fetchWithoutBody('PUT', uri, typ);
    }
}

export async function fetchDelete<T>(
    uri: string,
    body?: Object,
    typ: 'json' | 'form' = 'json',
): Promise<IResponse<T>> {
    if (body) {
        return await fetchWithBody('DELETE', uri, typ, body);
    } else {
        return await fetchWithoutBody('DELETE', uri, typ);
    }
}

async function fetchWithoutBody<T>(
    method: 'GET' | 'POST' | 'PUT' | 'DELETE',
    uri: string,
    typ: 'json' | 'form',
): Promise<IResponse<T>> {
    let res = await fetch(uri, {
        method,
        headers: buildHeaders(method, typ),
        redirect: 'manual',
    });
    return await handleResponse(res, true);
}

async function fetchWithBody<T>(
    method: 'GET' | 'POST' | 'PUT' | 'DELETE',
    uri: string,
    typ: 'json' | 'form',
    body: any,
): Promise<IResponse<T>> {
    let res = await fetch(uri, {
        method,
        headers: buildHeaders(method, typ),
        redirect: 'manual',
        body,
    });
    return handleResponse(res, true);
}

export async function handleResponse<T>(res: Response, redirect401: boolean): Promise<IResponse<T>> {
    // TODO we could either do a redirect to a given loc header
    // or post any error to a global events / toast component for better UX in such a case
    // -> will come in a later PR

    // if (res.status === 401) {
    //     if (redirect401) {
    //         let loc = res.headers.get('location');
    //         if (loc) {
    //             window.location.href = loc;
    //         }
    //     }
    // }

    let resp: IResponse<T> = {
        body: undefined,
        text: undefined,
        error: undefined,
    };

    if (res.ok) {
        let contentType = res.headers.get('content-type');
        if (contentType === 'application/json') {
            resp.body = await res.json();
            // } else if (contentType === 'application/x-www-form-urlencoded') {
            //     resp.body = await res.formData();
        } else {
            resp.text = await res.text();
        }
    } else {
        resp.error = await res.json();
    }

    return resp;
}
