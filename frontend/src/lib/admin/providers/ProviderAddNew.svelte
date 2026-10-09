<script lang="ts">
    import { untrack } from 'svelte';
    import Button from '$lib5/button/Button.svelte';
    import Input from '$lib5/form/Input.svelte';
    import CheckIcon from '$lib5/CheckIcon.svelte';
    import { fetchPost } from '$api/fetch';
    import { useI18nAdmin } from '$state/i18n_admin.svelte';
    import Options from '$lib5/Options.svelte';
    import InputCheckbox from '$lib5/form/InputCheckbox.svelte';
    import LabeledValue from '$lib5/LabeledValue.svelte';
    import IconCheck from '$icons/IconCheck.svelte';
    import type {
        AuthProviderType,
        ProviderLookupRequest,
        ProviderLookupResponse,
        ProviderRequest,
    } from '$api/types/auth_provider.ts';
    import { useI18n } from '$state/i18n.svelte';
    import { PATTERN_URI } from '$utils/patterns';
    import Form from '$lib5/form/Form.svelte';
    import ProviderConfigURLs from '$lib/admin/providers/blocks/ProviderConfigURLs.svelte';
    import ProviderConfigClientInfo from '$lib/admin/providers/blocks/ProviderConfigClientInfo.svelte';

    let { onSave }: { onSave: () => void } = $props();

    let t = useI18n();
    let ta = useI18nAdmin();

    const inputWidth = 'min(calc(100dvw - 1.75rem), 30rem)';

    let isLoading = $state(false);
    let err = $state('');
    let lookupSuccess = $state(false);
    let success = $state(false);

    let configLookup: ProviderLookupRequest = $state({
        issuer: '',
        metadata_url: '',
    });
    let config: ProviderRequest = $state({
        enabled: true,
        typ: 'oidc',

        // fixed values after lookup
        issuer: '',
        authorization_endpoint: '',
        token_endpoint: '',
        token_auth_method_basic: false,
        userinfo_endpoint: '',
        use_pkce: true,
        client_secret_basic: true,
        client_secret_post: false,
        auto_onboarding: false,
        auto_link: false,

        // user defined values
        name: '',
        client_id: '',
        client_secret: '',
        scope: '',

        admin_claim_path: '',
        admin_claim_value: '',
        mfa_claim_path: '',
        mfa_claim_value: '',
        // maybe additional ones in the future like client_logo
    });

    let modes: string[] = ['OIDC', 'Auto', 'Custom', 'GitHub', 'Google'];
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
            };

            switch (mode) {
                case 'GitHub':
                    // GitHub does not implement metadata lookup -> configure manually
                    config = {
                        enabled: true,

                        // fixed values after lookup
                        issuer: 'github.com',
                        typ: 'github',
                        authorization_endpoint: 'https://github.com/login/oauth/authorize',
                        token_endpoint: 'https://github.com/login/oauth/access_token',
                        client_secret_basic: true,
                        client_secret_post: true,
                        auto_onboarding: false,
                        auto_link: false,
                        userinfo_endpoint: 'https://api.github.com/user',
                        use_pkce: false,

                        // user defined values
                        name: 'GitHub',
                        client_id: '',
                        client_secret: '',
                        scope: 'user:email',

                        admin_claim_path: '',
                        admin_claim_value: '',
                        mfa_claim_path: '$.two_factor_authentication',
                        mfa_claim_value: 'true',
                        // maybe additional ones in the future like client_logo
                    };
                    lookupSuccess = true;
                    break;
                case 'Google':
                    // Google supports oidc metadata lookup
                    configLookup = {
                        issuer: 'accounts.google.com',
                        metadata_url: '',
                    };
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
                        authorization_endpoint: '',
                        token_endpoint: '',
                        client_secret_basic: true,
                        client_secret_post: false,
                        auto_onboarding: false,
                        auto_link: false,
                        userinfo_endpoint: '',
                        use_pkce: true,

                        // user defined values
                        name: '',
                        client_id: '',
                        client_secret: '',
                        scope: '',

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
        if (config.issuer === 'atproto') {
            err = 'Must not contain a reserved name';
            return;
        }
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
            typ: isAuto ? 'custom' : (mode.toLowerCase() as AuthProviderType),
            enabled: config.enabled,

            issuer: config.issuer,
            authorization_endpoint: config.authorization_endpoint,
            token_endpoint: config.token_endpoint,
            userinfo_endpoint: config.userinfo_endpoint,

            use_pkce: config.use_pkce,
            client_secret_basic: config.client_secret_basic,
            client_secret_post: config.client_secret_post,
            auto_onboarding: config.auto_onboarding,
            auto_link: config.auto_link,

            client_id: config.client_id,
            client_secret: config.client_secret,
            scope: config.scope.trim(),

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
        };
        let res = await fetchPost<ProviderLookupResponse>(url, payload);
        if (res.body) {
            config.issuer = res.body.issuer;
            config.authorization_endpoint = res.body.authorization_endpoint;
            config.token_endpoint = res.body.token_endpoint;
            config.userinfo_endpoint = res.body.userinfo_endpoint;
            config.client_secret_basic = res.body.client_secret_basic;
            config.client_secret_post = res.body.client_secret_post;
            config.use_pkce = res.body.use_pkce;
            config.client_secret_basic = res.body.client_secret_basic;
            // we want to res.enable basic only if it is supported for better compatibility out of the box
            config.client_secret_post =
                !res.body.client_secret_basic && res.body.client_secret_post;
            config.scope = res.body.scope;

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
        };
        config = {
            enabled: true,
            issuer: '',
            typ: 'oidc',
            authorization_endpoint: '',
            token_endpoint: '',
            userinfo_endpoint: '',
            use_pkce: true,
            name: '',
            client_id: '',
            client_secret_basic: true,
            client_secret_post: false,
            auto_onboarding: false,
            auto_link: false,
            scope: '',
            admin_claim_path: '',
            admin_claim_value: '',
            mfa_claim_path: '',
            mfa_claim_value: '',
        };
        success = false;
        lookupSuccess = false;
    }
