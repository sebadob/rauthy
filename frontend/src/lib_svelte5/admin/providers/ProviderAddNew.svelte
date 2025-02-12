<script lang="ts">
    import {onMount, untrack} from "svelte";
    import Button from "$lib5/button/Button.svelte";
    import Input from "$lib5/form/Input.svelte";
    import Switch from "$lib/Switch.svelte";
    import CheckIcon from "$lib/CheckIcon.svelte";
    import JsonPathDesc from "$lib5/admin/providers/JsonPathDesc.svelte";
    import {fetchPost} from "$api/fetch.ts";
    import {useI18nAdmin} from "$state/i18n_admin.svelte.ts";
    import Options from "$lib5/Options.svelte";
    import InputCheckbox from "$lib5/form/InputCheckbox.svelte";
    import InputArea from "$lib5/form/InputArea.svelte";
    import LabeledValue from "$lib5/LabeledValue.svelte";
    import IconCheck from "$icons/IconCheck.svelte";
    import type {
        AuthProviderType,
        ProviderLookupRequest,
        ProviderLookupResponse,
        ProviderRequest
    } from "$api/types/auth_provider.ts";
    import InputPassword from "$lib5/form/InputPassword.svelte";
    import {useI18n} from "$state/i18n.svelte.ts";
    import {slide} from "svelte/transition";
    import {PATTERN_CLIENT_NAME, PATTERN_PEM, PATTERN_SCOPE_SPACE, PATTERN_URI} from "$utils/patterns.ts";
    import Form from "$lib5/form/Form.svelte";

    let {onSave}: { onSave: () => void } = $props();

    let t = useI18n();
    let ta = useI18nAdmin();

    const inputWidth = 'min(calc(100dvw - 1.75rem), 30rem)';

    let isLoading = $state(false);
    let err = $state('');
    let lookupSuccess = $state(false);
    let success = $state(false);

    let showRootPem = $state(false);

    let configLookup: ProviderLookupRequest = $state({
        issuer: '',
        metadata_url: '',
        danger_allow_insecure: false,
        root_pem: '',
    });
    let config: ProviderRequest = $state({
        enabled: true,
        typ: 'oidc',

        // fixed values after lookup
        issuer: '',
        danger_allow_insecure: false,
        authorization_endpoint: '',
        token_endpoint: '',
        token_auth_method_basic: false,
        userinfo_endpoint: '',
        use_pkce: true,
        client_secret_basic: true,
        client_secret_post: false,

        // user defined values
        name: '',
        client_id: '',
        client_secret: '',
        scope: '',
        root_pem: '',

        admin_claim_path: '',
        admin_claim_value: '',
        mfa_claim_path: '',
        mfa_claim_value: '',
        // maybe additional ones in the future like client_logo
    })

    let modes: string[] = ['OIDC', 'Auto', 'Custom', 'Github', 'Google'];
    let mode = $state(modes[0]);
    let isAuto = $derived(mode === 'Auto');
    let isCustom = $derived(mode === 'Custom');
    let isOidc = $derived(mode === 'OIDC');
    let isSpecial = $derived(!isAuto && !isCustom && !isOidc);

    let formActionLookup = $derived(!(isSpecial || isCustom || lookupSuccess));
    const urlLookup = '/auth/v1/providers/lookup';
    let formAction = $derived(formActionLookup ? urlLookup : '/auth/v1/providers/create');

    $effect(() => {
        if (mode) {
            // reset values
            lookupSuccess = false;
            configLookup = {
                issuer: '',
                metadata_url: '',
                danger_allow_insecure: false,
                root_pem: '',
            };

            switch (mode) {
                case 'Github':
                    // Github does not implement metadata lookup -> configure manually
                    config = {
                        enabled: true,

                        // fixed values after lookup
                        issuer: 'github.com',
                        typ: 'github',
                        danger_allow_insecure: false,
                        authorization_endpoint: 'https://github.com/login/oauth/authorize',
                        token_endpoint: 'https://github.com/login/oauth/access_token',
                        client_secret_basic: true,
                        client_secret_post: true,
                        userinfo_endpoint: 'https://api.github.com/user',
                        use_pkce: false,

                        // user defined values
                        name: 'Github',
                        client_id: '',
                        client_secret: '',
                        scope: 'user:email',
                        root_pem: '',

                        admin_claim_path: '',
                        admin_claim_value: '',
                        mfa_claim_path: '$.two_factor_authentication',
                        mfa_claim_value: 'true',
                        // maybe additional ones in the future like client_logo
                    };
                    break;
                case 'Google':
                    // Google supports oidc metadata lookup
                    configLookup = {
                        issuer: 'accounts.google.com',
                        metadata_url: '',
                        danger_allow_insecure: false,
                        root_pem: '',
                    }
                    untrack(() => {
                        onSubmitLookup(urlLookup);
                    });
                    break;
                default:
                    config = {
                        enabled: true,

                        // fixed values after lookup
                        issuer: '',
                        typ: 'oidc',
                        danger_allow_insecure: false,
                        authorization_endpoint: '',
                        token_endpoint: '',
                        client_secret_basic: true,
                        client_secret_post: false,
                        userinfo_endpoint: '',
                        use_pkce: true,

                        // user defined values
                        name: '',
                        client_id: '',
                        client_secret: '',
                        scope: '',
                        root_pem: '',

                        admin_claim_path: '',
                        admin_claim_value: '',
                        mfa_claim_path: '',
                        mfa_claim_value: '',
                        // maybe additional ones in the future like client_logo
                    };
            }
        }
    });

    $effect(() => {
        if (success) {
            setTimeout(() => {
                onSave();
                resetValues();
            }, 1500);
        }
    });

    async function onSubmit(form: HTMLFormElement, params: URLSearchParams) {
        if (formActionLookup) {
            await onSubmitLookup(form.action);
        } else {
            await onSubmitConfig(form.action);
        }
    }

    async function onSubmitConfig(url: string) {
        if (config.client_secret && !(config.client_secret_basic || config.client_secret_post)) {
            err = ta.providers.config.errNoAuthMethod;
            return;
        }
        if (!config.use_pkce && !config.client_secret) {
            err = ta.providers.config.errConfidential;
            return;
        }

        err = '';
        isLoading = true;

        let payload: ProviderRequest = {
            name: config.name,
            typ: isAuto ? 'custom' : mode.toLowerCase() as AuthProviderType,
            enabled: config.enabled,

            issuer: config.issuer,
            authorization_endpoint: config.authorization_endpoint,
            token_endpoint: config.token_endpoint,
            userinfo_endpoint: config.userinfo_endpoint,

            danger_allow_insecure: showRootPem && config.root_pem ? false : config.danger_allow_insecure,
            use_pkce: config.use_pkce,
            client_secret_basic: config.client_secret_basic,
            client_secret_post: config.client_secret_post,

            client_id: config.client_id,
            client_secret: config.client_secret,
            scope: config.scope.trim(),
            root_pem: showRootPem && config.root_pem ? config.root_pem.trim() : undefined,

            admin_claim_path: config.admin_claim_path || undefined,
            admin_claim_value: config.admin_claim_value || undefined,
            mfa_claim_path: config.mfa_claim_path || undefined,
            mfa_claim_value: config.mfa_claim_value || undefined,
        };
        let res = await fetchPost(url, payload);
        if (res.error) {
            if (res.error.message.includes('InvalidCertificate')) {
                err = 'Insecure connection not allowed';
            } else {
                err = res.error.message;
            }
        } else {
            success = true;
        }

        isLoading = false;
    }

    async function onSubmitLookup(url: string) {
        if (!configLookup.issuer && !configLookup.metadata_url) {
            err = 'Provide at least one of Issuer / Metadata URL';
            return;
        }

        err = '';
        isLoading = true;

        let payload: ProviderLookupRequest = {
            issuer: configLookup.issuer || undefined,
            metadata_url: configLookup.metadata_url || undefined,
            danger_allow_insecure: configLookup.danger_allow_insecure,
            root_pem: configLookup.root_pem || undefined,
        }
        let res = await fetchPost<ProviderLookupResponse>(url, payload);
        if (res.body) {
            config.issuer = res.body.issuer;
            config.authorization_endpoint = res.body.authorization_endpoint;
            config.danger_allow_insecure = res.body.danger_allow_insecure;
            config.token_endpoint = res.body.token_endpoint;
            config.userinfo_endpoint = res.body.userinfo_endpoint;
            config.client_secret_basic = res.body.client_secret_basic;
            config.client_secret_post = res.body.client_secret_post;
            config.use_pkce = res.body.use_pkce;
            config.client_secret_basic = res.body.client_secret_basic;
            // we want to res.enable basic only if it is supported for better compatibility out of the box
            config.client_secret_post = !res.body.client_secret_basic && res.body.client_secret_post;
            config.scope = res.body.scope;
            config.root_pem = res.body.root_pem;

            lookupSuccess = true;
        } else if (res.error) {
            if (res.error.message.includes('InvalidCertificate')) {
                err = 'Insecure connection not allowed';
            } else {
                err = res.error.message;
            }
        }

        isLoading = false;
    }

    function resetValues() {
        configLookup = {
            issuer: '',
            metadata_url: '',
            danger_allow_insecure: false,
            root_pem: '',
        };
        config = {
            enabled: true,
            issuer: '',
            typ: 'oidc',
            danger_allow_insecure: false,
            authorization_endpoint: '',
            token_endpoint: '',
            userinfo_endpoint: '',
            use_pkce: true,
            name: '',
            client_id: '',
            client_secret_basic: true,
            client_secret_post: false,
            scope: '',
            root_pem: '',
            admin_claim_path: '',
            admin_claim_value: '',
            mfa_claim_path: '',
            mfa_claim_value: '',
        }
        success = false;
        lookupSuccess = false;
        showRootPem = false;
    }
