import type { I18nAdmin } from './interface.ts';

export let I18nAdminEn: I18nAdmin = {
    api_key: {
        delete1: 'Are you sure, that you want to delete this API Key?',
        expires: 'Expiry',
        generate1: 'Here you can generate a new secret for this API Key.',
        generate2: `You will only see this secret once after the generation.
            When a new one has been generated, the old secret will be overridden permanently.
            This operation cannot be reverted!`,
        generate3: `An API Key must be provided in the HTTP <code>Authorization</code>
            header in the following format:`,
        generate4: 'You can use the following <code>curl</code> request to test your new Key:',
        generate5: "If you don't have <code>jq</code> installed and the above fails:",
        keyName: 'Key Name',
        limitedValidity: 'Limited Validity',
    },
    attrs: {
        delete1: 'Are you sure you want to delete this attribute?',
        defaultValue: 'Default Value',
        desc: 'Description',
        makeEditable: 'Make Editable',
        makeEditableP1: 'You can convert this attribute and make it editable by users themselves.',
        makeEditableP2: `<b>CAUTION:</b> This can never be changed back! All inputs from a user directly are always
            untrusted data and MUST NEVER be used for any form of authentication or authorization!`,
        makeEditableP3: `An attribute cannot be changed from editable to non-editable, because it allowed untrusted
            inputs in the past, no matter for how long this was the case.`,
        name: 'Attribute Name',
        userEditable: 'User Editable',
    },
    backup: {
        createBackup: 'Create Backup',
        disabledDesc: 'This functionality only exists, if Hiqlite is configured as the database.',
        lastModified: 'Last Modified',
        local: 'Local',
        name: 'Name',
        size: 'Size',
    },
    clients: {
        backchannelLogout: 'If this client supports {{ OIDC_BCL }}, you can provide the URI here.',
        branding: {
            descHsl: `The following values must be given as HSL values. You only provide the base colors.
            Alpha channels and other values are manipulated dynamically by the theme.`,
            descFullCss: `The following values must be fully valid CSS <code>color</code> values.
                You can also use complex calculations or the above defined CSS variables.`,
            descVariables: `Each following label is at the same time the name of the CSS variable. This means,
                that you can reference these in the free inputs, e.g. with <code>hsla(var(--action) / .7)</code>.`,
        },
        confidential: 'Confidential',
        confidentialNoSecret: 'This is a non-confidential client and therefore has not secret.',
        config: 'Client Configuration',
        delete1: 'Are you sure you want to delete this client?',
        descAuthCode: `The validity for auth codes can be adjusted for increased security. Auth codes
            can be used only once and are valid for 60 seconds by default. The shorter the validity, the
            better, as long as the client can perform the login procedure fast enough.`,
        descClientUri: `Information about this clients URI and contacts to be shown on
            the login page.`,
        descName: `The client name can be changed without any impact on the client configuration.
            It only exists to be shown on the login page.`,
        descGroupPrefix: `The login to this client may be restricted by an optional group prefix.
            Only users, that are assigned to a matching group, will be allowed to log in.`,
        descOrigin: `External, additionally allowed origins - usually only necessary, if this client
            needs to make requests to Rauthy directly from the browser, typically SPAs.`,
        descPKCE: `If the client supports it, you should always activate S256 PKCE for additional 
            security. If a non-confidential client (e.g. a SPA) is being used, you must at least 
            activate one of the PKCE challenges to have enough security.`,
        descPKCEEnforce: `If any PKCE is activated, Rauthy will enforce the usage during Logins, and
            rejects login request that do not contain a valida challenge.`,
        descUri: `You can provide as many redirect URIs as you like. At the end of each, you can use 
            <code>*</code> as a Wildcard.`,
        errConfidentialPKCE: `The client must either be confidential or have at least one PKCE
            challenge activated.`,
        forceMfa: 'Force MFA',
        groupLoginPrefix: 'Login Group Prefix',
        name: 'Client Name',
        scim: {
            baseUri: `The SCIM base URI is the one from which the sub routes like 
                <code>{base_uri}/Users/{id}</base_uri></code> can be derived correctly.`,
            desc: 'If this client supports {{ SCIM_LINK }}, you can activate it here.',
            enable: 'Enable SCIMv2',
            groupSync: 'Synchronize Groups',
            groupSyncPrefix: 'Groups Filter Prefix',
            groupSyncPrefixDesc: `You can filter the groups for the synchronization by an optional prefix.
                For instance, if the groups <code>app:admins</code> and <code>app:users</code> exist, the prefix
                 <code>app:</code> would only sync these groups, as well as only those users that are linked to at least
                 one of these groups.`,
            reqDesc: 'A few things are required for compatibility:',
            reqLi1: 'The client must handle <code>externalId</code> correctly.',
            reqLi2: `At least <code>/Users</code> endpoints with <code>filter=externalId eq "*"</code> and
                <code>filter=userName eq "*"</code> must be supported.`,
            reqLi3: `If groups should be synchronized, <code>/Groups</code> must also support 
                <code>filter=displayName eq "*"</code>.`,
        },
        scopes: {
            allowed: 'Allowed Scopes',
            default: 'Default Scopes',
            desc: `Allowed Scopes are the ones the client is allowed to request dynamically during
            a redirect to the login when using the <code>authorization_code</code> flow. The default
            scopes will always be added to the tokens to solve some issues when using the 
            <code>password</code> for instance.`,
        },
        secret: {
            doCache: 'Cache Client Secret',
            cacheDuration: 'Cache Duration (hours)',
            generate: 'Generate New Secret',
            rotateDesc1: `To make graceful updates and secret rotations possible, you have the ability to keep the
                current secret in an in-memory cache for some time. You can enter a value between 1 and 24 hours.`,
            rotateDesc2: 'Caution: You should not cache the current secret if you had a leak!',
        },
        tokenLifetime: {
            p1: `The token lifetime applies to Access and ID tokens and is given in seconds.`,
            p2: `If the client supports EdDSA / ed25519 algorithms, it should always be the preferred
                choice. RSA algorithms exist for compatibility only.`,
            p3: `The algorithm for refresh tokens cannot be changed, since these are used by Rauthy only.`,
        },
    },
    common: {
        account: 'Account',
        addNew: 'Add New',
        back: 'Back',
        caution: 'CAUTION',
        contact: 'Contact',
        copiedToClip: 'Value has been copied to clipboard',
        details: 'Details',
        edit: 'Edit',
        enabled: 'Enabled',
        filter: 'Filter',
        from: 'From',
        information: 'Information',
        language: 'Language',
        loading: 'Loading',
        name: 'Name',
        nameExistsAlready: 'Name exists already',
        note: 'Note',
        noEntries: 'No Entries',
        preview: 'Preview',
        reset: 'Reset',
        searchOptions: 'Search Options',
        until: 'Until',
    },
    docs: {
        book: 'For general documentation about Rauthy itself, you should take a look at the',
        encryption: 'Encryption',
        encKeys: {
            header: 'Encryption Keys',
            keyActive: 'Active Key',
            keysAvailable: 'Available Keys',
            migrate: 'Migrate',
            migrateToKey: 'Migrate all existing encrypted values to the following key',
            p1: `These Keys are used for an additional encryption at rest, independently from any data store technology 
            used under the hood. They are configured statically, but can be rotated and migrated on this page manually.`,
            p2: `The active key is statically set in the Rauthy config file / environment variables. It cannot be changed
            here dynamically. All new JWK encryption's will always use the currently active key.`,
            p3: `If you migrate all existing secrets, it might take a few seconds to finish, if you have a big 
            dataset.`,
            pNotPossible: 'To be able to migrate, at least 2 encryption keys need to be available.',
        },
        hashing: {
            calculate: 'Calculate',

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

            time: 'Time',
            targetTime: 'Target Time',
            tune: 'Important: These values need to be tuned on the final architecture!',
            pDetials: `If you want a detailed introduction to Argon2ID, many sources exist online. This guide just 
            gives very short overview about the values. Three of them need to be configured:`,
            pTune: `They change depending on the capabilities of the system. The more powerful the system, the more safe 
            these values can be.`,
            pUtility: `This utility helps you find the best Argon2ID settings for your platform.
            Argon2ID is currently the safest available password hashing algorithm. To use it to its fullest potential, 
            it has to be tuned for each deployment.`,
        },
        openapi:
            "If you want to integrate an external application and use Rauthy's API, take a look at the",
        openapiNote: `Depending on the backend configuration, the Swagger UI may not be exposed publicly at this point.
            It is however by default available via the internal metrics HTTP server to not expose any 
            information.`,
        source: 'The source code can be found here',
    },
    editor: {
        bold: 'Bold',
        code: 'Code',
        heading1: 'Heading 1',
        heading2: 'Heading 2',
        heading3: 'Heading 3',
        italic: 'Italic',
        link: 'Link',
        listBullet: 'List',
        listTasks: 'Tasks',
        listNumbered: 'Numbered List',
        paragraph: 'Paragraph',
        quote: 'Quote',
        removeFmt: 'Remove Formatting',
        strikeThrough: 'Strikethrough',
        textArea: 'Edit Text',
    },
    email: {
        cancelJob: 'Cancel Job',
        filterType: ['None', 'In Group', 'Not in Group', 'Has Role', 'Has not Role'],
        immediate: 'Immediate',
        jobs: 'E-Mail Jobs',
        scheduled: 'Scheduled Dispatch',
        sendAllUsers: 'This E-Mail will be sent to all users.',
        sendAllUsersFiltered: 'This E-Mail will be sent to all users filtered by:',
        sendMail: 'Send E-Mail',
        subject: 'Subject',
        userFilter: 'User Filter',
    },
    error: {
        needsAdminRole: `You are not assigned to the <b>rauthy_admin</b> role.<br/>
            You do not have access to the admin panel.`,
        noAdmin: `A Rauthy admin account must have <b>MFA enabled.</b><br>
            Please navigate to your <b>account</b> and activate MFA.<br>
            Afterward, you need to do a logout and log back in.`,
    },
    events: {
        eventLevel: 'Event Level',
        eventType: 'Event Type',
    },
    groups: {
        delete1: 'Are you sure you want to delete this group?',
        name: 'Group Name',
    },
    jwks: {
        alg: 'Algorithm',
        p1: 'These are the Json Web Keys (JWKs) used for token singing.',
        p2: `The JWKs will be rotated by default every 1st of a month. For all newly created tokens, only the latest
        available key for the given algorithm will be used for signing. Old keys will be kept for a while to make sure
        that currently valid tokens can still be validated properly. After a while, they will be cleaned up 
        automatically.`,
        p3: `Keys can also be rotated manually. Depending on the hardware this Rauthy instance is running on, it might
        take a few seconds.`,
        type: 'Type',
        rotateKeys: 'Rotate Keys',
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
    options: {
        expires: 'Expires',
        lastSeen: 'Last Seen',
        state: 'State',
    },
    pam: {
        addGroup: 'New PAM Group',
        addHost: 'New PAM Host',
        addUser: 'New PAM User',
        deleteHost: 'Do you really want to delete this host?',
        groupDescGeneric: `Generic groups are the counterpart to entries that are usually found in /etc/group. Users 
            can be assigned to these and they are returned to the system by NSS Lookups.`,
        groupDescHost: `Host groups are used to group hosts. NSS lookups of a host within the group return all other 
            hosts within it as a result. Users can access hosts by assigning them to a host group.`,
        groupDescLocal: `Local groups behave almost identically to Generic groups, with the difference that they have 
            an ID in the Rauthy database, but the NSS proxy on the respective host will convert it to an ID from 
            /etc/group. In this way, Rauthy users can be assigned to groups that already exist locally.`,
        groupDescUser: `User groups are managed automatically and tightly coupled with the user with the same
            username.`,
        groupDescWheel: `This group is special. It is immutable and is assigned to users dynamically depending on their
            group configuration.`,
        groupName: 'Groupname',
        groups: 'Groups',
        groupType: 'Group Type',
        hostAliases: 'Host Aliases',
        hostLocalPwdOnly: 'Local Password Login',
        hostLocalPwdOnlyInfo: `When Local Password Login is set, it overwrites Force MFA for local logins. At the same
            time, passkeys will never be requested (locally) during logins, even if a user is MFA-secured. This option
            should only be set if really necessary, for instance if MFA-secured users should be able to do local logins
            while not using hardware passkeys.`,
        ipAddresses: 'IP Addresses',
        member: 'Member',
        nameExistsAlready: 'Name is already taken',
        notes: 'Notes',
        secretShow: 'Show Secret',
        secretRotate: 'Rotate Secret',
        userEmail: 'Linked User E-Mail',
        username: 'Username',
        usernameNewDesc: `The Username should be chosen carefully. Once created, it cannot be changed easily afterwards
            for security reasons.`,
    },
    passwordPolicy: {
        configDesc: 'Policy for new passwords.',
        resetSet0: 'The value 0 deactivates the requirement.',
        validForDays: 'Valid For Days',
        validityNew: 'Validity for new passwords.',
    },
    providers: {
        config: {
            allowInsecureTls: 'Allow insecure TLS',
            autoLink: 'Auto-Link User',
            autoLinkDesc1: `If Auto-Link User is activated, the login via this provider will automatically link a
                possibly existing, non-linked user to this provider.`,
            autoLinkDesc2: `CAUTION: This option can be very dangerous and lead to account takeover if the provider
                does not fully validate E-Mail addresses for users and therefore makes it possible to add a foreign
                address for a user! MUST NEVER be used in such a case!`,
            clientName: 'Client Name',
            custRootCa: 'Custom Root CA PEM',
            descAuthMethod: `The authentication method to use on the <code>/token</code> endpoint.<br>
                Most providers should work with <code>basic</code>, some only with <code>post</code>.
                In rare situations, you need both, while it can lead to errors with others.`,
            descClientId: 'Client ID given by the auth provider.',
            descClientName: 'Client name that should be shown on the Rauthy login page.',
            descClientSecret: `Client Secret given by the auth provider.
                At least a client secret or PKCE is required.`,
            descScope: `The scope the client should use when redirecting to the login.
                Provide the values separated by space.`,
            errNoAuthMethod: 'You have given a client secret, but no client auth method is active',
            errConfidential: 'Must at least be a confidential client or use PKCE',
            jsonPath: {
                p1: 'Values from the ID token after a successful upstream login can be mapped automatically.',
                p2: `The <code>path</code> needs to be given in a regex like syntax. It can resolve to
                    single JSON values or resolve to a value in a JSON object or array.`,
                p3: '<code>$.</code> marks the start of the JSON object',
                p4: '<code>*</code> can be used as a wildcard in your path',
                p5: '<code>$.roles</code> would target <code>&#123;"roles": "value"&#125;</code>',
                p6: `<code>$.roles.*</code> can target a value inside an object or array like<br>
                    <code>&#123;"roles": ["value", "notMyValue"]&#125;</code>`,
            },
            lookup: 'Lookup',
            pathAdminClaim: 'Admin Claim Path',
            pathMfaClaim: 'MFA Claim Path',
            rootPemCert: 'Root PEM Certificate',
            mapMfa: `If your provider issues a claim indicating that the user has used at least 2FA during
                login, you can specify the mfa claim path.`,
            mapUser: `You can map a user to be a Rauthy admin depending on an upstream ID claim.`,
            valueAdminClaim: 'Admin Claim Value',
            valueMfaClaim: 'MFA Claim Value',
        },
        delete: {
            areYouSure: 'Are you sure you want to delete this provider?',
            forceDelete: 'Force Delete',
            isInUse1: 'This provider is in use by active users!',
            isInUse2: `You can force delete it, but users without a local password or passkey
                will not be able to log in anymore.`,
            linkedUsers: 'Linked Users',
        },
    },
    roles: {
        adminNoMod: 'The <code>rauthy_admin</code> role is immutable.',
        delete1: 'Are you sure you want to delete this role?',
        name: 'Role Name',
    },
    scopes: {
        defaultNoMod: 'This is a default OIDC Scope. These are immutable.',
        delete1: 'Are you sure you want to delete this scope?',
        deleteDefault: 'Default OIDC scopes cannot be deleted.',
        mapping1: 'You can map custom scopes to attributes.',
        mapping2: `All additional attributes, that were configured, can have a custom value for each user.
            When they are mapped to a scope, they can be included in the Access and / or ID Tokens.`,
        name: 'Scope Name',
    },
    search: {
        orderBy: 'Order by ...',
        orderChangeToAsc: 'Change sort to ascending',
        orderChangeToDesc: 'Change sort to descending',
    },
    sessions: {
        invalidateAll: 'Invalidate All Sessions',
    },
    tabs: {
        config: 'Config',
        delete: 'Delete',
    },
    tos: {
        accepted: 'Accepted',
        addNewToS: 'New ToS',
        addNewToSFromCurrent: 'New ToS from selected',
        added: 'Added',
        checkStatus: 'Check user status',
        immutable: `CAUTION: After adding new Terms of Service, they are immutable and cannot be 
			deleted!`,
        noneExist: 'No Terms of Service have been added yet.',
        optUntil: {
            desc: `During the transition time, accepting updated ToS is optional. It only becomes 
				mandatory afterward.`,
            enable: 'Enable Transition Time',
            label: 'End of Transition Time',
        },
        tos: 'ToS',
    },
    users: {
        antiLockout: {
            rule: 'Anti-Lockout Rule',
            delete: 'cannot be deleted',
            disable: 'cannot be disabled',
            rauthyAdmin: 'rauthy_admin rule cannot be removed',
        },
        attributes: 'Attributes',
        deleteUser: 'Are you sure you want to delete this user?',
        descAttr: `Set custom user attributes. All key / value pairs will be handles as String / JSON Value.`,
        forceLogout: `Are you sure you want to invalidate all existing sessions and delete all refresh tokens
            for this user?`,
        lastLogin: 'Last Login',
        manualInitDesc: `The user can also be initialized here, In this case though, you need to communicate the 
            password directly.`,
        manualInit: 'Manual Initialization',
        mfaDelete1: 'You can delete Passkeys for this users.',
        mfaDelete2: `Caution! The deletion of a Passkey <b>cannot be reverted</b> without the user
            doing a fully new registration.`,
        noMfaKeys: 'This user has no registered Passkeys.',
        pkOnly1: 'This is a passkey-only account.',
        pkOnly2:
            'This means that this user uses the passwordless login and has no password set at all.',
        pkOnly3: `If this user has lost all Passkeys, the account can be fully reset and a new password reset E-Mail
            can be sent. To achieve this, navigate to the 'MFA' tab an delete all existing passkeys.`,
        pwdNoInit: 'The user has not performed the initial password reset yet.',
        pwdSendEmailBtn: 'Send Reset E-Mail',
        pwdSendEmailDesc: 'You may send out a new reset E-Mail, if the user has not received one.',
        savePassword: 'Save Password',
        selfServiceDesc: 'You can either set a new password, or send out a reset E-Mail.',
        sendResetEmail: 'Send Reset E-Mail',
    },
    validation: {
        css: 'Invalid CSS Value',
        origin: 'Invalid Origin',
        uri: 'Invalid URI',
    },
};
