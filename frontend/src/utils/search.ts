import { fetchGet, type IResponse } from '#api/fetch.ts';

export type SearchParamsType = 'user' | 'session';
export type SearchParamsIdxUser = 'id' | 'email' | 'name' | 'username';
export type SearchParamsIdxSession = 'userid' | 'sessionid' | 'ip';

export interface SearchParams {
    /// Data type
    ty: SearchParamsType;
    /// Index
    idx: SearchParamsIdxUser | SearchParamsIdxSession;
    /// The actual search query - validation: `[,:/_&?=~#!$()*+@\p{L}\p{M}\p{N}\p{Zs}'.-]+`
    q: string;
    limit?: number;
}

export async function fetchSearchServer<T>(params: SearchParams): Promise<IResponse<T>> {
    // `q` is raw user input and allows characters that would break a raw query string, like a `+`
    // (which the backend decodes as a space) or a `&`
    let q = encodeURIComponent(params.q);
    let url = `/auth/v1/search?ty=${params.ty}&idx=${params.idx}&q=${q}`;
    if (params.limit !== undefined) {
        url += `&limit=${params.limit}`;
    }
    return await fetchGet<T>(url);
}
