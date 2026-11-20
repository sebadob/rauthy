<script lang="ts">
    import { tick, untrack } from 'svelte';
    import ClientSecret from '#lib/admin/clients/ClientSecret.svelte';
    import Button from '#lib/button/Button.svelte';
    import Input from '#lib/form/Input.svelte';
    import IconCheck from '#icons/IconCheck.svelte';
    import { useI18n } from '#state/i18n.svelte.ts';
    import { useI18nAdmin } from '#state/i18n_admin.svelte.ts';
    import { fetchGet, fetchPut } from '#api/fetch.ts';
    import Form from '#lib/form/Form.svelte';
    import LabeledValue from '#lib/LabeledValue.svelte';
    import Section from '#lib/Section.svelte';
    import {
        PATTERN_CLIENT_NAME,
        PATTERN_CLIENT_URI,
        PATTERN_CONSENT_URL,
        PATTERN_CONTACT,
        PATTERN_GROUP,
        PATTERN_ORIGIN,
        PATTERN_RESOURCE,
        PATTERN_URI,
    } from '#utils/patterns.ts';
    import {
        AuthFlowDeviceCode,
        AuthFlowTokenExchange,
        type ClientResponse,
        type ScimClientRequestResponse,
        type UpdateClientRequest,
    } from '#api/types/clients.ts';
    import type { JwkKeyPairAlg } from '#api/types/oidc.ts';
    import InputTags from '#lib/form/InputTags.svelte';
    import InputCheckbox from '#lib/form/InputCheckbox.svelte';
    import SelectList from '#lib/select_list/SelectList.svelte';
    import type { SelectItem } from '#lib/select_list/props.ts';
    import { slide } from 'svelte/transition';
    import Options from '#lib/Options.svelte';
    import InputPassword from '#lib/form/InputPassword.svelte';
    import InputArea from '#lib/form/InputArea.svelte';
    import { parseJsonValue, stringifyJsonValue } from '#utils/jsonValue.ts';
    import {
        invalidPostLogoutRedirectUrisMsg,
        invalidRedirectUrisMsg,
        postLogoutRedirectUriShapeErrorMsg,
        redirectUriShapeErrorMsg,
    } from '#utils/redirectUri.ts';

    let {
        section = $bindable(0),
        client,
        clients,
        scopesAll,
        onSave,
    }: {
        section: number;
        client: ClientResponse;
        clients: ClientResponse[];
        scopesAll: string[];
        onSave: () => void;
    } = $props();

    let t = useI18n();
    let ta = useI18nAdmin();

    const inputWidth = '100%';
    let err = $state('');
    let success = $state(false);

    const initialClient = untrack(() => client);

    let name: string = $state(initialClient.name || '');
    let enabled = $state(initialClient.enabled);
    let confidential = $state(initialClient.confidential);
    let uri: string = $state(initialClient.client_uri || '');
    let contacts: string[] = $state(
        initialClient.contacts ? Array.from(initialClient.contacts) : [],
    );
    let origins: string[] = $state(
        initialClient.allowed_origins ? Array.from(initialClient.allowed_origins) : [],
    );
    let redirectURIs: string[] = $state(Array.from(initialClient.redirect_uris));
    let postLogoutRedirectURIs: string[] = $state(
        initialClient.post_logout_redirect_uris
            ? Array.from(initialClient.post_logout_redirect_uris)
            : [],
    );
    // A client stored before the stricter validation may still contain an invalid (post-logout)
    // redirect URI, which must be replaced before the client can be saved.
    let redirectURIsErr = $derived(
        invalidRedirectUrisMsg(redirectURIs, ta.validation.redirectUri) || '',
    );
    let postLogoutRedirectURIsErr = $derived(
        invalidPostLogoutRedirectUrisMsg(postLogoutRedirectURIs, ta.validation.redirectUri) || '',
    );
    let backchannel_logout_uri: string = $state(initialClient.backchannel_logout_uri || '');
    let restrict_group_prefix: string = $state(initialClient.restrict_group_prefix || '');
    let allowedResources: string[] = $state(
        initialClient.allowed_resources ? Array.from(initialClient.allowed_resources) : [],
    );
    let defaultAud: string[] = $state(
        initialClient.default_aud ? Array.from(initialClient.default_aud) : [],
    );

    // `local` and unset behave the same, both are shown as built-in
    const consentUrl = 'URL';
    let consentOptions = $derived([ta.clients.consent.builtIn, consentUrl]);
    let consentMode: string = $state(
        untrack(() =>
            isConsentUrl(initialClient.consent) ? consentUrl : ta.clients.consent.builtIn,
        ),
    );
    let consentSourceUrl: string = $state(
        untrack(() => (isConsentUrl(initialClient.consent) ? initialClient.consent || '' : '')),
    );

    let scimEnabled = $state(initialClient.scim !== undefined);
    let scim: ScimClientRequestResponse = $state({
        base_uri: initialClient.scim?.base_uri || '',
        bearer_token: initialClient.scim?.bearer_token || '',
        // group sync defaults to on only when SCIM is not configured yet
        sync_groups: initialClient.scim ? initialClient.scim.sync_groups : true,
        group_sync_prefix: initialClient.scim?.group_sync_prefix || '',
    });

    let flows = $state({
        authorizationCode: initialClient.flows_enabled.includes('authorization_code'),
        clientCredentials: initialClient.flows_enabled.includes('client_credentials'),
        password: initialClient.flows_enabled.includes('password'),
        refreshToken: initialClient.flows_enabled.includes('refresh_token'),
        deviceCode: initialClient.flows_enabled.includes(AuthFlowDeviceCode),
        tokenExchange: initialClient.flows_enabled.includes(AuthFlowTokenExchange),
    });

    const optionsAlgs: JwkKeyPairAlg[] = ['RS256', 'RS384', 'RS512', 'EdDSA', 'Ed25519'];
    let accessTokenAlg: JwkKeyPairAlg = $state(initialClient.access_token_alg);
    let idTokenAlg: JwkKeyPairAlg = $state(initialClient.id_token_alg);
    let tokenLifetime: string = $state(initialClient.access_token_lifetime.toString());
    let authCodeLifetime: string = $state(initialClient.auth_code_lifetime.toString());

    let scopes: SelectItem[] = $state(
        untrack(() =>
            scopesAll.map(name => {
                let i: SelectItem = {
                    name,
                    selected: initialClient.scopes.includes(name) || false,
                };
                return i;
            }),
        ),
    );
    let defaultScopes: SelectItem[] = $state(
        untrack(() =>
            scopesAll.map(name => {
                let i: SelectItem = {
                    name,
                    selected: initialClient.default_scopes.includes(name) || false,
                };
                return i;
            }),
        ),
    );
    let challenges = $state({
        plain: initialClient.challenges?.includes('plain') || false,
        s256: initialClient.challenges?.includes('S256') || false,
    });

    let forceMfa = $state(initialClient.force_mfa);

    let jsonClaims = $state(untrack(() => stringifyJsonValue(initialClient.claims) || ''));
    let claimsAtRoot = $state(untrack(() => initialClient.claims_at_root));
    let actorSameClient = $state(untrack(() => initialClient.actor_same_client));

    $effect(() => {
        if (client.id) {
            name = client.name || '';
            enabled = client.enabled;
            forceMfa = client.force_mfa;
            confidential = client.confidential;
            uri = client.client_uri || '';
            backchannel_logout_uri = client.backchannel_logout_uri || '';
            restrict_group_prefix = client.restrict_group_prefix || '';
            contacts = client.contacts ? Array.from(client.contacts) : [];
            origins = client.allowed_origins ? Array.from(client.allowed_origins) : [];
            allowedResources = client.allowed_resources ? Array.from(client.allowed_resources) : [];
            defaultAud = client.default_aud ? Array.from(client.default_aud) : [];
            consentMode = isConsentUrl(client.consent) ? consentUrl : ta.clients.consent.builtIn;
            consentSourceUrl = isConsentUrl(client.consent) ? client.consent || '' : '';
            redirectURIs = Array.from(client.redirect_uris);
            postLogoutRedirectURIs = client.post_logout_redirect_uris
                ? Array.from(client.post_logout_redirect_uris)
                : [];

            flows.authorizationCode = client.flows_enabled.includes('authorization_code');
            flows.clientCredentials = client.flows_enabled.includes('client_credentials');
            flows.password = client.flows_enabled.includes('password');
            flows.refreshToken = client.flows_enabled.includes('refresh_token');
            flows.deviceCode = client.flows_enabled.includes(AuthFlowDeviceCode);
            flows.tokenExchange = client.flows_enabled.includes(AuthFlowTokenExchange);

            accessTokenAlg = client.access_token_alg;
            idTokenAlg = client.id_token_alg;
            tokenLifetime = client.access_token_lifetime.toString();
            authCodeLifetime = client.auth_code_lifetime.toString();

            scopes = scopesAll.map(name => {
                let i: SelectItem = {
                    name,
                    selected: client.scopes.includes(name) || false,
                };
                return i;
            });
            defaultScopes = scopesAll.map(name => {
                let i: SelectItem = {
                    name,
                    selected: client.default_scopes.includes(name) || false,
                };
                return i;
            });
            challenges.plain = client.challenges?.includes('plain') || false;
            challenges.s256 = client.challenges?.includes('S256') || false;

            jsonClaims = stringifyJsonValue(client.claims) || '';
            claimsAtRoot = client.claims_at_root;
            actorSameClient = client.actor_same_client;

            scim = {
                base_uri: client.scim?.base_uri || '',
                bearer_token: client.scim?.bearer_token || '',
                sync_groups: client.scim ? client.scim.sync_groups : true,
                group_sync_prefix: client.scim?.group_sync_prefix || '',
            };

            fetchClientDetails();
        }
    });

    function isConsentUrl(consent?: string) {
        return !!consent && consent !== 'local';
    }

    async function fetchClientDetails() {
        let res = await fetchGet<ClientResponse>(`/auth/v1/clients/${client.id}`);
        if (res.body) {
            let resp = res.body;
            // The only real difference here can be the `client.scim`, as all the rest is already included in the
            // GET all endpoint
            scimEnabled = !!resp.scim;
            if (resp.scim) {
                scim = {
                    base_uri: resp.scim.base_uri,
                    bearer_token: resp.scim.bearer_token,
                    sync_groups: resp.scim.sync_groups,
                    group_sync_prefix: resp.scim.group_sync_prefix || '',
                };
            }
        } else {
            console.error(res.error);
        }
    }

    async function onSubmit(form: HTMLFormElement) {
        if (redirectURIsErr || postLogoutRedirectURIsErr) {
            section = 1;
            err = redirectURIsErr || postLogoutRedirectURIsErr;
            return;
        }
        err = '';

        let payload: UpdateClientRequest = {
            name: name || undefined,
            enabled,
            confidential,
            allowed_origins: origins.length > 0 ? origins : undefined,
            redirect_uris: redirectURIs,
            post_logout_redirect_uris:
                postLogoutRedirectURIs.length > 0 ? postLogoutRedirectURIs : undefined,

            flows_enabled: [],
            access_token_alg: accessTokenAlg,
            id_token_alg: idTokenAlg,
            access_token_lifetime: Number.parseInt(tokenLifetime),
            auth_code_lifetime: Number.parseInt(authCodeLifetime),

            scopes: scopes.filter(s => s.selected).map(s => s.name),
            default_scopes: defaultScopes.filter(s => s.selected).map(s => s.name),
            challenges: undefined,

            force_mfa: forceMfa,
            client_uri: uri || undefined,
            contacts: contacts.length > 0 ? contacts : undefined,
            backchannel_logout_uri: backchannel_logout_uri || undefined,
            restrict_group_prefix: restrict_group_prefix || undefined,
            claims: parseJsonValue(jsonClaims),
            claims_at_root: claimsAtRoot,
            allowed_resources: allowedResources.length > 0 ? allowedResources : undefined,
            default_aud: defaultAud.length > 0 ? defaultAud : undefined,
            consent: consentMode === consentUrl ? consentSourceUrl.trim() || undefined : undefined,
            actor_same_client: actorSameClient,
        };

        if (flows.authorizationCode) {
            payload.flows_enabled.push('authorization_code');
        }
        if (flows.clientCredentials) {
            payload.flows_enabled.push('client_credentials');
        }
        if (flows.password) {
            payload.flows_enabled.push('password');
        }
        if (flows.refreshToken) {
            payload.flows_enabled.push('refresh_token');
        }
        if (flows.deviceCode) {
            payload.flows_enabled.push(AuthFlowDeviceCode);
        }
        if (flows.tokenExchange) {
            payload.flows_enabled.push(AuthFlowTokenExchange);
        }

        if (challenges.plain) {
            payload.challenges = ['plain'];
        }
        if (challenges.s256) {
            if (payload.challenges) {
                payload.challenges.push('S256');
            } else {
                payload.challenges = ['S256'];
            }
        }

        if (scimEnabled) {
            payload.scim = {
                base_uri: scim.base_uri.trim(),
                bearer_token: scim.bearer_token.trim(),
                sync_groups: scim.sync_groups,
                group_sync_prefix:
                    scim.sync_groups && scim.group_sync_prefix?.length
                        ? scim.group_sync_prefix
                        : undefined,
            };
        }

        let res = await fetchPut(form.action, payload);
        if (res.error) {
            err = res.error.message;
        } else {
            success = true;
            onSave();
            setTimeout(() => {
                success = false;
            }, 2000);
        }
    }

    // Keep all settings in one form, and reveal the first invalid field before focusing it.
    async function showInvalidSection(event: Event) {
        const field = event.target;
        if (!(field instanceof HTMLInputElement || field instanceof HTMLTextAreaElement)) {
            return;
        }
        const form = field.form;
        if (form?.querySelector('input:invalid, textarea:invalid, select:invalid') !== field) {
            return;
        }
        const panel = field.closest<HTMLElement>('[data-client-section]');
        if (panel?.hidden) {
            section = Number(panel.dataset.clientSection);
            await tick();
            field.focus();
            field.reportValidity();
        }
    }
