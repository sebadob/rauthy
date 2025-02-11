<script lang="ts">
    import Button from "$lib5/button/Button.svelte";
    import Input from "$lib5/form/Input.svelte";
    import JsonPathDesc from "$lib5/admin/providers/JsonPathDesc.svelte";
    import ProviderLogo from "../../../components/ProviderLogo.svelte";
    import type {ProviderRequest, ProviderResponse} from "$api/types/auth_provider.ts";
    import IconCheck from "$icons/IconCheck.svelte";
    import InputArea from "$lib5/form/InputArea.svelte";
    import InputPassword from "$lib5/form/InputPassword.svelte";
    import Form from "$lib5/form/Form.svelte";
    import {fetchPut, uploadFile} from "$api/fetch.ts";
    import {useI18nAdmin} from "$state/i18n_admin.svelte.ts";
    import {useI18n} from "$state/i18n.svelte.ts";
    import LabeledValue from "$lib5/LabeledValue.svelte";
    import InputCheckbox from "$lib5/form/InputCheckbox.svelte";
    import {PATTERN_CLIENT_NAME, PATTERN_PEM, PATTERN_SCOPE_SPACE, PATTERN_URI} from "$utils/patterns.ts";
    import {slide} from "svelte/transition";
    import InputFile from "$lib5/form/InputFile.svelte";
    import {genKey} from "$utils/helpers.ts";

    let {
        provider,
        onSave = $bindable(),
    }: {
        provider: ProviderResponse,
        onSave: () => void,
    } = $props();

    let t = useI18n();
    let ta = useI18nAdmin();

    const inputWidth = 'min(calc(100dvw - .5rem), 30rem)';

    let isLoading = $state(false);
    let err = $state('');
    let success = $state(false);
    let showRootPem = $state(!!provider.root_pem);
    let logoKey = $state(genKey());

    $effect(() => {
        if (provider.scope) {
            provider.scope = provider.scope.replaceAll('+', ' ');
        }
    });

    async function onSubmit(form: HTMLFormElement, params: URLSearchParams) {
        err = '';

        if (provider.client_secret && !(provider.client_secret_basic || provider.client_secret_post)) {
            err = ta.providers.config.errNoAuthMethod;
            return false;
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

            danger_allow_insecure: showRootPem && provider.root_pem ? false : provider.danger_allow_insecure,
            use_pkce: provider.use_pkce,
            client_secret_basic: provider.client_secret_basic,
            client_secret_post: provider.client_secret_post,

            client_id: provider.client_id,
            client_secret: provider.client_secret,
            scope: provider.scope,
            root_pem: showRootPem && provider.root_pem ? provider.root_pem.trim() : undefined,

            admin_claim_path: provider.admin_claim_path,
            admin_claim_value: provider.admin_claim_value,
            mfa_claim_path: provider.mfa_claim_path,
            mfa_claim_value: provider.mfa_claim_value,
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
                        bind:value={provider.root_pem}
                        errMsg="-----BEGIN CERTIFICATE----- ..."
                        width="min(40rem, calc(100dvw - .5rem))"
                        fontMono
                        pattern={PATTERN_PEM}
                />
            </div>
        {:else}
            <div class="checkbox">
                <InputCheckbox
                        ariaLabel={ta.providers.config.allowInsecureTls}
                        bind:checked={provider.danger_allow_insecure}
                >
                    {ta.providers.config.allowInsecureTls}
                </InputCheckbox>
            </div>
        {/if}

        <Input
                bind:value={provider.issuer}
                autocomplete="off"
                label="Issuer URL"
                placeholder="Issuer URL"
                required
                pattern={PATTERN_URI}
                width={inputWidth}
        />
        <Input
                bind:value={provider.authorization_endpoint}
                autocomplete="off"
                label="Authorization Endpoint"
                placeholder="Authorization Endpoint"
                required
                pattern={PATTERN_URI}
                width={inputWidth}
        />
        <Input
                bind:value={provider.token_endpoint}
                autocomplete="off"
                label="Token Endpoint"
                placeholder="Token Endpoint"
                required
                pattern={PATTERN_URI}
                width={inputWidth}
        />
        <Input
                bind:value={provider.userinfo_endpoint}
                autocomplete="off"
                label="Userinfo Endpoint"
                placeholder="Userinfo Endpoint"
                required
                pattern={PATTERN_URI}
                width={inputWidth}
        />

        <div class="checkbox">
            <InputCheckbox ariaLabel="PKCE" bind:checked={provider.use_pkce}>
                PKCE
            </InputCheckbox>
        </div>

        <p class="desc">{ta.providers.config.descScope}</p>
        <Input
                bind:value={provider.scope}
                autocomplete="off"
                label="Scope"
                placeholder="openid profile email"
                required
                pattern={PATTERN_SCOPE_SPACE}
                width={inputWidth}
        />

        <p class="desc">{ta.providers.config.descClientName}</p>
        <Input
                bind:value={provider.name}
                autocomplete="off"
                label={ta.providers.config.clientName}
                placeholder={ta.providers.config.clientName}
                required
                pattern={PATTERN_CLIENT_NAME}
                width={inputWidth}
        />

        <p class="desc">{ta.providers.config.descClientId}</p>
        <Input
                bind:value={provider.client_id}
                autocomplete="off"
                label="Client ID"
                placeholder="Client ID"
                required
                pattern={PATTERN_URI}
                width={inputWidth}
        />

        <p class="desc">{ta.providers.config.descClientSecret}</p>
        <InputPassword
                bind:value={provider.client_secret}
                autocomplete="off"
                label="Client Secret"
                placeholder="Client Secret"
                maxLength={256}
                width={inputWidth}
        />

        <p>{@html ta.providers.config.descAuthMethod}</p>
        <div class="checkbox">
            <InputCheckbox ariaLabel="client_secret_basic" bind:checked={provider.client_secret_basic}>
                client_secret_basic
            </InputCheckbox>
        </div>
        <div class="checkbox">
            <InputCheckbox ariaLabel="client_secret_post" bind:checked={provider.client_secret_post}>
                client_secret_post
            </InputCheckbox>
        </div>

        <JsonPathDesc/>

        <p class="desc">{ta.providers.config.mapUser}</p>
        <Input
                bind:value={provider.admin_claim_path}
                autocomplete="off"
                label={ta.providers.config.pathAdminClaim}
                placeholder="$.roles.*"
                pattern={PATTERN_URI}
                width={inputWidth}
        />
        <Input
                bind:value={provider.admin_claim_value}
                autocomplete="off"
                label={ta.providers.config.valueAdminClaim}
                placeholder="rauthy_admin"
                pattern={PATTERN_URI}
                width={inputWidth}
        />

        <p class="desc">{ta.providers.config.mapMfa}</p>
        <Input
                bind:value={provider.mfa_claim_path}
                autocomplete="off"
                label={ta.providers.config.pathMfaClaim}
                placeholder="$.amr.*"
                pattern={PATTERN_URI}
                width={inputWidth}
        />
        <Input
                bind:value={provider.mfa_claim_value}
                autocomplete="off"
                label={ta.providers.config.valueMfaClaim}
                placeholder="mfa"
                pattern={PATTERN_URI}
                width={inputWidth}
        />

        <div class="logo">
            {#key logoKey}
                <div>
                    <ProviderLogo providerId={provider.id}/>
                </div>
            {/key}
            <InputFile
                    method="PUT"
                    url={`/auth/v1/providers/${provider.id}/img`}
                    fileName="logo"
                    onSuccess={() => logoKey = genKey()}
            />
        </div>

        <div class="flex gap-05">
            <Button type="submit" {isLoading}>
                {t.common.save}
            </Button>

            {#if success}
                <IconCheck/>
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
    .container {
        margin-bottom: 1rem;
    }

    .desc {
        margin-bottom: -.5rem;
    }

    .logo {
        margin: 1rem .25rem;
        display: flex;
        flex-direction: column;
        gap: .25rem;
    }

    .logo > div {
        margin-left: .25rem;
    }

    .checkbox {
        margin: .25rem 0;
    }
</style>
