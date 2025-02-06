import type {I18nAdmin} from "./interface.ts";

export let I18nAdminEn: I18nAdmin = {
    common: {
        account: 'Account',
        back: 'Back',
        copiedToClip: 'Value has been copied to clipboard',
        copyToClip: 'Copy value to clipboard',
        filter: "Filter",
        from: "From",
        search: 'Search',
        searchOptions: 'Search Options',
        until: "Until",
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
    nav: {
        apiKeys: 'API Keys',
        attributes: 'Attributes',
        blacklist: 'Blacklist',
        clients: 'Clients',
        config: 'Config',
        docs: 'Docs',
        events: 'Events',
        groups: 'Groups',
        providers: 'Providers',
        roles: 'Roles',
        scopes: 'Scopes',
        sessions: 'Sessions',
        users: 'Users',
    },
    search: {
        orderBy: 'Order by ...',
        orderChangeToAsc: 'Change sort to ascending',
        orderChangeToDesc: 'Change sort to descending',
    }
};