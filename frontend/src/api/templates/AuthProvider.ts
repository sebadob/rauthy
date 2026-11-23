export type AuthProvidersTemplate = AuthProviderTemplate[];

export interface AuthProviderTemplate {
    id: string;
    name: string;
    updated: number;
    /** No login button on the login and registration pages; `idp_hint` still works. */
    hide_login: boolean;
}
