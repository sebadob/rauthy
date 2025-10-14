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
