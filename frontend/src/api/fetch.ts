import {type ErrorResponse} from "$api/types/error";
import {getCsrfToken} from "$utils/helpers";

export interface IResponse<T> {
    body: undefined | T,
    text: undefined | string,
    error: undefined | ErrorResponse,
    status: number,
    headers: Headers,
}

function buildHeaders(
    method: 'GET' | 'PATCH' | 'POST' | 'PUT' | 'DELETE',
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
        headers['x-csrf-token'] = getCsrfToken();
    }

    return headers;
}

export async function fetchGet<T>(
    uri: string,
    typ: 'json' | 'form' = 'json',
    redirect: 'handle401' | 'noRedirect' = 'handle401',
): Promise<IResponse<T>> {
    return fetchWithoutBody('GET', uri, typ, redirect);
}

export async function fetchPost<T>(
    uri: string,
    payload?: Object,
    typ: 'json' | 'form' = 'json',
    redirect: 'handle401' | 'noRedirect' = 'handle401',
): Promise<IResponse<T>> {
    if (payload) {
        return fetchWithBody('POST', uri, typ, redirect, payload);
    } else {
        return fetchWithoutBody('POST', uri, typ, redirect);
    }
}

// export async function fetchPostMultipart(
//     uri: string,
//     body?: Object,
//     typ: 'json' | 'form' = 'json',
// ) {
//     console.warn('TODO fetchPostMultipart');
// }

export async function fetchForm(form: HTMLFormElement, body: URLSearchParams) {
    return await fetch(form.action, {
        method: form.method,
        headers: {
            'content-type': 'application/x-www-form-urlencoded',
        },
        body,
    })
}

export async function fetchPatch<T>(
    uri: string,
    payload?: Object,
    typ: 'json' | 'form' = 'json',
    redirect: 'handle401' | 'noRedirect' = 'handle401',
): Promise<IResponse<T>> {
    if (payload) {
        return fetchWithBody('PATCH', uri, typ, redirect, payload);
    } else {
        return fetchWithoutBody('PATCH', uri, typ, redirect);
    }
}

export async function fetchPut<T>(
    uri: string,
    payload?: Object,
    typ: 'json' | 'form' = 'json',
    redirect: 'handle401' | 'noRedirect' = 'handle401',
): Promise<IResponse<T>> {
    if (payload) {
        return fetchWithBody('PUT', uri, typ, redirect, payload);
    } else {
        return fetchWithoutBody('PUT', uri, typ, redirect);
    }
}

export async function fetchDelete<T>(
    uri: string,
    body?: Object,
    typ: 'json' | 'form' = 'json',
    redirect: 'handle401' | 'noRedirect' = 'handle401',
): Promise<IResponse<T>> {
    if (body) {
        return await fetchWithBody('DELETE', uri, typ, redirect, body);
    } else {
        return await fetchWithoutBody('DELETE', uri, typ, redirect);
    }
}

async function fetchWithoutBody<T>(
    method: 'GET' | 'PATCH' | 'POST' | 'PUT' | 'DELETE',
    uri: string,
    typ: 'json' | 'form',
    redirect: 'handle401' | 'noRedirect',
): Promise<IResponse<T>> {
    let res = await fetch(uri, {
        method,
        headers: buildHeaders(method, typ),
        redirect: 'manual',
    });
    return await handleResponse(res, redirect);
}

async function fetchWithBody<T>(
    method: 'GET' | 'PATCH' | 'POST' | 'PUT' | 'DELETE',
    uri: string,
    typ: 'json' | 'form',
    redirect: 'handle401' | 'noRedirect',
    payload: Object | FormData,
): Promise<IResponse<T>> {
    let body;
    if (typ === 'json') {
        body = JSON.stringify(payload);
    } else if (typ === 'form') {
        body = formDataFromObj(payload);
    }

    let res = await fetch(uri, {
        method,
        headers: buildHeaders(method, typ),
        redirect: 'manual',
        body,
    });
    return handleResponse(res, redirect);
}

export function formDataFromObj(obj: Object) {
    let params = new URLSearchParams();
    for (let key of Object.keys(obj)) {
        // @ts-ignore
        let v = obj[key];
        if (typeof v !== 'undefined') {
            if (typeof v === 'object') {
                params.append(key, JSON.stringify(v));
            } else {
                // @ts-ignore
                params.append(key, v);
            }
        }
    }
    return params;
}

export async function handleResponse<T>(res: Response, redirect: 'handle401' | 'noRedirect',): Promise<IResponse<T>> {
    if (redirect === 'handle401' && res.status === 401) {
        window.location.reload();
    }

    let resp: IResponse<T> = {
        body: undefined,
        text: undefined,
        error: undefined,
        status: res.status,
        headers: res.headers,
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
    } else if (res.status !== 405) {
        resp.error = await res.json();
    }

    return resp;
}

export async function errorFromResponse(res: Response, eventOnError?: boolean): Promise<ErrorResponse> {
    let err = await res.json();
    // if (eventOnError) {
    //     useEvents().push('error', ErrorType[err.error], err.message, 5);
    // }
    return err;
}

export async function uploadFile<T>(method: 'POST' | 'PUT', url: string, file: File, name: string): Promise<IResponse<T>> {
    const formData = new FormData();
    formData.append(name, file);

    const res = await fetch(url, {
        method: method,
        headers: {
            'x-csrf-token': getCsrfToken(),
        },
        body: formData,
    });
    return handleResponse(res, 'handle401');
}