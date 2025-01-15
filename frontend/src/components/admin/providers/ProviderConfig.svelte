<script>
    import {run} from 'svelte/legacy';

    import * as yup from "yup";
    import {extractFormErrors} from "../../../utils/helpers";
    import Button from "$lib/Button.svelte";
    import {
        REGEX_CLIENT_NAME,
        REGEX_URI,
        REGEX_PEM,
        REGEX_PROVIDER_SCOPE
    } from "../../../utils/constants.js";
    import {onMount} from "svelte";
    import {putProvider, putProviderLogo} from "../../../utils/dataFetchingAdmin.js";
    import Input from "$lib/inputs/Input.svelte";
    import Switch from "$lib/Switch.svelte";
    import PasswordInput from "$lib/inputs/PasswordInput.svelte";
    import JsonPathDesc from "./JsonPathDesc.svelte";
    import Textarea from "$lib/inputs/Textarea.svelte";
    import ImageUploadRaw from "../../ImageUploadRaw.svelte";
    import ProviderLogo from "../../ProviderLogo.svelte";

    let {provider = $bindable({}), onSave = $bindable()} = $props();

    const inputWidth = '25rem';

    let isLoading = $state(false);
    let err = $state('');
    let success = $state(false);
    let timer = $state();
    let isDefault = false;
    let showRootPem = $state(provider.root_pem);
    let logo = $state();


    onMount(() => {
        return () => clearTimeout(timer);
    });

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

    function handleKeyPress(event) {
        if (event.code === 'Enter') {
            onSubmit();
        }
    }

    async function onSubmit() {
        if (!showRootPem) {
            provider.root_pem = undefined;
        }

        const valid = await validateForm();
        if (!valid) {
            return;
        }

        if (!provider.use_pkce && !provider.client_secret) {
            err = 'Must at least be a confidential client or use PKCE';
            return;
        }

        err = '';
        isLoading = true;

        if (provider.root_pem) {
            // make sure we reset to false, which is what a user would expect
            provider.danger_allow_insecure = false;
            provider.root_pem = provider.root_pem.trim();
        } else {
            // make sure to not submit am empty string
            provider.root_pem = undefined;
        }

        let res = await putProvider(provider.id, provider);
        if (res.ok) {
            success = true;
        } else {
            let body = await res.json();
            err = body.message;
        }

        isLoading = false;
    }

    async function uploadLogo(payload) {
        isLoading = true;

        let res = await putProviderLogo(provider.id, payload);
        if (res.ok) {
            // We don't need to do anything in that case.
            // A reload of the logo in the body below will be done depending
            // on state changes of `isLoading`.
        } else {
            let body = await res.json();
            console.error(body.message);
        }

        isLoading = false;
    }

    async function validateForm() {
        formErrors = {};
        try {
            await schema.validate(provider, {abortEarly: false});

            if (provider.client_secret && !(provider.client_secret_basic || provider.client_secret_post)) {
                err = 'You have given a client secret, but no client auth method is active';
                return false;
            } else if (provider.root_pem && provider.root_pem.length > 0) {
                if (!REGEX_PEM.test(provider.root_pem.trim())) {
                    formErrors.root_pem = 'Invalid PEM certificate';
                }
            } else {
                err = 'Invalid input';
            }

            return true;
        } catch (err) {
            formErrors = extractFormErrors(err);
            return false;
        }
    }

    run(() => {
        if (success) {
            timer = setTimeout(() => {
                success = false;
                onSave();
            }, 2000);
        }
    });
    run(() => {
        if (provider.scope) {
            provider.scope = provider.scope.replaceAll('+', ' ');
        }
    });
    // This will trigger when the upload image button has been clicked
    run(() => {
        if (logo) {
            uploadLogo(logo);
        }
    });
</script>

