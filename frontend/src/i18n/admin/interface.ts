/**
 * Translations specific to only the Admin UI.
 * These must be a subset of the common translations. If a translations does not exist for the admin UI,
 * which does exist for the end user facing pages, the admin translation will fall back to EN.
 */
export interface I18nAdmin {
    api_key: {
        delete1: string,
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
    attrs: {
        delete1: string,
        desc: string,
        name: string,
    },
    clients: {
        backchannelLogout: string,
        branding: {
            descHsl: string,
            // inserted as html
            descFullCss: string,
            descVariables: string,
        },
        confidential: string,
        confidentialNoSecret: string,
        config: string,
        delete1: string,
        descAuthCode: string,
        descClientUri: string,
        descName: string,
        descOrigin: string,
        descPKCE: string,
        descPKCEEnforce: string,
        // inserted as html
        descUri: string,
        errConfidentialPKCE: string,
        forceMfa: string,
        generateSecret: string,
        name: string,
        scopes: {
            allowed: string,
            default: string,
            // inserted as html
            desc: string,
        },
        tokenLifetime: {
            p1: string,
            p2: string,
            p3: string,
        },
    }
    common: {
        account: string,
        addNew: string,
        back: string,
        contact: string,
        copiedToClip: string,
        details: string,
        edit: string,
        enabled: string,
        filter: string,
        from: string,
        information: string,
        language: string,
        loading: string,
        name: string,
        nameExistsAlready: string,
        note: string,
        noEntries: string,
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
            pNotPossible: string,
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
    groups: {
        delete1: string,
        name: string,
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
    options: {
        expires: string,
        lastSeen: string,
        state: string,
    },
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
    roles: {
        // inserted as html
        adminNoMod: string,
        delete1: string,
        name: string,
    },
    scopes: {
        defaultNoMod: string,
        delete1: string,
        deleteDefault: string,
        mapping1: string,
        mapping2: string,
        name: string,
    },
    search: {
        orderBy: string,
        orderChangeToAsc: string,
        orderChangeToDesc: string,
    },
    sessions: {
        invalidateAll: string,
    },
    tabs: {
        config: string,
        delete: string,
    },
    users: {
        attributes: string,
        deleteUser: string,
        descAttr: string,
        forceLogout: string,
        lastLogin: string,
        mfaDelete1: string,
        // inserted as html
        mfaDelete2: string,
        noMfaKeys: string,
        pkOnly1: string,
        pkOnly2: string,
        pkOnly3: string,
        pwdNoInit: string,
        pwdSendEmailBtn: string,
        pwdSendEmailDesc: string,
        savePassword: string,
        selfServiceDesc: string,
        sendResetEmail: string,
    },
    validation: {
        css: string,
        origin: string,
        uri: string,
    }
}