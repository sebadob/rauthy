<script>
    import ExpandContainer from "$lib/ExpandContainer.svelte";
    import * as yup from "yup";
    import {extractFormErrors} from "../../../utils/helpers.js";
    import {onMount} from "svelte";
    import Button from "$lib/Button.svelte";
    import {postProviderLookup, postProvider} from "../../../utils/dataFetchingAdmin.js";
    import Input from "$lib/inputs/Input.svelte";
    import Switch from "$lib/Switch.svelte";
    import CheckIcon from "$lib/CheckIcon.svelte";
    import OptionSelect from "$lib/OptionSelect.svelte";
    import PasswordInput from "$lib/inputs/PasswordInput.svelte";
    import {REGEX_CLIENT_NAME, REGEX_LOWERCASE_SPACE, REGEX_ROLES, REGEX_URI} from "../../../utils/constants.js";

    export let idx = -1;
    export let onSave;

    const inputWidth = '25rem';

    let expandContainer = false;
    let isLoading = false;
    let err = '';
    let success = false;
    let timer;

    let configLookup = {
        issuer: '',
        danger_allow_http: false,
        danger_allow_insecure: false,
    };
    let config = {
        // fixed values after lookup
        issuer: '',
        danger_allow_http: false,
        danger_allow_insecure: false,
        authorization_endpoint: '',
        token_endpoint: '',
        token_auth_method_basic: false,
        userinfo_endpoint: '',
        use_pkce: true,
        // user defined values
        name: '',
        client_id: '',
        client_secret: '',
        scope: '',
        // maybe additional ones in the future like client_logo
    }
    // TODO add "the big ones" as templates in the future
    let modes = ['Auto', 'Custom'];
    let mode = modes[0];
    $: isAuto = mode === modes[0];

    let formErrors = {};
    const schemaConfig = yup.object().shape({
        issuer: yup.string().trim().matches(REGEX_URI, "Can only contain URI safe characters, length max: 128"),
        authorization_endpoint: yup.string().url(),
        token_endpoint: yup.string().url(),
        userinfo_endpoint: yup.string().url(),

        name: yup.string().trim().matches(REGEX_CLIENT_NAME, "Can only contain: 'a-zA-Z0-9À-ÿ- ', length max: 128"),
        client_id: yup.string().trim().matches(REGEX_URI, "Can only contain URI safe characters, length max: 128"),
        client_secret: yup.string().trim().max(256, "Max 256 characters"),
        scope: yup.string().trim().matches(REGEX_LOWERCASE_SPACE, "Can only contain: 'a-zA-Z0-9-_/ ', length max: 128"),
    });
    const schemaLookup = yup.object().shape({
        issuer: yup.string().trim().matches(REGEX_URI, "Can only contain URI safe characters, length max: 128"),
    });

    $: if (success) {
        timer = setTimeout(() => {
            onSave();
            success = false;
            expandContainer = false;
            resetValues();
        }, 1500);
    }

    onMount(() => {
        return () => {
            expandContainer = false;
            clearTimeout(timer);
        }
    });

    async function onSubmitConfig() {
        const valid = await validateFormConfig();
        if (!valid) {
            err = 'Invalid input';
            return;
        }

        if (!config.use_pkce && !config.client_secret) {
            err = 'Must at least be a confidential client or use PKCE';
            return;
        }

        err = '';
        isLoading = true;

        config.scope = config.scope.trim();
        let res = await postProvider(config);
        if (res.ok) {
            success = true;
        } else {
            let body = await res.json();
            if (body.message.includes('InvalidCertificate')) {
                err = 'Insecure connection not allowed';
            } else {
                err = body.message;
            }
        }

        isLoading = false;
    }

    async function onSubmitLookup() {
        const valid = await validateFormLookup();
        if (!valid) {
            err = 'Invalid input';
            return;
        }

        err = '';
        isLoading = true;

        let res = await postProviderLookup(configLookup);
        if (res.ok) {
            const body = await res.json();
            config.issuer = body.issuer;
            config.authorization_endpoint = body.authorization_endpoint;
            config.danger_allow_http = body.danger_allow_http;
            config.danger_allow_insecure = body.danger_allow_insecure;
            config.token_endpoint = body.token_endpoint;
            config.userinfo_endpoint = body.userinfo_endpoint;
            config.token_auth_method_basic = body.token_auth_method_basic;
            config.use_pkce = body.use_pkce;
            config.scope = body.scope;
        } else {
            let body = await res.json();
            if (body.message.includes('InvalidCertificate')) {
                err = 'Insecure connection not allowed';
            } else {
                err = body.message;
            }
        }

        isLoading = false;
    }

    function resetValues() {
        configLookup = {
            issuer: '',
            danger_allow_http: false,
            danger_allow_insecure: false,
        };
        config = {
            issuer: '',
            danger_allow_http: false,
            danger_allow_insecure: false,
            authorization_endpoint: '',
            token_endpoint: '',
            userinfo_endpoint: '',
            use_pkce: true,
            scope: '',
        }
    }

    async function validateFormConfig() {
        try {
            await schemaConfig.validate(config, {abortEarly: false});
            formErrors = {};
            return true;
        } catch (err) {
            formErrors = extractFormErrors(err);
            return false;
        }
    }

    async function validateFormLookup() {
        try {
            await schemaLookup.validate(configLookup, {abortEarly: false});
            formErrors = {};
            return true;
        } catch (err) {
            formErrors = extractFormErrors(err);
            return false;
        }
    }

