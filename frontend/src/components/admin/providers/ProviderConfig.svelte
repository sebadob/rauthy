<script>
    import * as yup from "yup";
    import {extractFormErrors} from "../../../utils/helpers.js";
    import Button from "$lib/Button.svelte";
    import {REGEX_CLIENT_NAME, REGEX_LOWERCASE_SPACE, REGEX_URI} from "../../../utils/constants.js";
    import {onMount} from "svelte";
    import {putProvider} from "../../../utils/dataFetchingAdmin.js";
    import Input from "$lib/inputs/Input.svelte";
    import Switch from "$lib/Switch.svelte";
    import PasswordInput from "$lib/inputs/PasswordInput.svelte";

    export let provider = {};
    export let onSave;

    const inputWidth = '25rem';

    let isLoading = false;
    let err = '';
    let success = false;
    let timer;
    let isDefault = false;

    $: if (success) {
        timer = setTimeout(() => {
            success = false;
            onSave();
        }, 2000);
    }

    $: if (provider.scope) {
        provider.scope = provider.scope.replaceAll('+', ' ');
    }

    onMount(() => {
        console.log(provider);
        return () => clearTimeout(timer);
    });

    let formErrors = {};
    const schema = yup.object().shape({
        issuer: yup.string().trim().matches(REGEX_URI, "Can only contain URI safe characters, length max: 128"),
        authorization_endpoint: yup.string().url(),
        token_endpoint: yup.string().url(),
        userinfo_endpoint: yup.string().url(),

        name: yup.string().trim().matches(REGEX_CLIENT_NAME, "Can only contain: 'a-zA-Z0-9À-ÿ- ', length max: 128"),
        client_id: yup.string().trim().matches(REGEX_URI, "Can only contain URI safe characters, length max: 128"),
        client_secret: yup.string().trim().max(256, "Max 256 characters"),
        scope: yup.string().trim().matches(REGEX_LOWERCASE_SPACE, "Can only contain: 'a-zA-Z0-9-_/ ', length max: 128"),

        admin_claim_path: yup.string().trim().nullable().matches(REGEX_URI, "Can only contain URI safe characters, length max: 128"),
        admin_claim_value: yup.string().trim().nullable().matches(REGEX_URI, "Can only contain URI safe characters, length max: 128"),
    });

    function handleKeyPress(event) {
        if (event.code === 'Enter') {
            onSubmit();
        }
    }

    async function onSubmit() {
        const valid = await validateForm();
        if (!valid) {
            err = 'Invalid input';
            return;
        }

        if (!provider.use_pkce && !provider.client_secret) {
            err = 'Must at least be a confidential client or use PKCE';
            return;
        }

        err = '';
        isLoading = true;

        let res = await putProvider(provider.id, provider);
        if (res.ok) {
            success = true;
        } else {
            let body = await res.json();
            err = body.message;
        }

        isLoading = false;
    }

    async function validateForm() {
        try {
            await schema.validate(provider, {abortEarly: false});
            formErrors = {};
            return true;
        } catch (err) {
            formErrors = extractFormErrors(err);
            return false;
        }
    }

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

    <!--    <Input-->
    <!--            bind:value={scope.name}-->
    <!--            bind:error={formErrors.name}-->
    <!--            autocomplete="off"-->
    <!--            placeholder="Scope Name"-->
    <!--            on:input={validateForm}-->
    <!--            disabled={isDefault}-->
    <!--    >-->
    <!--        SCOPE NAME-->
    <!--    </Input>-->

    <!-- Mappings -->
    <div class="separator"></div>

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

    <div class="header">
        Allow insecure TLS certificates
    </div>
    <div class="ml mb">
        <Switch bind:selected={provider.danger_allow_insecure}/>
    </div>

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
            bind:error={formErrors.client_secret}
            autocomplete="off"
            placeholder="Client Secret"
            on:input={validateForm}
            width={inputWidth}
    >
        CLIENT SECRET
    </PasswordInput>

    <div class="desc">
        Rauthy Admin mapping.<br>
        If the user logging in should be mapped automatically to the rauthy_admin role,<br>
        specify the json path and expected value here.
    </div>
    <Input
            bind:value={provider.admin_claim_path}
            bind:error={formErrors.admin_claim_path}
            autocomplete="off"
            placeholder="Admin Claim Path"
            on:input={validateForm}
            width={inputWidth}
    >
        ADMIN CLAIM PATH
    </Input>
    <Input
            bind:value={provider.admin_claim_value}
            bind:error={formErrors.admin_claim_value}
            autocomplete="off"
            placeholder="Admin Claim Value"
            on:input={validateForm}
            width={inputWidth}
    >
        ADMIN CLAIM VALUE
    </Input>

    {#if !isDefault}
        <Button on:click={onSubmit} level={1} width="4rem">SAVE</Button>

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
        margin: .5rem .5rem .25rem .5rem;
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

    .ml {
        margin-left: .5rem;
    }

    .mb {
        margin-bottom: .5rem;
    }

    .success {
        color: var(--col-ok);
    }

    .unit {
        margin: 7px 0;
    }

    .value {
        margin-left: 5px;
        display: flex;
    }
</style>