</script>

<div class="container">
    <Options ariaLabel="Select Mode" options={modes} bind:value={mode} />
    <div style:height=".5rem"></div>

    <Form action={formAction} {onSubmit}>
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
        {:else if isSpecial || isCustom || lookupSuccess}
            <ProviderConfigURLs
                bind:issuer={config.issuer}
                bind:authorizationEndpoint={config.authorization_endpoint}
                bind:tokenEndpoint={config.token_endpoint}
                bind:userinfoEndpoint={config.userinfo_endpoint}
                {inputWidth}
                disabled={lookupSuccess}
            />

            <div class="checkbox">
                {#if lookupSuccess}
                    <LabeledValue label="PKCE">
                        <CheckIcon checked={config.use_pkce} />
                    </LabeledValue>
                {:else}
                    <InputCheckbox ariaLabel="PKCE" bind:checked={config.use_pkce}>
                        PKCE
                    </InputCheckbox>
                {/if}
            </div>

            <ProviderConfigClientInfo
                bind:scope={config.scope}
                bind:name={config.name}
                bind:clientId={config.client_id}
                bind:clientSecret={config.client_secret}
                bind:clientSecretBasic={config.client_secret_basic}
                bind:clientSecretPost={config.client_secret_post}
                bind:adminClaimPath={config.admin_claim_path}
                bind:adminClaimValue={config.admin_claim_value}
                bind:mfaClaimPath={config.mfa_claim_path}
                bind:mfaClaimValue={config.mfa_claim_value}
                usePKCE={config.use_pkce}
                {inputWidth}
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
                <IconCheck />
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
        margin: 0.25rem 0;
    }

    .container {
        height: 90dvh;
        max-height: 90dvh;
        width: min(40rem, calc(100dvw - 1.75rem));
        overflow-y: auto;
    }
</style>