<div class="container">
    <div class="unit">
        <div class="label font-label">
            ID
        </div>
        <div class="value font-mono">
            {provider.id}
        </div>
    </div>

    <!-- Mappings -->
    <div class="separator"></div>

    <div class="header">
        Enabled
    </div>
    <div class="ml mb">
        <Switch bind:selected={provider.enabled}/>
    </div>

    <div class="header">
        Custom Root CA PEM
    </div>
    <div class="ml mb">
        <Switch bind:selected={showRootPem}/>
    </div>

    {#if showRootPem}
         <Textarea
                 rows={17}
                 name="rootPem"
                 placeholder="-----BEGIN CERTIFICATE-----
-----END CERTIFICATE-----"
                 bind:value={provider.root_pem}
                 error={formErrors.root_pem}
         >
            Root Certificate in PEM format
        </Textarea>
    {:else}
        <div class="header">
            Allow insecure TLS certificates
        </div>
        <div class="ml mb">
            <Switch bind:selected={provider.danger_allow_insecure}/>
        </div>
    {/if}

    <Input
            bind:value={provider.issuer}
            bind:error={formErrors.issuer}
            autocomplete="off"
            placeholder="Issuer URL"
            on:input={validateForm}
            width={inputWidth}
    >
        ISSUER URL
    </Input>

    <Input
            bind:value={provider.authorization_endpoint}
            bind:error={formErrors.authorization_endpoint}
            autocomplete="off"
            placeholder="Authorization Endpoint"
            on:input={validateForm}
            width={inputWidth}
    >
        AUTHORIZATION ENDPOINT
    </Input>

    <Input
            bind:value={provider.token_endpoint}
            bind:error={formErrors.token_endpoint}
            autocomplete="off"
            placeholder="Token Endpoint"
            on:input={validateForm}
            width={inputWidth}
    >
        TOKEN ENDPOINT
    </Input>

    <Input
            bind:value={provider.userinfo_endpoint}
            bind:error={formErrors.userinfo_endpoint}
            autocomplete="off"
            placeholder="Userinfo Endpoint"
            on:input={validateForm}
            width={inputWidth}
    >
        USERINFO ENDPOINT
    </Input>

    <div class="header">
        Use PKCE
    </div>
    <div class="ml mb">
        <Switch bind:selected={provider.use_pkce}/>
    </div>

    <div class="desc">
        The scope the client should use when redirecting to the login.<br>
        Provide the values separated by space.
    </div>
    <Input
            bind:value={provider.scope}
            bind:error={formErrors.scope}
            autocomplete="off"
            placeholder="openid profile email"
            on:input={validateForm}
            width={inputWidth}
    >
        SCOPE
    </Input>

    <div class="desc">
        Client name for the Rauthy login form
    </div>
    <Input
            bind:value={provider.name}
            bind:error={formErrors.name}
            autocomplete="off"
            placeholder="Client Name"
            on:input={validateForm}
            width={inputWidth}
    >
        CLIENT NAME
    </Input>

    <div class="desc">
        Client ID given by the auth provider
    </div>
    <Input
            bind:value={provider.client_id}
            bind:error={formErrors.client_id}
            autocomplete="off"
            placeholder="Client ID"
            on:input={validateForm}
            width={inputWidth}
    >
        CLIENT ID
    </Input>

    <div class="desc">
        Client Secret given by the auth provider.<br>
        At least a client secret or PKCE is required.
    </div>
    <PasswordInput
            bind:value={provider.client_secret}
            error={formErrors.client_secret}
            autocomplete="off"
            placeholder="Client Secret"
            on:input={validateForm}
            width={inputWidth}
    >
        CLIENT SECRET
    </PasswordInput>

    <div class="desc">
        <p>
            The authentication method to use on the <code>/token</code> endpoint.<br>
            Most providers should work with <code>basic</code>, some only with <code>post</code>.
            In rare situations, you need both, while it can lead to errors with others.
        </p>
    </div>
    <div class="switchRow">
        <div>
            client_secret_basic
        </div>
        <Switch
                bind:selected={provider.client_secret_basic}
        />
    </div>
    <div class="switchRow">
        <div>
            client_secret_post
        </div>
        <Switch
                bind:selected={provider.client_secret_post}
        />
    </div>

    <JsonPathDesc/>
    <div class="desc">
        <p>
            You can map a user to be a rauthy admin depending on an upstream ID claim.
        </p>
    </div>
    <Input
            bind:value={provider.admin_claim_path}
            bind:error={formErrors.admin_claim_path}
            autocomplete="off"
            placeholder="$.roles.*"
            on:input={validateForm}
            width={inputWidth}
    >
        ADMIN CLAIM PATH
    </Input>
    <Input
            bind:value={provider.admin_claim_value}
            bind:error={formErrors.admin_claim_value}
            autocomplete="off"
            placeholder="rauthy_admin"
            on:input={validateForm}
            width={inputWidth}
    >
        ADMIN CLAIM VALUE
    </Input>

    <div class="desc">
        <p>
            If your provider issues a claim indicating that the user has used at least 2FA during
            login, you can specify the mfa claim path.
        </p>
    </div>
    <Input
            bind:value={provider.mfa_claim_path}
            bind:error={formErrors.mfa_claim_path}
            autocomplete="off"
            placeholder="$.amr.*"
            on:input={validateForm}
            width={inputWidth}
    >
        MFA CLAIM PATH
    </Input>
    <Input
            bind:value={provider.mfa_claim_value}
            bind:error={formErrors.mfa_claim_value}
            autocomplete="off"
            placeholder="mfa"
            on:input={validateForm}
            width={inputWidth}
    >
        MFA CLAIM VALUE
    </Input>

    <div class="logo">
        <ImageUploadRaw bind:image={logo}/>
        {#if !isLoading}
            <ProviderLogo providerId={provider.id}/>
        {/if}
    </div>

    {#if !isDefault}
        <Button on:click={onSubmit} level={1} width="4rem" bind:isLoading>SAVE</Button>

        {#if success}
            <div class="success">
                Success
            </div>
        {/if}

        {#if err}
            <div class="mainErr err">
                {err}
            </div>
        {/if}
    {/if}
</div>

<style>
    .container {
        padding: 0 10px 10px 10px;
    }

    .desc {
        display: flex;
        margin: .5rem;
    }

    .desc > p {
        margin: .2rem 0;
    }

    .err {
        color: var(--col-err);
    }

    .err, .success {
        margin: 0 7px;
    }

    .header {
        display: flex;
        font-size: .9rem;
        margin-left: 10px;
    }

    .label {
        margin: 5px 5px 0 5px;
        font-size: .9rem;
    }

    .logo {
        margin: 1rem .25rem;
    }

    .ml {
        margin-left: .5rem;
    }

    .mb {
        margin-bottom: .5rem;
    }

    .success {
        color: var(--col-ok);
    }

    .switchRow {
        margin-bottom: .25rem;
        padding-left: .5rem;
        display: grid;
        grid-template-columns: 9rem 1fr;
    }

    .unit {
        margin: 7px 0;
    }

    .value {
        margin-left: 5px;
        display: flex;
    }
</style>
