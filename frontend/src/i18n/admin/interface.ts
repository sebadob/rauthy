/**
 * Translations specific to only the Admin UI.
 * These must be a subset of the common translations. If a translations does not exist for the admin UI,
 * which does exist for the end user facing pages, the admin translation will fall back to EN.
 */
export interface I18nAdmin {
    common: {
        account: string,
        back: string,
        search: string,
        searchOptions: string,
    }
    error: {
        // will be inserted as html
        needsAdminRole: string,
        // will be inserted as html
        noAdmin: string,
    },
    nav: {
        apiKeys: string,
        attributes: string,
        blacklist: string,
        clients: string,
        config: string,
        docs: string,
        events: string,
        groups: string,
        providers: string,
        roles: string,
        scopes: string,
        sessions: string,
        users: string,
    }
}