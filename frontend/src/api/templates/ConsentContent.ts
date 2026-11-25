export interface ConsentScope {
    name: string;
    /// only for `remote`
    title: string | null;
    /// only for `remote`, may contain HTML, of which only the text is shown
    description: string | null;
    warning: boolean;
}

/// The consent the login page asks for. `scopes` are the requested scopes, sorted.
export interface ConsentContentTemplate {
    source: 'local' | 'remote';
    content_hash: string;
    /// the name of the consent source, i.e. of the resource
    name: string | null;
    scopes: ConsentScope[];
    intro: string | null;
    terms_url: string | null;
    /// always accepted, shown checked and locked
    default_scopes: string[];
    /// the scopes of the user's current grant, empty without one or while the user is unknown
    granted_scopes: string[];
}