</script>

<div class="container admin-client-config">
    <Form action={`/auth/v1/clients/${client.id}`} {onSubmit} onInvalid={showInvalidSection}>
        <section data-client-section="0" hidden={section !== 0} class="settings-cards">
            <Section title={ta.common.information}>
                <div class="client-identity">
                    <LabeledValue label="ID" mono copyToClip={client.id}>
                        {client.id}
                    </LabeledValue>
                    <Input
                        bind:value={name}
                        autocomplete="off"
                        label={ta.clients.name}
                        placeholder={ta.clients.name}
                        description={ta.clients.descName}
                        width={inputWidth}
                        pattern={PATTERN_CLIENT_NAME}
                    />
                </div>
                <div class="ui-field-grid field-grid">
                    <Input
                        typ="url"
                        bind:value={uri}
                        autocomplete="off"
                        label="URI"
                        placeholder="URI"
                        width={inputWidth}
                        pattern={PATTERN_CLIENT_URI}
                    />
                    <InputTags
                        bind:values={contacts}
                        label={ta.common.contact}
                        pattern={PATTERN_CONTACT}
                    />
                    <p class="settings-hint">{ta.clients.descClientUri}</p>
                </div>
            </Section>

            <Section title={ta.clients.config}>
                <div class="client-flags">
                    <InputCheckbox ariaLabel={ta.common.enabled} bind:checked={enabled}>
                        {ta.common.enabled}
                    </InputCheckbox>
                    <InputCheckbox ariaLabel={ta.clients.forceMfa} bind:checked={forceMfa}>
                        {ta.clients.forceMfa}
                    </InputCheckbox>
                </div>
                <Input
                    bind:value={restrict_group_prefix}
                    autocomplete="off"
                    label={ta.clients.groupLoginPrefix}
                    placeholder={ta.clients.groupLoginPrefix}
                    description={ta.clients.descGroupPrefix}
                    width={inputWidth}
                    pattern={PATTERN_GROUP}
                />
            </Section>
        </section>

        <section data-client-section="1" hidden={section !== 1} class="admin-form-section">
            <p class="mb-0"><b>Authentication Flows</b></p>
            <InputCheckbox ariaLabel="authorization_code" bind:checked={flows.authorizationCode}>
                authorization_code
            </InputCheckbox>
            <InputCheckbox
                ariaLabel="urn:ietf:params:oauth:grant-type:device_code"
                bind:checked={flows.deviceCode}
            >
                device_code
            </InputCheckbox>
            <InputCheckbox ariaLabel="client_credentials" bind:checked={flows.clientCredentials}>
                client_credentials
            </InputCheckbox>
            <InputCheckbox ariaLabel="password" bind:checked={flows.password}
                >password</InputCheckbox
            >
            {#if forceMfa && flows.password}
                <div transition:slide={{ duration: 150 }} class="passwordMfaWarn">
                    {ta.clients.passwordFlowMfaWarn}
                </div>
            {/if}
            <InputCheckbox ariaLabel="refresh_token" bind:checked={flows.refreshToken}>
                refresh_token
            </InputCheckbox>
            <InputCheckbox
                ariaLabel="urn:ietf:params:oauth:grant-type:token-exchange"
                bind:checked={flows.tokenExchange}
            >
                token_exchange
            </InputCheckbox>

            <div style:height=".5rem"></div>
            {#if flows.tokenExchange}
                <div transition:slide={{ duration: 150 }} class="tokenExchangeOpts">
                    <InputCheckbox
                        ariaLabel={ta.clients.actorSameClient}
                        bind:checked={actorSameClient}
                    >
                        {ta.clients.actorSameClient}
                    </InputCheckbox>
                    <p class="desc">{ta.clients.descActorSameClient}</p>
                </div>
            {/if}

            <p class="mb-0"><b>PKCE</b></p>
            <p class="desc">{ta.clients.descPKCE}</p>
            <p class="desc"><strong>{ta.clients.descPKCEEnforce}</strong></p>
            <InputCheckbox ariaLabel="PKCE plain" bind:checked={challenges.plain}
                >plain</InputCheckbox
            >
            <InputCheckbox ariaLabel="PKCE S256" bind:checked={challenges.s256}>S256</InputCheckbox>

            {#if !confidential && !challenges.plain && !challenges.s256}
                <div class="err" transition:slide={{ duration: 150 }}>
                    {ta.clients.errConfidentialPKCE}
                </div>
            {/if}
            <div style:height=".5rem"></div>

            <p class="mb-0"><b>Origin</b></p>
            <p class="desc">{ta.clients.descOrigin}</p>
            <InputTags
                bind:values={origins}
                label="Allowed Origins"
                errMsg={ta.validation.origin}
                pattern={PATTERN_ORIGIN}
            />

            <p class="desc">{@html ta.clients.descUri}</p>
            <InputTags
                bind:values={redirectURIs}
                label="Redirect URIs"
                errMsg={ta.validation.uri}
                required={flows.authorizationCode}
                pattern={PATTERN_CLIENT_URI}
                validate={uri => redirectUriShapeErrorMsg(uri, ta.validation.redirectUri)}
            />
            {#if redirectURIsErr}
                <div class="err" transition:slide={{ duration: 150 }}>
                    {redirectURIsErr}
                </div>
            {/if}
            <InputTags
                bind:values={postLogoutRedirectURIs}
                label="Post Logout Redirect URIs"
                errMsg={ta.validation.uri}
                pattern={PATTERN_CLIENT_URI}
                validate={uri => postLogoutRedirectUriShapeErrorMsg(uri, ta.validation.redirectUri)}
            />
            {#if postLogoutRedirectURIsErr}
                <div class="err" transition:slide={{ duration: 150 }}>
                    {postLogoutRedirectURIsErr}
                </div>
            {/if}

            <div style:height=".5rem"></div>
            <p class="mb-0"><b>Consent</b></p>
            <p class="desc">{ta.clients.consent.desc}</p>
            <LabeledValue label={ta.clients.consent.source}>
                <Options
                    ariaLabel={ta.clients.consent.source}
                    options={consentOptions}
                    bind:value={consentMode}
                    borderless
                />
            </LabeledValue>
            {#if consentMode === consentUrl}
                <div transition:slide={{ duration: 150 }}>
                    <Input
                        typ="url"
                        bind:value={consentSourceUrl}
                        autocomplete="off"
                        label={ta.clients.consent.source}
                        placeholder="https://"
                        width={inputWidth}
                        maxLength={1024}
                        pattern={PATTERN_CONSENT_URL}
                        required
                    />
                </div>
            {/if}

            <div style:height=".5rem"></div>
            <InputCheckbox ariaLabel={ta.clients.confidential} bind:checked={confidential}>
                {ta.clients.confidential}
            </InputCheckbox>
            {#if section === 1}
                {#key client.id}
                    <ClientSecret {client} />
                {/key}
            {/if}
        </section>

        <section data-client-section="2" hidden={section !== 2} class="admin-form-section">
            <p class="mb-0"><b>Resource Indicators</b></p>
            <p class="desc">{ta.clients.descAllowedResources}</p>
            <InputTags
                bind:values={allowedResources}
                label={ta.clients.allowedResources}
                errMsg={ta.validation.uri}
                pattern={PATTERN_RESOURCE}
            />
            <p class="desc">{ta.clients.descDefaultAud}</p>
            <InputTags
                bind:values={defaultAud}
                label={ta.clients.defaultAud}
                errMsg={ta.validation.uri}
                pattern={PATTERN_RESOURCE}
            />

            <div style:height=".5rem"></div>
            <p class="mb-0"><b>Scopes</b></p>
            <p class="desc">{@html ta.clients.scopes.desc}</p>
            <SelectList bind:items={scopes}>
                {ta.clients.scopes.allowed}
            </SelectList>
            <SelectList bind:items={defaultScopes}>
                {ta.clients.scopes.default}
            </SelectList>

            <div style:height=".75rem"></div>
            <p class="mb-0"><b>Tokens</b></p>
            <p>{ta.clients.tokenLifetime.p1}</p>
            <Input
                typ="number"
                bind:value={tokenLifetime}
                autocomplete="off"
                label="Token Lifetime"
                placeholder="Token Lifetime"
                width={inputWidth}
                min="10"
                max="86400"
                errMsg="10 <= Token Lifetime <= 86400"
            />

            <div style:height=".5rem"></div>
            <p>{ta.clients.tokenLifetime.p2}</p>
            <p>{ta.clients.tokenLifetime.p3}</p>
            <LabeledValue label="Access Token Algorithm">
                <Options
                    ariaLabel="Access Token Algorithm"
                    options={optionsAlgs}
                    bind:value={accessTokenAlg}
                    borderless
                />
            </LabeledValue>
            <LabeledValue label="ID Token Algorithm">
                <Options
                    ariaLabel="ID Token Algorithm"
                    options={optionsAlgs}
                    bind:value={idTokenAlg}
                    borderless
                />
            </LabeledValue>

            <div style:height=".5rem"></div>
            <p>{ta.clients.descAuthCode}</p>
            <Input
                typ="number"
                bind:value={authCodeLifetime}
                autocomplete="off"
                label="Auth Code Lifetime"
                placeholder="Auth Code Lifetime"
                width={inputWidth}
                min="10"
                max="300"
                errMsg="10 <= Auth Code Lifetime <= 300"
            />

            <div style:height=".5rem"></div>
            <p class="mb-0"><b>Custom Claims</b></p>
            <p class="desc">{ta.clients.claimsDesc}</p>
            <div class="claims">
                <InputArea
                    label={ta.clients.claims}
                    placeholder={ta.clients.claims}
                    rows={10}
                    bind:value={jsonClaims}
                    formatBadge={t.account.stringOrJson ?? 'String / JSON'}
                />
            </div>
            <InputCheckbox ariaLabel={ta.clients.claimsAtRoot} bind:checked={claimsAtRoot}>
                {ta.clients.claimsAtRoot}
            </InputCheckbox>
            {#if claimsAtRoot}
                <p class="warn">
                    {ta.clients.claimsAtRootWarning}
                    <a
                        href="https://www.iana.org/assignments/jwt/jwt.xhtml"
                        target="_blank"
                        rel="noopener noreferrer"
                    >
                        IANA JWT Claims Registry
                    </a>
                </p>
            {/if}
        </section>

        <section data-client-section="3" hidden={section !== 3} class="settings-cards">
            <Section title="Backchannel Logout">
                <p class="settings-hint">
                    {@html ta.clients.backchannelLogout.replace(
                        '{{ OIDC_BCL }}',
                        '<a href="https://openid.net/specs/openid-connect-backchannel-1_0.html" target="_blank" rel="noopener noreferrer">OpenID Connect Back-Channel Logout</a>',
                    )}
                </p>
                <Input
                    typ="url"
                    bind:value={backchannel_logout_uri}
                    autocomplete="off"
                    label="Backchannel Logout URI"
                    placeholder="Backchannel Logout URI"
                    width={inputWidth}
                    pattern={PATTERN_URI}
                    disabled={client.id === 'rauthy'}
                />
            </Section>

            <Section title="SCIM v2">
                {#snippet actions()}
                    <InputCheckbox
                        ariaLabel={ta.clients.scim.enable}
                        disabled={client.id === 'rauthy'}
                        bind:checked={scimEnabled}
                    >
                        {ta.clients.scim.enable}
                    </InputCheckbox>
                {/snippet}

                <p class="settings-hint">
                    {@html ta.clients.scim.desc.replace(
                        '{{ SCIM_LINK }}',
                        '<a href="https://www.rfc-editor.org/rfc/rfc7644" target="_blank" rel="noopener noreferrer">SCIM v2</a>',
                    )}
                </p>

                {#if scimEnabled}
                    <div class="ui-field-grid field-grid">
                        <Input
                            typ="url"
                            bind:value={scim.base_uri}
                            autocomplete="off"
                            label="SCIM Base URI"
                            placeholder="SCIM Base URI"
                            width={inputWidth}
                            pattern={PATTERN_CLIENT_URI}
                            required={scimEnabled}
                        />
                        <InputPassword
                            bind:value={scim.bearer_token}
                            autocomplete="off"
                            label="Bearer Token"
                            placeholder="Bearer Token"
                            width={inputWidth}
                            pattern={PATTERN_URI}
                            maxLength={2048}
                            required={scimEnabled}
                        />
                        <p class="settings-hint">
                            {@html ta.clients.scim.baseUri}
                        </p>
                    </div>

                    <div class="scim-groups">
                        <InputCheckbox
                            ariaLabel={ta.clients.scim.groupSync}
                            disabled={client.id === 'rauthy'}
                            bind:checked={scim.sync_groups}
                        >
                            {ta.clients.scim.groupSync}
                        </InputCheckbox>
                        {#if scim.sync_groups}
                            <Input
                                bind:value={scim.group_sync_prefix}
                                autocomplete="off"
                                label={ta.clients.scim.groupSyncPrefix}
                                placeholder={ta.clients.scim.groupSyncPrefix}
                                width={inputWidth}
                                pattern={PATTERN_GROUP}
                            />
                            <p class="settings-hint">
                                {@html ta.clients.scim.groupSyncPrefixDesc}
                            </p>
                        {/if}
                    </div>

                    <details class="scim-compatibility">
                        <summary>{ta.clients.scim.reqDesc}</summary>
                        <ul>
                            <li>{@html ta.clients.scim.reqLi1}</li>
                            <li>{@html ta.clients.scim.reqLi2}</li>
                            <li>{@html ta.clients.scim.reqLi3}</li>
                        </ul>
                    </details>
                {/if}
            </Section>
        </section>
        <div class="flex gap-05 admin-save-bar">
            <Button type="submit">
                {t.common.save}
            </Button>

            {#if success}
                <IconCheck />
            {/if}
        </div>

        {#if err}
            <div class="err">
                {err}
            </div>
        {/if}

        <div style:height="1rem"></div>
    </Form>
</div>

<style>
    [hidden] {
        display: none;
    }

    .desc {
        margin-bottom: -0.25rem;
    }

    .container {
        /* matches <p> max width */
        max-width: 467pt;
    }

    .mb-0 {
        margin-bottom: 0;
    }

    .claims {
        max-width: 40rem;
    }

    .passwordMfaWarn {
        color: hsl(var(--error));
        margin-left: 1.5rem;
    }

    .tokenExchangeOpts {
        margin-left: 1.5rem;
    }

    .warn {
        max-width: 40rem;
        color: hsl(var(--error));
        font-size: 0.9rem;
        opacity: 0.8;
    }

    .settings-cards {
        min-width: 0;
        --ui-field-margin: 0;
    }

    .settings-cards:not([hidden]) {
        display: grid;
        gap: var(--ui-space);
    }

    .client-identity,
    .scim-groups {
        display: grid;
        gap: 0.75rem;
    }

    .client-identity :global(.admin-labeled-value) {
        margin: 0;
    }

    .field-grid {
        row-gap: 0.5rem;
    }

    .settings-cards .field-grid :global(.admin-password-field),
    .settings-cards .field-grid :global(.admin-tags-field) {
        margin-block: 0;
    }

    .settings-cards .field-grid :global(.admin-tags-field > label) {
        margin-bottom: var(--ui-label-gap);
        font-size: 0.8125rem;
        font-weight: 500;
        line-height: var(--ui-label-height);
        color: var(--ui-muted);
    }

    .settings-cards .field-grid :global(.admin-tags-field > ul) {
        margin: 0;
    }

    .settings-hint {
        margin: 0;
        color: var(--ui-muted);
        overflow-wrap: anywhere;
    }

    .field-grid > .settings-hint {
        grid-column: 1 / -1;
    }

    .client-flags {
        display: flex;
        flex-wrap: wrap;
        gap: 0.5rem var(--ui-space);
    }

    .scim-groups {
        padding-top: var(--ui-space);
        border-top: 1px solid var(--ui-outline);
    }

    .scim-compatibility {
        padding: 0.625rem 0.75rem;
        border-radius: var(--border-radius);
        background: var(--ui-container);
        color: var(--ui-muted);
        font-size: var(--ui-control-font-size);
        overflow-wrap: anywhere;
    }

    .scim-compatibility summary {
        color: var(--ui-text);
        cursor: pointer;
    }

    .scim-compatibility ul {
        display: grid;
        gap: 0.375rem;
        margin: 0.5rem 0 0;
        padding-inline-start: 1.25rem;
    }

    .admin-form-section {
        padding: 24px;
        border: 1px solid var(--ui-outline);
        border-radius: var(--border-radius);
        min-width: 0;
        background: transparent;
    }
    .admin-form-section :global(h3) {
        margin: 0 0 24px;
        color: var(--ui-text);
        font-size: 16px;
        font-weight: 600;
        line-height: 24px;
    }
    .admin-client-config > :global(form) > :global(section) {
        margin: 20px 0;
    }
    .admin-client-config :global(.desc) {
        margin: 16px 0 8px;
    }
    .admin-save-bar {
        display: flex;
        align-items: center;
        gap: 12px;
        position: sticky;
        bottom: -32px;
        z-index: 2;
        margin: 24px -1px -32px;
        padding: 16px 1px;
        background: var(--ui-surface);
        border-top: 1px solid var(--ui-outline);
    }
    @media (max-width: 800px) {
        .admin-form-section {
            padding: 20px;
        }
        .admin-save-bar {
            bottom: -24px;
            margin-bottom: -24px;
        }
    }

    .admin-client-config {
        max-width: none;
    }
</style>
