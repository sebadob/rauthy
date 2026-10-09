<script lang="ts">
    import Button from '$lib5/button/Button.svelte';
    import ProviderLogo from '../../ProviderLogo.svelte';
    import type { ProviderRequest, ProviderResponse } from '$api/types/auth_provider.ts';
    import IconCheck from '$icons/IconCheck.svelte';
    import Form from '$lib5/form/Form.svelte';
    import { fetchDelete, fetchPut } from '$api/fetch';
    import { useI18nAdmin } from '$state/i18n_admin.svelte';
    import { useI18n } from '$state/i18n.svelte';
    import LabeledValue from '$lib5/LabeledValue.svelte';
    import InputCheckbox from '$lib5/form/InputCheckbox.svelte';
    import InputFile from '$lib5/form/InputFile.svelte';
    import { genKey } from '$utils/helpers';
    import ProviderConfigURLs from '$lib/admin/providers/blocks/ProviderConfigURLs.svelte';
    import ProviderConfigClientInfo from '$lib/admin/providers/blocks/ProviderConfigClientInfo.svelte';
    import { slide } from 'svelte/transition';

    let {
        provider = $bindable(),
        onSave = $bindable(),
    }: {
        provider: ProviderResponse;
        onSave: () => void;
    } = $props();

    let t = useI18n();
    let ta = useI18nAdmin();

    const inputWidth = 'min(calc(100dvw - .5rem), 30rem)';

    let isLoading = $state(false);
    let err = $state('');
    let success = $state(false);
    let logoKey = $state(genKey());

    let urlImg = $derived(`/auth/v1/providers/${provider.id}/img`);

    $effect(() => {
        if (provider.id) {
            provider.client_secret = provider.client_secret || '';
            provider.admin_claim_path = provider.admin_claim_path || '';
            provider.admin_claim_value = provider.admin_claim_value || '';
            provider.mfa_claim_path = provider.mfa_claim_path || '';
            provider.mfa_claim_value = provider.mfa_claim_value || '';
        }
    });

    $effect(() => {
        if (provider.scope) {
            provider.scope = provider.scope.replaceAll('+', ' ');
        }
    });

    async function onLogoDelete() {
        let res = await fetchDelete(urlImg);
        if (res.error) {
            err = res.error.message;
        } else {
            logoKey = genKey();
        }
    }

    async function onSubmit(form: HTMLFormElement, params: URLSearchParams) {
        err = '';

        if (
            provider.client_secret &&
            !(provider.client_secret_basic || provider.client_secret_post)
        ) {
            err = ta.providers.config.errNoAuthMethod;
            return;
        }
        if (!provider.use_pkce && !provider.client_secret) {
            err = ta.providers.config.errConfidential;
            return;
        }

        let payload: ProviderRequest = {
            name: provider.name,
            typ: provider.typ,
            enabled: provider.enabled,

            issuer: provider.issuer,
            authorization_endpoint: provider.authorization_endpoint,
            token_endpoint: provider.token_endpoint,
            userinfo_endpoint: provider.userinfo_endpoint,

            use_pkce: provider.use_pkce,
            client_secret_basic: provider.client_secret_basic,
            client_secret_post: provider.client_secret_post,
            auto_onboarding: provider.auto_onboarding,
            auto_link: provider.auto_link,

            client_id: provider.client_id,
            client_secret: provider.client_secret || undefined,
            scope: provider.scope.trim(),

            admin_claim_path: provider.admin_claim_path || undefined,
            admin_claim_value: provider.admin_claim_value || undefined,
            mfa_claim_path: provider.mfa_claim_path || undefined,
            mfa_claim_value: provider.mfa_claim_value || undefined,
        };

        let res = await fetchPut(form.action, payload);
        if (res.error) {
            err = res.error.message;
        } else {
            success = true;
            onSave();
            setTimeout(() => {
                success = false;
            }, 3000);
        }
    }
</script>

<div class="container">
    <Form action={`/auth/v1/providers/${provider.id}`} {onSubmit}>
        <LabeledValue label="ID" mono>
            {provider.id}
        </LabeledValue>
        <div style:height=".15rem"></div>

        <div class="checkbox">
            <InputCheckbox ariaLabel={ta.common.enabled} bind:checked={provider.enabled}>
                {ta.common.enabled}
            </InputCheckbox>
        </div>
        <div class="checkbox">
            <InputCheckbox ariaLabel="Auto-Onboarding" bind:checked={provider.auto_onboarding}>
                Auto-Onboarding
            </InputCheckbox>
        </div>
        <div class="checkbox">
            <InputCheckbox
                ariaLabel={ta.providers.config.autoLink}
                bind:checked={provider.auto_link}
            >
                {ta.providers.config.autoLink}
            </InputCheckbox>
            {#if provider.auto_link}
                <div transition:slide={{ duration: 150 }}>
                    <p>{ta.providers.config.autoLinkDesc1}</p>
                    <p class="err">{ta.providers.config.autoLinkDesc2}</p>
                </div>
            {/if}
        </div>

        <ProviderConfigURLs
            bind:issuer={provider.issuer}
            bind:authorizationEndpoint={provider.authorization_endpoint}
            bind:tokenEndpoint={provider.token_endpoint}
            bind:userinfoEndpoint={provider.userinfo_endpoint}
            {inputWidth}
        />

        <div class="checkbox">
            <InputCheckbox ariaLabel="PKCE" bind:checked={provider.use_pkce}>PKCE</InputCheckbox>
        </div>

        <ProviderConfigClientInfo
            bind:scope={provider.scope}
            bind:name={provider.name}
            bind:clientId={provider.client_id}
            bind:clientSecret={provider.client_secret}
            bind:clientSecretBasic={provider.client_secret_basic}
            bind:clientSecretPost={provider.client_secret_post}
            bind:adminClaimPath={provider.admin_claim_path}
            bind:adminClaimValue={provider.admin_claim_value}
            bind:mfaClaimPath={provider.mfa_claim_path}
            bind:mfaClaimValue={provider.mfa_claim_value}
            usePKCE={provider.use_pkce}
            {inputWidth}
        />

        <div class="logo">
            {#key logoKey}
                <div>
                    <ProviderLogo providerId={provider.id} onDelete={onLogoDelete} />
                </div>
            {/key}
            <InputFile
                method="PUT"
                url={urlImg}
                fileName="logo"
                onSuccess={() => (logoKey = genKey())}
            />
        </div>

        <div class="flex gap-05">
            <Button type="submit" {isLoading}>
                {t.common.save}
            </Button>

            {#if success}
                <IconCheck />
            {/if}

            {#if err}
                <span class="err">
                    {err}
                </span>
            {/if}
        </div>
    </Form>
</div>

<style>
    .checkbox {
        margin: 0.25rem 0;
    }

    .container {
        margin-bottom: 1rem;
    }

    .logo {
        margin: 1rem 0.25rem;
        display: flex;
        flex-direction: column;
        gap: 0.25rem;
    }

    .logo > div {
        margin-left: 0.25rem;
    }
</style>
