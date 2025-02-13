/**
 * Translations specific to only the Admin UI.
 * These must be a subset of the common translations. If a translations does not exist for the admin UI,
 * which does exist for the end user facing pages, the admin translation will fall back to EN.
 */
export interface I18nAdmin {
    api_key: {
        expires: string,
        generate1: string,
        generate2: string,
        // inserted as html
        generate3: string,
        // inserted as html
        generate4: string,
        // inserted as html
        generate5: string,
        keyName: string,
        limitedValidity: string,
    },
    common: {
        account: string,
        addNew: string,
        back: string,
        copiedToClip: string,
        details: string,
        enabled: string,
        filter: string,
        from: string,
        loading: string,
        note: string,
        reset: string,
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
        config: {
            allowInsecureTls: string,
            clientName: string,
            custRootCa: string,
            // inserted as html
            descAuthMethod: string,
            descClientId: string,
            descClientName: string,
            descClientSecret: string,
            descScope: string,
            errNoAuthMethod: string,
            errConfidential: string,
            jsonPath: {
                p1: string,
                // inserted as html
                p2: string,
                // inserted as html
                p3: string,
                // inserted as html
                p4: string,
                // inserted as html
                p5: string,
                // inserted as html
                p6: string,
            },
            lookup: string,
            pathAdminClaim: string,
            pathMfaClaim: string,
            rootPemCert: string,
            mapMfa: string,
            mapUser: string,
            valueAdminClaim: string,
            valueMfaClaim: string,
        },
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
    tabs: {
        config: string,
        delete: string,
    }
}