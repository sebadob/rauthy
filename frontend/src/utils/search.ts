import { fetchGet, type IResponse } from '$api/fetch';

export type SearchParamsType = 'user' | 'session';
export type SearchParamsIdxUser = 'id' | 'email';
export type SearchParamsIdxSession = 'userid' | 'sessionid' | 'ip';

export interface SearchParams {
	/// Data type
	ty: SearchParamsType;
	/// Index
	idx: SearchParamsIdxUser | SearchParamsIdxSession;
	/// The actual search query - validation: `[a-zA-Z0-9,.:/_\-&?=~#!$'()*+%@]+`
	q: string;
	limit?: number;
}

export async function fetchSearchServer<T>(params: SearchParams): Promise<IResponse<T>> {
	let url = `/auth/v1/search?ty=${params.ty}&idx=${params.idx}&q=${params.q}`;
	if (params.limit) {
		url += `$limit=${params.limit}`;
	}
	return await fetchGet<T>(url);
}
