<script lang="ts">
    import * as yup from "yup";
    import Button from "$lib5/button/Button.svelte";
    import {
        REGEX_CLIENT_NAME,
        REGEX_URI,
        REGEX_PROVIDER_SCOPE
    } from "$utils/constants";
    import Input from "$lib5/form/Input.svelte";
    import Switch from "$lib5/Switch.svelte";
    import JsonPathDesc from "./JsonPathDesc.svelte";
    import ImageUploadRaw from "../../ImageUploadRaw.svelte";
    import ProviderLogo from "../../ProviderLogo.svelte";
    import type {ProviderRequest, ProviderResponse} from "$api/types/auth_provider.ts";
    import IconCheck from "$icons/IconCheck.svelte";
    import InputArea from "$lib5/form/InputArea.svelte";
    import InputPassword from "$lib5/form/InputPassword.svelte";
    import Form from "$lib5/form/Form.svelte";
    import {fetchPut} from "$api/fetch.ts";
    import {useI18nAdmin} from "$state/i18n_admin.svelte.ts";
    import {useI18n} from "$state/i18n.svelte.ts";
    import LabeledValue from "$lib5/LabeledValue.svelte";
    import InputCheckbox from "$lib5/form/InputCheckbox.svelte";

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
    const labelWidth = '12rem';

    let isLoading = $state(false);
    let err = $state('');
    let success = $state(false);
    let isDefault = false;
    let showRootPem = $state(!!provider.root_pem);
    let logo = $state();

    let formErrors = $state({});
    const schema = yup.object().shape({
        issuer: yup.string().trim().matches(REGEX_URI, "Can only contain URI safe characters, length max: 128"),
        authorization_endpoint: yup.string().url(),
        token_endpoint: yup.string().url(),
        userinfo_endpoint: yup.string().url(),

        name: yup.string().trim().matches(REGEX_CLIENT_NAME, "Can only contain: 'a-zA-Z0-9À-ÿ- ', length max: 128"),
        client_id: yup.string().trim().matches(REGEX_URI, "Can only contain URI safe characters, length max: 128"),
        client_secret: yup.string().trim().max(256, "Max 256 characters"),
        scope: yup.string().trim().matches(REGEX_PROVIDER_SCOPE, "Can only contain: 'a-zA-Z0-9-_/ ', length max: 128"),

        admin_claim_path: yup.string().trim().nullable().matches(REGEX_URI, "Can only contain URI safe characters, length max: 128"),
        admin_claim_value: yup.string().trim().nullable().matches(REGEX_URI, "Can only contain URI safe characters, length max: 128"),
        mfa_claim_path: yup.string().trim().nullable().matches(REGEX_URI, "Can only contain URI safe characters, length max: 128"),
        mfa_claim_value: yup.string().trim().nullable().matches(REGEX_URI, "Can only contain URI safe characters, length max: 128"),
    });

    $effect(() => {
        if (provider.scope) {
            provider.scope = provider.scope.replaceAll('+', ' ');
        }
    });

    $effect(() => {
        uploadLogo();
    });

    async function onSubmit(form: HTMLFormElement, params: URLSearchParams) {
        err = '';

        if (provider.client_secret && !(provider.client_secret_basic || provider.client_secret_post)) {
            err = 'You have given a client secret, but no client auth method is active';
            return false;
        }
        if (!provider.use_pkce && !provider.client_secret) {
            err = 'Must at least be a confidential client or use PKCE';
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
            setTimeout(() => {
                success = false;
                onSave();
            }, 3000);
        }
    }

    async function uploadLogo() {
        if (!logo) {
            return;
        }

        isLoading = true;

        console.log('logo upload, typeof', typeof logo, logo);
        const formData = new FormData();
        formData.append('logo', logo);

        let res = await fetchPut(`/auth/v1/providers/${provider.id}/img`, formData);
        if (res.error) {
            console.error(res.error);
        } else {
            // We don't need to do anything in that case.
            // A reload of the logo in the body below will be done depending
            // on state changes of `isLoading`.
        }

        isLoading = false;
    }
</script>

