/**
 * Translations specific to only the Admin UI.
 * These must be a subset of the common translations. If a translations does not exist for the admin UI,
 * which does exist for the end user facing pages, the admin translation will fall back to EN.
 */
export interface I18nAdmin {
    api_key: {
        delete1: string;
        expires: string;
        generate1: string;
        generate2: string;
        // inserted as html
        generate3: string;
        // inserted as html
        generate4: string;
        // inserted as html
        generate5: string;
        keyName: string;
        limitedValidity: string;
    };
    attrs: {
        delete1: string;
        defaultValue: string;
        desc: string;
        makeEditable: string;
        makeEditableP1: string;
        // inserted as html
        makeEditableP2: string;
        makeEditableP3: string;
        name: string;
        userEditable: string;
    };
    backup: {
        createBackup: string;
        disabledDesc: string;
        lastModified: string;
        local: string;
        name: string;
        size: string;
    };
    clients: {
        backchannelLogout: string;
        branding: {
            descHsl: string;
            // inserted as html
            descFullCss: string;
            descVariables: string;
        };
        confidential: string;
        confidentialNoSecret: string;
        config: string;
        delete1: string;
        descAuthCode: string;
        descClientUri: string;
        descGroupPrefix: string;
        descName: string;
        descOrigin: string;
        descPKCE: string;
        descPKCEEnforce: string;
        // inserted as html
        descUri: string;
        errConfidentialPKCE: string;
        forceMfa: string;
        groupLoginPrefix: string;
        name: string;
        scim: {
            // inserted as html
            baseUri: string;
            // inserted as html
            desc: string;
            enable: string;
            groupSync: string;
            groupSyncPrefix: string;
            groupSyncPrefixDesc: string;
            reqDesc: string;
            // inserted as html
            reqLi1: string;
            // inserted as html
            reqLi2: string;
            // inserted as html
            reqLi3: string;
        };
        scopes: {
            allowed: string;
            default: string;
            // inserted as html
            desc: string;
        };
        secret: {
            doCache: string;
            cacheDuration: string;
            generate: string;
            rotateDesc1: string;
            rotateDesc2: string;
        };
        tokenLifetime: {
            p1: string;
            p2: string;
            p3: string;
        };
    };
    common: {
        account: string;
        addNew: string;
        back: string;
        caution: string;
        contact: string;
        copiedToClip: string;
        details: string;
        edit: string;
        enabled: string;
        filter: string;
        from: string;
        information: string;
        language: string;
        loading: string;
        name: string;
        nameExistsAlready: string;
        note: string;
        noEntries: string;
        preview: string;
        reset: string;
        searchOptions: string;
        until: string;
    };
    docs: {
        book: string;
        encryption: string;
        encKeys: {
            header: string;
            keyActive: string;
            keysAvailable: string;
            migrate: string;
            migrateToKey: string;
            p1: string;
            p2: string;
            p3: string;
            pNotPossible: string;
        };
        hashing: {
            calculate: string;

            currValuesHead: string;
            currValues1: string;
            currValuesNote: string;
            currValuesThreadsAccess: string;

            loginTimeHead: string;
            loginTime1: string;
            loginTime2: string;

            // inserted as html
            mCost1: string;
            // inserted as html
            mCost2: string;
            // inserted as html
            mCost3: string;

            // inserted as html
            pCost1: string;
            // inserted as html
            pCost2: string;

            // inserted as html
            tCost1: string;
            // inserted as html
            tCost2: string;

            utilityHead: string;
            utility1: string;
            // inserted as html
            utility2: string;

            time: string;
            targetTime: string;
            tune: string;
            pDetials: string;
            pTune: string;
            pUtility: string;
        };
        openapi: string;
        openapiNote: string;
        source: string;
    };
    editor: {
        bold: string;
        code: string;
        heading1: string;
        heading2: string;
        heading3: string;
        italic: string;
        link: string;
        listBullet: string;
        listTasks: string;
        listNumbered: string;
        paragraph: string;
        quote: string;
        removeFmt: string;
        strikeThrough: string;
        textArea: string;
    };
    email: {
        cancelJob: string;
        // MUST be in the order:
        // 'none' | 'in_group' | 'not_in_group' | 'has_role' | 'has_not_role'
        filterType: string[];
        immediate: string;
        jobs: string;
        scheduled: string;
        sendAllUsers: string;
        sendAllUsersFiltered: string;
        sendMail: string;
        subject: string;
        userFilter: string;
    };
    error: {
        // inserted as html
        needsAdminRole: string;
        // inserted as html
        noAdmin: string;
    };
    events: {
        eventLevel: string;
        eventType: string;
    };
    groups: {
        delete1: string;
        name: string;
    };
    jwks: {
        alg: string;
        p1: string;
        p2: string;
        p3: string;
        type: string;
        rotateKeys: string;
    };
    nav: {
        apiKeys: string;
        attributes: string;
        blacklist: string;
        clients: string;
        config: string;
        docs: string;
        events: string;
        groups: string;
        providers: string;
        roles: string;
        scopes: string;
        sessions: string;
        users: string;
    };
    options: {
        expires: string;
        lastSeen: string;
        state: string;
    };
    pam: {
        addGroup: string;
        addHost: string;
        addUser: string;
        deleteHost: string;
        groupDescGeneric: string;
        groupDescHost: string;
        groupDescLocal: string;
        groupDescUser: string;
        groupDescWheel: string;
        groupName: string;
        groups: string;
        groupType: string;
        hostAliases: string;
        hostLocalPwdOnly: string;
        hostLocalPwdOnlyInfo: string;
        ipAddresses: string;
        member: string;
        nameExistsAlready: string;
        notes: string;
        secretShow: string;
        secretRotate: string;
        userEmail: string;
        username: string;
        usernameNewDesc: string;
    };
    passwordPolicy: {
        configDesc: string;
        resetSet0: string;
        validForDays: string;
        validityNew: string;
    };
    providers: {
        config: {
            allowInsecureTls: string;
            autoLink: string;
            autoLinkDesc1: string;
            autoLinkDesc2: string;
            clientName: string;
            custRootCa: string;
            // inserted as html
            descAuthMethod: string;
            descClientId: string;
            descClientName: string;
            descClientSecret: string;
            descScope: string;
            errNoAuthMethod: string;
            errConfidential: string;
            jsonPath: {
                p1: string;
                // inserted as html
                p2: string;
                // inserted as html
                p3: string;
                // inserted as html
                p4: string;
                // inserted as html
                p5: string;
                // inserted as html
                p6: string;
            };
            lookup: string;
            pathAdminClaim: string;
            pathMfaClaim: string;
            rootPemCert: string;
            mapMfa: string;
            mapUser: string;
            valueAdminClaim: string;
            valueMfaClaim: string;
        };
        delete: {
            areYouSure: string;
            forceDelete: string;
            isInUse1: string;
            isInUse2: string;
            linkedUsers: string;
        };
    };
    roles: {
        // inserted as html
        adminNoMod: string;
        delete1: string;
        name: string;
    };
    scopes: {
        defaultNoMod: string;
        delete1: string;
        deleteDefault: string;
        mapping1: string;
        mapping2: string;
        name: string;
    };
    search: {
        orderBy: string;
        orderChangeToAsc: string;
        orderChangeToDesc: string;
    };
    sessions: {
        invalidateAll: string;
    };
    tabs: {
        config: string;
        delete: string;
    };
    tos: {
        accepted: string;
        addNewToS: string;
        addNewToSFromCurrent: string;
        added: string;
        checkStatus: string;
        immutable: string;
        noneExist: string;
        optUntil: {
            desc: string;
            enable: string;
            label: string;
        };
        tos: string;
    };
    users: {
        antiLockout: {
            rule: string;
            delete: string;
            disable: string;
            rauthyAdmin: string;
        };
        attributes: string;
        deleteUser: string;
        descAttr: string;
        forceLogout: string;
        lastLogin: string;
        manualInitDesc: string;
        manualInit: string;
        mfaDelete1: string;
        // inserted as html
        mfaDelete2: string;
        noMfaKeys: string;
        pkOnly1: string;
        pkOnly2: string;
        pkOnly3: string;
        pwdNoInit: string;
        pwdSendEmailBtn: string;
        pwdSendEmailDesc: string;
        savePassword: string;
        selfServiceDesc: string;
        sendResetEmail: string;
    };
    validation: {
        css: string;
        origin: string;
        uri: string;
    };
}
