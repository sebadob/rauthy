export interface ToSRequest {
	is_html: boolean;
	opt_until?: number;
	content: string;
}

export interface ToSUserAcceptRequest {
	tos_ts: number;
	// 65 char code
	accept_code: string;
}

export interface ToSResponse {
	ts: number;
	author: string;
	is_html: boolean;
	opt_until?: number;
	content: string;
}

export interface ToSAwaitLoginResponse {
	tos_await_code: string;
	user_id: string;
}

export interface ToSLatestResponse {
	ts: number;
	is_html: boolean;
	opt_until?: number;
	content: string;
}

export interface ToSUserAcceptResponse {
	user_id: string;
	tos_ts: number;
	accept_ts: number;
	location: string;
}