<div class="container">
    <Form action={`/auth/v1/providers/${provider.id}`} {onSubmit}>
        <LabeledValue label="ID" mono>
            {provider.id}
        </LabeledValue>

        <!-- Mappings -->
        <div class="separator"></div>

        <div class="checkbox">
            <InputCheckbox ariaLabel="Enabled" bind:checked={provider.enabled}>
                Enabled
            </InputCheckbox>
        </div>
        <div class="checkbox">
            <InputCheckbox ariaLabel="Custom Root CA PEM" bind:checked={showRootPem}>
                Custom Root CA PEM
            </InputCheckbox>
        </div>

        {#if showRootPem}
            <InputArea
                    rows={17}
                    name="rootPem"
                    label="PEM Root Certificate"
                    placeholder="-----BEGIN CERTIFICATE-----
...
 -----END CERTIFICATE-----"
                    bind:value={provider.root_pem}
                    errMsg="-----BEGIN CERTIFICATE----- ..."
            />
        {:else}
            <div class="checkbox">
                <InputCheckbox ariaLabel="Allow insecure TLS" bind:checked={provider.danger_allow_insecure}>
                    Allow insecure TLS
                </InputCheckbox>
            </div>
        {/if}

        <Input
                bind:value={provider.issuer}
                autocomplete="off"
                label="Issuer URL"
                placeholder="Issuer URL"
                width={inputWidth}
        />

        <Input
                bind:value={provider.authorization_endpoint}
                autocomplete="off"
                label="Authorization Endpoint"
                placeholder="Authorization Endpoint"
                width={inputWidth}
        />

        <Input
                bind:value={provider.token_endpoint}
                autocomplete="off"
                label="Token Endpoint"
                placeholder="Token Endpoint"
                width={inputWidth}
        />

        <Input
                bind:value={provider.userinfo_endpoint}
                autocomplete="off"
                label="Userinfo Endpoint"
                placeholder="Userinfo Endpoint"
                width={inputWidth}
        />

        <div class="checkbox">
            <InputCheckbox ariaLabel="Use PKCE" bind:checked={provider.use_pkce}>
                Use PKCE
            </InputCheckbox>
        </div>

        <p>
            The scope the client should use when redirecting to the login.<br>
            Provide the values separated by space.
        </p>
        <Input
                bind:value={provider.scope}
                autocomplete="off"
                label="Scope"
                placeholder="openid profile email"
                width={inputWidth}
        />

        <p>
            Client name for the Rauthy login form
        </p>
        <Input
                bind:value={provider.name}
                autocomplete="off"
                label="Client Name"
                placeholder="Client Name"
                width={inputWidth}
        />

        <p>
            Client ID given by the auth provider
        </p>
        <Input
                bind:value={provider.client_id}
                autocomplete="off"
                label="Client ID"
                placeholder="Client ID"
                width={inputWidth}
        />

        <p>
            Client Secret given by the auth provider.<br>
            At least a client secret or PKCE is required.
        </p>
        <InputPassword
                bind:value={provider.client_secret}
                autocomplete="off"
                label="Client Secret"
                placeholder="Client Secret"
                width={inputWidth}
        />

        <p>
            The authentication method to use on the <code>/token</code> endpoint.<br>
            Most providers should work with <code>basic</code>, some only with <code>post</code>.
            In rare situations, you need both, while it can lead to errors with others.
        </p>
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
        <p>
            You can map a user to be a rauthy admin depending on an upstream ID claim.
        </p>
        <Input
                bind:value={provider.admin_claim_path}
                autocomplete="off"
                label="Admin Claim Path"
                placeholder="$.roles.*"
                width={inputWidth}
        />
        <Input
                bind:value={provider.admin_claim_value}
                autocomplete="off"
                label="Admin Claim Value"
                placeholder="rauthy_admin"
                width={inputWidth}
        />

        <p>
            If your provider issues a claim indicating that the user has used at least 2FA during
            login, you can specify the mfa claim path.
        </p>
        <Input
                bind:value={provider.mfa_claim_path}
                autocomplete="off"
                label="MFA Claim Path"
                placeholder="$.amr.*"
                width={inputWidth}
        />
        <Input
                bind:value={provider.mfa_claim_value}
                autocomplete="off"
                label="MFA Claim Value"
                placeholder="mfa"
                width={inputWidth}
        />

        <div class="logo">
            <ImageUploadRaw bind:image={logo}/>
            {#if !isLoading}
                <ProviderLogo providerId={provider.id}/>
            {/if}
        </div>

        {#if !isDefault}
            <Button onclick={onSubmit} {isLoading}>
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
        {/if}
    </Form>
</div>

<style>
    .logo {
        margin: 1rem .25rem;
    }

    .checkbox {
        margin: .25rem 0;
    }
</style>