</script>

<div class="container">
    <Options ariaLabel="Select Mode" options={modes} bind:value={mode}/>
    <div style:height=".5rem"></div>

    <Form action={formAction} {onSubmit}>
        {#if !lookupSuccess}
            <div class="checkbox">
                <InputCheckbox ariaLabel={ta.providers.config.custRootCa} bind:checked={showRootPem}>
                    {ta.providers.config.custRootCa}
                </InputCheckbox>
            </div>

            {#if showRootPem}
                <div transition:slide={{duration: 150}}>
                    <InputArea
                            rows={15}
                            name="rootPem"
                            label={ta.providers.config.rootPemCert}
                            placeholder="-----BEGIN CERTIFICATE-----
...
-----END CERTIFICATE-----"
                            bind:value={configLookup.root_pem}
                            errMsg="-----BEGIN CERTIFICATE----- ..."
                            width="min(40rem, calc(100dvw - 1.75rem))"
                            fontMono
                            pattern={PATTERN_PEM}
                    />
                </div>
            {:else}
                <div class="checkbox">
                    <InputCheckbox
                            ariaLabel={ta.providers.config.allowInsecureTls}
                            bind:checked={configLookup.danger_allow_insecure}
                    >
                        {ta.providers.config.allowInsecureTls}
                    </InputCheckbox>
                </div>
            {/if}
        {/if}

        {#if isOidc && !lookupSuccess}
            <Input
                    name="issuer"
                    label="Issuer URL"
                    placeholder="Issuer URL"
                    bind:value={configLookup.issuer}
                    width={inputWidth}
                    required
                    pattern={PATTERN_URI}
            />

            <!--            <Button type="submit" {isLoading}>-->
            <!--                {ta.providers.config.lookup}-->
            <!--            </Button>-->
        {:else if isAuto && !lookupSuccess}
            <Input
                    typ="url"
                    name="metadata"
                    bind:value={configLookup.metadata_url}
                    label="Metadata URL"
                    placeholder=".../.well-known/openid-configuration"
                    width={inputWidth}
                    required
                    pattern={PATTERN_URI}
            />

            <!--            <Button type="submit" {isLoading}>-->
            <!--                {ta.providers.config.lookup}-->
            <!--            </Button>-->
        {:else if isSpecial || isCustom || lookupSuccess}
            {#if showRootPem}
                <InputArea
                        rows={17}
                        name="rootPem"
                        label={ta.providers.config.rootPemCert}
                        placeholder="-----BEGIN CERTIFICATE-----
...
-----END CERTIFICATE-----"
                        bind:value={config.root_pem}
                        disabled={lookupSuccess}
                        errMsg="-----BEGIN CERTIFICATE----- ..."
                        width="min(40rem, calc(100dvw - 1.75rem))"
                        fontMono
                        pattern={PATTERN_PEM}
                />
            {:else}
                <div class="ml mb">
                    {#if lookupSuccess}
                        <LabeledValue label={ta.providers.config.allowInsecureTls}>
                            <CheckIcon check={config.danger_allow_insecure}/>
                        </LabeledValue>
                    {:else}
                        <InputCheckbox
                                ariaLabel={ta.providers.config.allowInsecureTls}
                                bind:checked={config.danger_allow_insecure}
                        >
                            {ta.providers.config.allowInsecureTls}
                        </InputCheckbox>
                    {/if}
                </div>
            {/if}

            <Input
                    name="issuer"
                    bind:value={config.issuer}
                    label="Issuer URL"
                    placeholder="Issuer URL"
                    width={inputWidth}
                    disabled={lookupSuccess}
                    required
                    pattern={PATTERN_URI}
            />
            <Input
                    typ="url"
                    name="auth_endpoint"
                    bind:value={config.authorization_endpoint}
                    label="Authorization Endpoint"
                    placeholder="Authorization Endpoint"
                    width={inputWidth}
                    disabled={lookupSuccess}
                    required
                    pattern={PATTERN_URI}
            />
            <Input
                    typ="url"
                    name="token_endpoint"
                    bind:value={config.token_endpoint}
                    label="Token Endpoint"
                    placeholder="Token Endpoint"
                    width={inputWidth}
                    disabled={lookupSuccess}
                    required
                    pattern={PATTERN_URI}
            />
            <Input
                    typ="url"
                    name="userinfo_endpoint"
                    bind:value={config.userinfo_endpoint}
                    label="Userinfo Endpoint"
                    placeholder="Userinfo Endpoint"
                    width={inputWidth}
                    disabled={lookupSuccess}
                    required
                    pattern={PATTERN_URI}
            />

            <div class="checkbox">
                {#if lookupSuccess}
                    <LabeledValue label="PKCE">
                        <CheckIcon check={config.use_pkce}/>
                    </LabeledValue>
                {:else}
                    <InputCheckbox
                            ariaLabel="PKCE"
                            bind:checked={config.use_pkce}
                    >
                        PKCE
                    </InputCheckbox>
                {/if}
            </div>

            <p class="desc">{ta.providers.config.descScope}</p>
            <Input
                    name="scope"
                    bind:value={config.scope}
                    label="Scope"
                    placeholder="openid profile email"
                    width={inputWidth}
                    required
                    pattern={PATTERN_SCOPE_SPACE}
            />

            <p class="desc">{ta.providers.config.descClientName}</p>
            <Input
                    name="client_name"
                    bind:value={config.name}
                    label="Client Name"
                    placeholder="Client Name"
                    width={inputWidth}
                    required
                    pattern={PATTERN_CLIENT_NAME}
            />

            <p class="desc">{ta.providers.config.descClientId}</p>
            <Input
                    name="client_id"
                    bind:value={config.client_id}
                    autocomplete="off"
                    label="Client ID"
                    placeholder="Client ID"
                    width={inputWidth}
                    required
                    pattern={PATTERN_URI}
            />

            <p class="desc">{ta.providers.config.descClientId}</p>
            <InputPassword
                    name="client_secret"
                    bind:value={config.client_secret}
                    autocomplete="off"
                    label="Client Secret"
                    placeholder="Client Secret"
                    width={inputWidth}
                    errMsg={ta.providers.config.errConfidential}
                    maxLength={256}
                    required={!config.use_pkce}
            />

            <p>{@html ta.providers.config.descAuthMethod}</p>
            <div class="checkbox">
                <InputCheckbox ariaLabel="client_secret_basic" bind:checked={config.client_secret_basic}>
                    client_secret_basic
                </InputCheckbox>
            </div>
            <div class="checkbox">
                <InputCheckbox ariaLabel="client_secret_post" bind:checked={config.client_secret_post}>
                    client_secret_post
                </InputCheckbox>
            </div>
            {#if !config.use_pkce && !config.client_secret_basic && !config.client_secret_post}
                <div class="err" transition:slide={{duration: 150}}>
                    {ta.providers.config.errNoAuthMethod}
                </div>
            {/if}

            <JsonPathDesc/>

            <p class="desc">{ta.providers.config.mapUser}</p>
            <Input
                    name="admin_claim_path"
                    bind:value={config.admin_claim_path}
                    label={ta.providers.config.pathAdminClaim}
                    placeholder="$.roles.*"
                    width={inputWidth}
                    pattern={PATTERN_URI}
            />
            <Input
                    name="admin_claim_value"
                    bind:value={config.admin_claim_value}
                    label={ta.providers.config.valueAdminClaim}
                    placeholder="rauthy_admin"
                    width={inputWidth}
                    pattern={PATTERN_URI}
            />

            <p class="desc">{ta.providers.config.mapMfa}</p>
            <Input
                    name="mfa_claim_path"
                    bind:value={config.mfa_claim_path}
                    label={ta.providers.config.pathMfaClaim}
                    placeholder="$.amr.*"
                    width={inputWidth}
                    pattern={PATTERN_URI}
            />
            <Input
                    name="mfa_claim_value"
                    bind:value={config.mfa_claim_value}
                    label={ta.providers.config.valueMfaClaim}
                    placeholder="mfa"
                    width={inputWidth}
                    pattern={PATTERN_URI}
            />

        {/if}
        <div class="flex gap-05">
            <Button type="submit" {isLoading}>
                {formActionLookup ? ta.providers.config.lookup : t.common.save}
            </Button>

            {#if !formActionLookup}
                <Button level={3} onclick={resetValues} {isLoading}>
                    {ta.common.reset}
                </Button>
            {/if}

            {#if success}
                <IconCheck/>
            {/if}

            {#if err}
                <div class="err">
                    {err}
                </div>
            {/if}
        </div>
    </Form>
</div>

<style>
    .checkbox {
        margin: .25rem 0;
    }

    .container {
        height: 90dvh;
        max-height: 90dvh;
        width: min(40rem, calc(100dvw - 1.75rem));
        overflow-y: auto;
    }

    .desc {
        margin-bottom: -.5rem;
    }
</style>
