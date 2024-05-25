<script>
    import {onMount} from "svelte";
    import {getKey} from "$lib/utils/helpers.js";
    import Button from "$lib/Button.svelte";
    import Input from "$lib/inputs/Input.svelte";
    import * as yup from "yup";
    import {REGEX_CLIENT_ID, REGEX_CLIENT_NAME, REGEX_URI} from "../../utils/constants.js";
    import {extractFormErrors} from "../../utils/helpers.js";
    import CheckIcon from "$lib/CheckIcon.svelte";

    let configUrl = '';
    let isSupported = false;
    let isLoggedIn = false;
    let credentials = '';

    $: console.log('built config url: ' + configUrl);

    let formValues = {
        clientId: 'fedcm',
        configUrl: 'any',
    }
    let formErrors = {};
    const schema = yup.object().shape({
        clientId: yup.string().required('Client ID is required').trim().matches(REGEX_URI, "Must be URL safe"),
        configUrl: yup.string().nullable().trim().matches(REGEX_URI, "Must be URL safe"),
    });

    onMount(async () => {
        configUrl = `${window.location.origin}/auth/v1/fed_cm/config`;

        if (window["IdentityProvider"] && // Is FedCM available?
            IdentityProvider.register != undefined  // Is the IdP Registration API available?
        ) {
            console.log('FedCM is supported');
            isSupported = true;
        } else {
            console.error('FedCM is not supported');
            return;
        }
    });

    async function login() {
        let isValid = await validateForm();
        if (!isValid) {
            return;
        }

        let configUrl = formValues.configUrl || undefined;
        let clientId = formValues.clientId;
        console.log('using credentials get values: configUrl: ' + configUrl + ' / clientId: ' + clientId);

        try {
            let creds = await navigator.credentials.get({
                identity: {
                    context: "signin",
                    providers: [{
                        configURL: configUrl,
                        // TODO configurable via input
                        clientId: clientId,
                        nonce: getKey(48),
                    }]
                }
            })
            console.log(creds);
            console.log(creds.id);
            console.log(creds.type);
            console.log(creds.token);
            credentials = creds.token || 'logged in';
            isLoggedIn = true;
        } catch (err) {
            console.error('FedCM credentials error: ' + err);
        }
    }

    function register() {
        IdentityProvider.register(configUrl);
    }

    async function validateForm() {
        try {
            await schema.validate(formValues, {abortEarly: false});
            formErrors = {};
            return true;
        } catch (err) {
            formErrors = extractFormErrors(err);
            return false;
        }
    }
</script>

<h1>FedCM Testing</h1>
<p>
    <b>FedCM supported:</b>
    <CheckIcon check={isSupported}/>
    {#if !isSupported}
        <br>-> enable idp registration:<br>
        <code>chrome://flags/#fedcm-idp-registration</code>
    {/if}
</p>

{#if isSupported}
    <p>
        <Button on:click={register} level={3}>Register IdP</Button>
    </p>

    <p>
        <Input
                bind:value={formValues.clientId}
                bind:error={formErrors.clientId}
                autocomplete="off"
                placeholder="Client Id"
                on:input={validateForm}
        >
            CLIENT ID
        </Input>
        <Input
                bind:value={formValues.configUrl}
                bind:error={formErrors.configUrl}
                autocomplete="off"
                placeholder="Config URL"
                on:input={validateForm}
        >
            CONFIG URL
        </Input>
        <Button on:click={login}>Login</Button>
    </p>
    <p>
        <b>Logged In:</b>
        <CheckIcon check={isLoggedIn}/>
    </p>
{/if}

{#if credentials}
    <p>{credentials}</p>
{/if}
