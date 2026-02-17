export type CodeChallengeMethod = 'plain' | 'S256';
export type JwtTokenType = 'Bearer' | 'DPoP' | 'Id' | 'Refresh';

export interface LoginRequest {
    /// Validation: `email`
    email: string;
    /// Validation: max 256
    password?: string;
    pow: string;
    /// Validation: PATTERN_CLIENT_ID
    client_id: string;
    /// Validation: PATTERN_URI
    redirect_uri: string;
    /// Validation: PATTERN_ROLE_SCOPE
    scopes?: string[];
    /// Validation: PATTERN_URI
    state?: string;
    /// Validation: PATTERN_URI
    nonce?: string;
    /// Validation: PATTERN_CODE_CHALLENGE
    code_challenge?: string;
    /// Validation: `plain|S256`
    code_challenge_method?: CodeChallengeMethod;
}

export interface LoginRefreshRequest {
    /// Validation: PATTERN_CLIENT_ID
    client_id: string;
    /// Validation: PATTERN_URI
    redirect_uri: string;
    /// Validation: PATTERN_ROLE_SCOPE
    scopes?: string[];
    /// Validation: PATTERN_URI
    state?: string;
    /// Validation: PATTERN_URI
    nonce?: string;
    /// Validation: PATTERN_CODE_CHALLENGE
    code_challenge?: string;
    code_challenge_method?: CodeChallengeMethod;
}

export interface RequestResetRequest {
    /// Validation: `email`
    email: string;
    /// Validation: PATTERN_URI
    redirect_uri?: string;
    pow: string;
}

// export interface TokenSet {
//     access_token: string,
//     token_type: JwtTokenType,
//     id_token?: string,
//     expires_in: number,
//     refresh_token?: string,
// }

export interface WebauthnLoginResponse {
    code: string;
    user_id: string;
    exp: number;
}
