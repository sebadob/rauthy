import type {JwkKeyPairAlg} from "./oidc";

export interface NewClientRequest {
    /// Validation: PATTERN_LOWERCASE
    id: string,
    /// Validation: PATTERN_CLIENT_NAME
    name?: string,
    confidential: boolean,
    /// Validation: PATTERN_URI
    redirect_uris: string[],
    /// Validation: PATTERN_URI
    post_logout_redirect_uris?: string[],
}

export interface ClientResponse {
    id: string,
    name?: string,
    enabled: boolean,
    confidential: boolean,
    redirect_uris: string[],
    post_logout_redirect_uris?: string[],
    allowed_origins?: string[],
    flows_enabled: string[],
    access_token_alg: JwkKeyPairAlg,
    id_token_alg: JwkKeyPairAlg,
    auth_code_lifetime: number,
    access_token_lifetime: number,
    scopes: string[],
    default_scopes: string[],
    challenges?: string[],
    force_mfa: boolean,
    client_uri?: string,
    contacts?: string[],
}
