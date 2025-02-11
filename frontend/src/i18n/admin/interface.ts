/**
 * Translations specific to only the Admin UI.
 * These must be a subset of the common translations. If a translations does not exist for the admin UI,
 * which does exist for the end user facing pages, the admin translation will fall back to EN.
 */
export interface I18nAdmin {
    common: {
        account: string,
        addNew: string,
        back: string,
        copiedToClip: string,
        copyToClip: string,
        details: string,
        filter: string,
        from: string,
        loading: string,
        note: string,
        search: string,
        searchOptions: string,
        until: string,
    }
    docs: {
        book: string,
        encryption: string,
        encKeys: {
            header: string,
            keyActive: string,
            keysAvailable: string,
            migrate: string,
            migrateToKey: string,
            p1: string,
            p2: string,
            p3: string,
        },
        hashing: {
            calculate: string,

            currValuesHead: string,
            currValues1: string,
            currValuesNote: string,
            currValuesThreadsAccess: string,

            loginTimeHead: string,
            loginTime1: string,
            loginTime2: string,

            // inserted as html
            mCost1: string,
            // inserted as html
            mCost2: string,
            // inserted as html
            mCost3: string,

            // inserted as html
            pCost1: string,
            // inserted as html
            pCost2: string,

            // inserted as html
            tCost1: string,
            // inserted as html
            tCost2: string,

            utilityHead: string,
            utility1: string,
            // inserted as html
            utility2: string,

            time: string,
            targetTime: string,
            tune: string,
            pDetials: string,
            pTune: string,
            pUtility: string,
        },
        openapi: string,
        openapiNote: string,
        source: string,
    }
    error: {
        // inserted as html
        needsAdminRole: string,
        // inserted as html
        noAdmin: string,
    },
    events: {
        eventLevel: string,
        eventType: string,
    },
    jwks: {
        alg: string,
        p1: string,
        p2: string,
        p3: string,
        type: string,
        rotateKeys: string,
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
    passwordPolicy: {
        configDesc: string,
        resetSet0: string,
        validForDays: string,
        validityNew: string,
    },
    providers: {
        delete: {
            areYouSure: string,
            forceDelete: string,
            isInUse1: string,
            isInUse2: string,
            linkedUsers: string,
        }
    }
    search: {
        orderBy: string,
        orderChangeToAsc: string,
        orderChangeToDesc: string,
    }
}