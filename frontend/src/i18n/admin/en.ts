import type {I18nAdmin} from "./interface.ts";

export let I18nAdminEn: I18nAdmin = {
    api_key: {
        delete1: "Are you sure, that you want to delete this API Key?",
        expires: "Expiry",
        generate1: "Here you can generate a new secret for this API Key.",
        generate2: `You will only see this secret once after the generation.
            When a new one has been generated, the old secret will be overridden permanently.
            This operation cannot be reverted!`,
        generate3: `An API Key must be provided in the HTTP <code>Authorization</code>
            header in the following format:`,
        generate4: "You can use the following <code>curl</code> request to test your new Key:",
        generate5: "If you don't have <code>jq</code> installed and the above fails:",
        keyName: "Key Name",
        limitedValidity: "Limited Validity",
    },
    common: {
        account: "Account",
        addNew: "Add New",
        back: "Back",
        copiedToClip: "Value has been copied to clipboard",
        details: "Details",
        edit: "Edit",
        enabled: "Enabled",
        filter: "Filter",
        from: "From",
        loading: "Loading",
        name: "Name",
        nameExistsAlready: "Name exists already",
        note: "Note",
        noEntries: "No Entries",
        reset: "Reset",
        search: "Search",
        searchOptions: "Search Options",
        until: "Until",
    },
    docs: {
        book: "For general documentation about Rauthy itself, you should take a look at the",
        encryption: "Encryption",
        encKeys: {
            header: "Encryption Keys",
            keyActive: "Active Key",
            keysAvailable: "Available Keys",
            migrate: "Migrate",
            migrateToKey: 'Migrate all existing encrypted values to the following key',
            p1: `These Keys are used for an additional encryption at rest, independently from any data store technology 
            used under the hood. They are configured statically, but can be rotated and migrated on this page manually.`,
            p2: `The active key is statically set in the Rauthy config file / environment variables. It cannot be changed
            here dynamically. All new JWK encryption's will always use the currently active key.`,
            p3: `If you migrate all existing secrets, it might take a few seconds to finish, if you have a big 
            dataset.`,
        },
        hashing: {
            calculate: "Calculate",

            currValuesHead: 'Current values',
            currValues1: 'The current values from the backend are the following:',
            currValuesNote: `Note: The Login Time from the backend does only provide a good guideline after at least 5 
            successful logins, after Rauthy has been started. The base value is always 2000 ms after a fresh restart 
            and will adjust over time with each successful login.`,
            currValuesThreadsAccess: 'Threads (p_cost) Rauthy has access to',

            loginTimeHead: 'A word about Login Time',
            loginTime1: `Generally, users want everything as fast as possible. When doing a safe login though, a time 
            between 500 - 1000 ms should not be a problem. The login time must not be too short, since it would lower 
            the strength of the hash, of course.`,
            loginTime2: `To provide as much safety by default as possible, this utility does not allow you to go below 
            500 ms for the login time.`,

            mCost1: `The <code>m_cost</code> defines the amount of <b>memory (in kB)</b>, which is used for the hashing.
            The higher the value, the better, of course. But you need to keep in mind the servers resources.<br>
            When you hash 4 passwords at the same time, for instance, the backend needs <code>4 x m_cost</code>
            during the hashing. These resources must be available.`,
            mCost2: `Tuning <code>m_cost</code> is pretty easy. Define the max amount of memory that Rauthy should use,
            divide it by the number of max allowed parallel logins (<code>MAX_HASH_THREADS</code>) and subtract a small 
            static amount of memory. How much static memory should be taken into account depends on the used database
            and the total amount of users, but will typically be in the range of 32 - 96 MB.`,
            mCost3: 'The minimal allowed <code>m_cost</code> is <code>32768</code>.',

            pCost1: `The <code>p_cost</code> defines the amount of <b>parallelism</b> for hashing. This value most often 
            tops out at ~8, which is the default for Rauthy.`,
            pCost2: `The general rule is:<br>
            Set the <code>p_cost</code> to twice the size of cores your have available.<br>
            For instance, if you have 4 cores available, set the <code>p_cost</code> to <code>8</code>.<br>
            However, this value must take the configured allowed parallel logins (<code>MAX_HASH_THREADS</code>) into
            account and be reduced accordingly.`,

            tCost1: `The <code>t_cost</code> defines the amount of <b>time</b> for hashing. This value is actually the 
            only value, that needs tuning, since <code>m_cost</code> and <code>p_cost</code> are basically given by the 
            environment.`,
            tCost2: `Tuning is easy: Set <code>m_cost</code> and <code>p_cost</code> accordingly and then increase
            <code>t_cost</code> as long as you have not reached your hashing-time-goal.`,

            utilityHead: 'Parameter Calculation Utility',
            utility1: `You can use this tool to approximate good values for your deployment. Keep in mind, that this 
            should be executed with Rauthy in its final place with all final resources available. You should execute 
            this utility during load to not over tune.`,
            utility2: `<code>m_cost</code> is optional and the safe minimal value of <code>32768</code> would be chosen, 
            if empty. <code>p_cost</code> is optional too and Rauthy will utilize all threads it can see, if empty.`,

            time: "Time",
            targetTime: "Target Time",
            tune: 'Important: These values need to be tuned on the final architecture!',
            pDetials: `If you want a detailed introduction to Argon2ID, many sources exist online. This guide just 
            gives very short overview about the values. Three of them need to be configured:`,
            pTune: `They change depending on the capabilities of the system. The more powerful the system, the more safe 
            these values can be.`,
            pUtility: `This utility helps you find the best Argon2ID settings for your platform.
            Argon2ID is currently the safest available password hashing algorithm. To use it to its fullest potential, 
            it has to be tuned for each deployment.`,
        },
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
    groups: {
        delete1: "Are you sure you want to delete this group?",
        name: "Group Name",
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
    options: {
        expires: "Expires",
        lastSeen: "Last Seen",
        state: "State",
    },
    passwordPolicy: {
        configDesc: "Policy for new passwords.",
        resetSet0: "The value 0 deactivates the requirement.",
        validForDays: "Valid For Days",
        validityNew: "Validity for new passwords.",
    },
    providers: {
        config: {
            allowInsecureTls: "Allow insecure TLS",
            clientName: "Client Name",
            custRootCa: "Custom Root CA PEM",
            descAuthMethod: `The authentication method to use on the <code>/token</code> endpoint.<br>
                Most providers should work with <code>basic</code>, some only with <code>post</code>.
                In rare situations, you need both, while it can lead to errors with others.`,
            descClientId: "Client ID given by the auth provider.",
            descClientName: "Client name that should be shown on the Rauthy login page.",
            descClientSecret: `Client Secret given by the auth provider.
                At least a client secret or PKCE is required.`,
            descScope: `The scope the client should use when redirecting to the login.
                Provide the values separated by space.`,
            errNoAuthMethod: "You have given a client secret, but no client auth method is active",
            errConfidential: "Must at least be a confidential client or use PKCE",
            jsonPath: {
                p1: "Values from the ID token after a successful upstream login can be mapped automatically.",
                p2: `The <code>path</code> needs to be given in a regex like syntax. It can resolve to
                    single JSON values or resolve to a value in a JSON object or array.`,
                p3: "<code>$.</code> marks the start of the JSON object",
                p4: "<code>*</code> can be used as a wildcard in your path",
                p5: "<code>$.roles</code> would target <code>&#123;\"roles\": \"value\"&#125;</code>",
                p6: `<code>$.roles.*</code> can target a value inside an object or array like<br>
                    <code>&#123;"roles": ["value", "notMyValue"]&#125;</code>`,
            },
            lookup: "Lookup",
            pathAdminClaim: "Admin Claim Path",
            pathMfaClaim: "MFA Claim Path",
            rootPemCert: "Root PEM Certificate",
            mapMfa: `If your provider issues a claim indicating that the user has used at least 2FA during
                login, you can specify the mfa claim path.`,
            mapUser: `You can map a user to be a Rauthy admin depending on an upstream ID claim.`,
            valueAdminClaim: "Admin Claim Value",
            valueMfaClaim: "MFA Claim Value",
        },
        delete: {
            areYouSure: "Are you sure you want to delete this provider?",
            forceDelete: "Force Delete",
            isInUse1: "This provider is in use by active users!",
            isInUse2: `You can force delete it, but users without a local password or passkey
                will not be able to log in anymore.`,
            linkedUsers: "Linked Users",
        },
    },
    roles: {
        adminNoMod: "The <code>rauthy_admin</code> is immutable.",
        delete1: "Are you sure you want to delete this role?",
        name: "Role Name",
    },
    scopes: {
        defaultNoMod: "This is a default OIDC Scope. These are immutable.",
        delete1: "Are you sure you want to delete this scope?",
        deleteDefault: "OIDC Default Scopes können nicht gelöscht werden.",
        mapping1: "You can map custom scopes to attributes.",
        mapping2: `All additional attributes, that were configured, can have a custom value for each user.
            When they are mapped to a scope, they can be included in the Access and / or ID Tokens.`,
        name: "Scope Name",
    },
    search: {
        orderBy: "Order by ...",
        orderChangeToAsc: "Change sort to ascending",
        orderChangeToDesc: "Change sort to descending",
    },
    sessions: {
        invalidateAll: "Invalidate All Sessions",
    },
    tabs: {
        config: "Config",
        delete: "Delete",
    },
};