</script>

<ExpandContainer bind:idx bind:show={expandContainer}>
    <div class="header font-label" slot="header">
        ADD NEW AUTHENTICATION PROVIDER
    </div>

    <div class="container" slot="body">
        <div class="header">
            Setup
        </div>
        <div class="ml mb">
            <OptionSelect bind:value={mode} options={modes}/>
        </div>

        {#if !config.authorization_endpoint && isAuto}
            <Input
                    bind:value={configLookup.issuer}
                    bind:error={formErrors.issuer}
                    autocomplete="off"
                    placeholder="Issuer URL"
                    on:input={validateFormLookup}
                    width={inputWidth}
            >
                ISSUER URL
            </Input>

            <div class="header">
                Allow unencrypted HTTP lookup
            </div>
            <div class="ml">
                <Switch bind:selected={configLookup.danger_allow_http}/>
            </div>

            <div class="header">
                Allow insecure TLS certificates
            </div>
            <div class="ml mb">
                <Switch bind:selected={configLookup.danger_allow_insecure}/>
            </div>

            <Button on:click={onSubmitLookup} bind:isLoading level={1} width="6rem">
                LOOKUP
            </Button>
        {:else}
            {#if isAuto}
                <Input
                        bind:value={config.issuer}
                        bind:error={formErrors.issuer}
                        autocomplete="off"
                        placeholder="Issuer URL"
                        on:input={validateFormLookup}
                        width={inputWidth}
                        disabled
                >
                    ISSUER URL
                </Input>

                <div class="header">
                    Allow unencrypted HTTP lookup
                </div>
                <div class="ml">
                    <CheckIcon bind:check={config.danger_allow_http}/>
                </div>

                <div class="header">
                    Allow insecure TLS certificates
                </div>
                <div class="ml mb">
                    <CheckIcon bind:check={config.danger_allow_insecure}/>
                </div>

                <Input
                        bind:value={config.authorization_endpoint}
                        bind:error={formErrors.authorization_endpoint}
                        autocomplete="off"
                        placeholder="Authorization Endpoint"
                        on:input={validateFormLookup}
                        width={inputWidth}
                        disabled
                >
                    AUTHORIZATION ENDPOINT
                </Input>

                <Input
                        bind:value={config.token_endpoint}
                        bind:error={formErrors.token_endpoint}
                        autocomplete="off"
                        placeholder="Token Endpoint"
                        on:input={validateFormLookup}
                        width={inputWidth}
                        disabled
                >
                    TOKEN ENDPOINT
                </Input>

                <Input
                        bind:value={config.userinfo_endpoint}
                        bind:error={formErrors.userinfo_endpoint}
                        autocomplete="off"
                        placeholder="Userinfo Endpoint"
                        on:input={validateFormLookup}
                        width={inputWidth}
                        disabled
                >
                    USERINFO ENDPOINT
                </Input>

                <div class="header">
                    Use PKCE
                </div>
                <div class="ml">
                    <CheckIcon bind:check={config.use_pkce}/>
                </div>
            {:else}
                <Input
                        bind:value={config.issuer}
                        bind:error={formErrors.issuer}
                        autocomplete="off"
                        placeholder="Issuer URL"
                        on:input={validateFormConfig}
                        width={inputWidth}
                >
                    ISSUER URL
                </Input>

                <div class="header">
                    Allow unencrypted HTTP lookup
                </div>
                <div class="ml">
                    <Switch bind:selected={config.danger_allow_http}/>
                </div>

                <div class="header">
                    Allow insecure TLS certificates
                </div>
                <div class="ml mb">
                    <Switch bind:selected={config.danger_allow_insecure}/>
                </div>

                <Input
                        bind:value={config.authorization_endpoint}
                        bind:error={formErrors.authorization_endpoint}
                        autocomplete="off"
                        placeholder="Authorization Endpoint"
                        on:input={validateFormConfig}
                        width={inputWidth}
                >
                    AUTHORIZATION ENDPOINT
                </Input>

                <Input
                        bind:value={config.token_endpoint}
                        bind:error={formErrors.token_endpoint}
                        autocomplete="off"
                        placeholder="Token Endpoint"
                        on:input={validateFormConfig}
                        width={inputWidth}
                >
                    TOKEN ENDPOINT
                </Input>

                <Input
                        bind:value={config.userinfo_endpoint}
                        bind:error={formErrors.userinfo_endpoint}
                        autocomplete="off"
                        placeholder="Userinfo Endpoint"
                        on:input={validateFormConfig}
                        width={inputWidth}
                >
                    USERINFO ENDPOINT
                </Input>

                <div class="header">
                    Use PKCE
                </div>
                <div class="ml mb">
                    <Switch bind:selected={config.use_pkce}/>
                </div>
            {/if}

            <div class="desc">
                The scope the client should use when redirecting to the login.<br>
                Provide the values separated by space.
            </div>
            <Input
                    bind:value={config.scope}
                    bind:error={formErrors.scope}
                    autocomplete="off"
                    placeholder="openid profile email"
                    on:input={validateFormConfig}
                    width={inputWidth}
            >
                SCOPE
            </Input>

            <div class="desc">
                Client name for the Rauthy login form
            </div>
            <Input
                    bind:value={config.name}
                    bind:error={formErrors.name}
                    autocomplete="off"
                    placeholder="Client Name"
                    on:input={validateFormConfig}
                    width={inputWidth}
            >
                CLIENT NAME
            </Input>

            <div class="desc">
                Client ID given by the auth provider
            </div>
            <Input
                    bind:value={config.client_id}
                    bind:error={formErrors.client_id}
                    autocomplete="off"
                    placeholder="Client ID"
                    on:input={validateFormConfig}
                    width={inputWidth}
            >
                CLIENT ID
            </Input>

            <div class="desc">
                Client Secret given by the auth provider.<br>
                At least a client secret or PKCE is required.
            </div>
            <PasswordInput
                    bind:value={config.client_secret}
                    bind:error={formErrors.client_secret}
                    autocomplete="off"
                    placeholder="Client Secret"
                    on:input={validateFormConfig}
                    width={inputWidth}
            >
                CLIENT SECRET
            </PasswordInput>

            <Button on:click={onSubmitConfig} bind:isLoading level={1} width="6rem">
                SAVE
            </Button>

            <Button on:click={resetValues} bind:isLoading level={4} width="6rem">
                RESET
            </Button>
        {/if}

        {#if success}
            <div class="success">
                Success
            </div>
        {/if}

        {#if err}
            <div class="err">
                {err}
            </div>
        {/if}
    </div>
</ExpandContainer>

<style>
    .container {
        padding: 10px;
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

    .ml {
        margin-left: .5rem;
    }

    .mb {
        margin-bottom: .5rem;
    }

    .success {
        color: var(--col-ok);
    }
</style>
