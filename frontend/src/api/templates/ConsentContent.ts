export interface ConsentScope {
    name: string;
    /// only for `remote`
    title?: string;
    /// only for `remote`, may contain HTML that must be sanitized
    description?: string;
    warning: boolean;
}

/// The consent the login page asks for. `scopes` are the requested scopes, sorted.
export interface ConsentContentTemplate {
    source: 'local' | 'remote';
    content_hash: string;
    name?: string;
    scopes: ConsentScope[];
    intro?: string;
    terms_url?: string;
    /// always accepted, shown checked and locked
    default_scopes: string[];
    /// the scopes of the user's current grant
    granted_scopes: string[];
}
