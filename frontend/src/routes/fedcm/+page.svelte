<script>
    import {onMount} from "svelte";
    import {getKey} from "$lib/utils/helpers";
    import Button from "$lib/Button.svelte";
    import Input from "$lib/inputs/Input.svelte";
    import * as yup from "yup";
    import {REGEX_URI} from "../../utils/constants.js";
    import {extractFormErrors} from "../../utils/helpers";
    import CheckIcon from "$lib/CheckIcon.svelte";
    import {getFedCMStatus} from "../../utils/dataFetching.js";
    import Main from "$lib5/Main.svelte";
    import ContentCenter from "$lib5/ContentCenter.svelte";

    let configUrl = $state('');
    let isSupported = $state(false);
    let isLoggedIn = $state(false);
    let credentials = $state('');
    let credentialType = $state('');

    let formValues = $state({
        clientId: 'fedcm',
        configUrl: 'any',
    })
    let formErrors = $state({});
    const schema = yup.object().shape({
        clientId: yup.string().required('Client ID is required').trim().matches(REGEX_URI, "Must be URL safe"),
        configUrl: yup.string().nullable().trim().matches(REGEX_URI, "Must be URL safe"),
    });

    onMount(async () => {
        configUrl = `${window.location.origin}/auth/v1/fed_cm/config`;
        formValues.clientId = `${window.location.origin}/auth/v1/fed_cm/client_config`;

        let res = await getFedCMStatus();
        if (res.ok) {
            console.log('FedCM status is: logged-in');
            isLoggedIn = true;
        } else {
            console.log('FedCM status is: logged-out');
        }

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
                    // context: "signin",
                    mode: "button", // button mode will actually open the `login_url` if `logged-out`
                    providers: [{
                        configURL: configUrl,
                        clientId: clientId,
                        nonce: getKey(48),
                    }]
                }
            });

            console.log(creds);
            credentialType = creds.type;
            credentials = creds.token;
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

<Main>
    <ContentCenter>
        <div class="flex-col">
            <h1>FedCM Testing</h1>
            <div class="row">
                <div><b>FedCM supported:</b></div>
                <div class="check">
                    <CheckIcon check={isSupported}/>
                </div>
            </div>
            {#if !isSupported}
                <br>-> enable idp registration:<br>
                <code>chrome://flags/#fedcm-idp-registration</code>
            {/if}

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
                <div class="row">
                    <b>Logged In:</b>
                    <div class="check">
                        <CheckIcon check={isLoggedIn}/>
                    </div>
                </div>
            {/if}
        </div>
    </ContentCenter>
</Main>

{#if credentials}
    <div class="row">
        <b>Credential Type:</b>
        <span>{credentialType}</span>
    </div>
    <div class="token">
        {credentials}
    </div>
{/if}

<style>
    .check {
        margin-bottom: -.3rem;
    }

    .row {
        display: inline-flex;
        align-items: center;
        gap: 0.25rem;
    }

    .token {
        word-wrap: break-word;
        width: 20rem;
    }
</style>
