<script lang="ts">
    import Button from '#lib/button/Button.svelte';
    import type { ProviderRequest, ProviderResponse } from '#api/types/auth_provider.ts';
    import IconCheck from '#icons/IconCheck.svelte';
    import Form from '#lib/form/Form.svelte';
    import { fetchPut } from '#api/fetch.ts';
    import { useI18nAdmin } from '#state/i18n_admin.svelte.ts';
    import { useI18n } from '#state/i18n.svelte.ts';
    import LabeledValue from '#lib/LabeledValue.svelte';
    import InputCheckbox from '#lib/form/InputCheckbox.svelte';
    import Section from '#lib/Section.svelte';
    import ProviderConfigURLs from '#lib/admin/providers/blocks/ProviderConfigURLs.svelte';
    import ProviderConfigClientInfo from '#lib/admin/providers/blocks/ProviderConfigClientInfo.svelte';
    import { slide } from 'svelte/transition';

    let {
        provider = $bindable(),
        onSave,
    }: {
        provider: ProviderResponse;
        onSave: () => void;
    } = $props();

    let t = useI18n();
    let ta = useI18nAdmin();

    const inputWidth = '100%';

    let isLoading = $state(false);
    let err = $state('');
    let success = $state(false);

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

        isLoading = true;
        let res = await fetchPut(form.action, payload);
        isLoading = false;
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

<div class="provider-form">
    <Form action={`/auth/v1/providers/${provider.id}`} {onSubmit}>
        <div class="sections">
            <Section title={ta.providers.config.sections?.access ?? 'Access and linking'}>
                <LabeledValue label="ID" mono>{provider.id}</LabeledValue>
                <div class="switches">
                    <InputCheckbox ariaLabel={ta.common.enabled} bind:checked={provider.enabled}>
                        {ta.common.enabled}
                    </InputCheckbox>
                    <InputCheckbox
                        ariaLabel="Auto-Onboarding"
                        bind:checked={provider.auto_onboarding}
                    >
                        Auto-Onboarding
                    </InputCheckbox>
                    <InputCheckbox
                        ariaLabel={ta.providers.config.autoLink}
                        bind:checked={provider.auto_link}
                    >
                        {ta.providers.config.autoLink}
                    </InputCheckbox>
                </div>
                {#if provider.auto_link}
                    <div transition:slide={{ duration: 150 }}>
                        <p>{ta.providers.config.autoLinkDesc1}</p>
                        <p class="err">{ta.providers.config.autoLinkDesc2}</p>
                    </div>
                {/if}
            </Section>

            <Section title={ta.providers.config.sections?.endpoints ?? 'Endpoints'}>
                <div class="field-grid">
                    <ProviderConfigURLs
                        bind:issuer={provider.issuer}
                        bind:authorizationEndpoint={provider.authorization_endpoint}
                        bind:tokenEndpoint={provider.token_endpoint}
                        bind:userinfoEndpoint={provider.userinfo_endpoint}
                        {inputWidth}
                    />
                </div>
            </Section>

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
            >
                {#snippet clientControls()}
                    <InputCheckbox ariaLabel="PKCE" bind:checked={provider.use_pkce}
                        >PKCE</InputCheckbox
                    >
                {/snippet}
            </ProviderConfigClientInfo>
        </div>

        <div class="form-actions">
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
    .provider-form {
        margin-bottom: 1rem;
    }

    .sections {
        display: grid;
        gap: var(--ui-space);
    }

    .field-grid {
        display: grid;
        grid-template-columns: repeat(auto-fit, minmax(min(100%, 16rem), 1fr));
        gap: var(--ui-space);
        --ui-field-margin: 0;
    }

    .switches,
    .form-actions {
        display: flex;
        align-items: center;
        flex-wrap: wrap;
        gap: 0.75rem 1.25rem;
    }

    .form-actions {
        margin-top: var(--ui-space);
    }

    .sections p {
        margin-block: 0;
    }
</style>
