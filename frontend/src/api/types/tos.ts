export interface ToSRequest {
	is_html: boolean;
	content: string;
}

export interface ToSResponse {
	ts: number;
	author: string;
	is_html: boolean;
	content: string;
}

export interface ToSLatestResponse {
	ts: number;
	is_html: boolean;
	content: string;
}

export interface ToSUserAcceptResponse {
	user_id: string;
	tos_ts: number;
	accept_ts: number;
	location: string;
}
