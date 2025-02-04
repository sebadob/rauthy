import type {I18nAdmin} from "./interface.ts";

export let I18nAdminEn: I18nAdmin = {
    common: {
        account: 'Account',
        back: 'Back',
    },
    error: {
        needsAdminRole: `You are not assigned to the <b>rauthy_admin</b> role.<br/>
            You do not have access to the admin panel.`,
        noAdmin: `A Rauthy admin account must have <b>MFA enabled.</b><br>
            Please navigate to your <b>account</b> and activate MFA.<br>
            Afterward, you need to do a logout and log back in.`,
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
    }
};