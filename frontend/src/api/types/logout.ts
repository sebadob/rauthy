export interface LogoutParams {
	post_logout_redirect_uri?: string | null;
	id_token_hint?: string | null;
	state?: string | null;
}
