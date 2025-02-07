import type {I18nAdmin} from "./interface.ts";

export let I18nAdminEn: I18nAdmin = {
    common: {
        account: 'Account',
        back: "Back",
        copiedToClip: "Value has been copied to clipboard",
        copyToClip: "Copy value to clipboard",
        filter: "Filter",
        from: "From",
        note: "Note",
        search: "Search",
        searchOptions: "Search Options",
        until: "Until",
    },
    docs: {
        book: "For general documentation about Rauthy itself, you should take a look at the",
        openapi: "If you want to integrate an external application and use Rauthy's API, take a look at the",
        openapiNote: `Depending on the backend configuration, the Swagger UI may not be exposed publicly at this point.
            It is however by default available via the internal metrics HTTP server to not expose any 
            information.`,
        source: "The source code can be found here",
    },
    error: {
        needsAdminRole: `You are not assigned to the <b>rauthy_admin</b> role.<br/>
            You do not have access to the admin panel.`,
        noAdmin: `A Rauthy admin account must have <b>MFA enabled.</b><br>
            Please navigate to your <b>account</b> and activate MFA.<br>
            Afterward, you need to do a logout and log back in.`,
    },
    events: {
        eventLevel: "Event Level",
        eventType: "Event Type",
    },
    jwks: {
        alg: "Algorithm",
        p1: "These are the Json Web Keys (JWKs) used for token singing.",
        p2: `The JWKs will be rotated by default every 1st of a month. For all newly created tokens, only the latest
        available key for the given algorithm will be used for signing. Old keys will be kept for a while to make sure
        that currently valid tokens can still be validated properly. After a while, they will be cleaned up 
        automatically.`,
        p3: `Keys can also be rotated manually. Depending on the hardware this Rauthy instance is running on, it might
        take a few seconds.`,
        type: "Type",
        rotateKeys: "Rotate Keys",
    },
    nav: {
        apiKeys: "API Keys",
        attributes: "Attributes",
        blacklist: "Blacklist",
        clients: "Clients",
        config: "Config",
        docs: "Docs",
        events: "Events",
        groups: "Groups",
        providers: "Providers",
        roles: "Roles",
        scopes: "Scopes",
        sessions: "Sessions",
        users: "Users",
    },
    passwordPolicy: {
        configDesc: "Policy for new passwords.",
        resetSet0: "The value 0 deactivates the requirement.",
        validForDays: "Valid For Days",
        validityNew: "Validity for new passwords.",
    },
    search: {
        orderBy: "Order by ...",
        orderChangeToAsc: "Change sort to ascending",
        orderChangeToDesc: "Change sort to descending",
    }
};