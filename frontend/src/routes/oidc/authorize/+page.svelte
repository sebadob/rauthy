<script>
    import {onMount, tick} from "svelte";
    import {authorize, authorizeRefresh, getClientLogo, postPasswordResetRequest} from "../../../utils/dataFetching.js";
    import * as yup from 'yup';
    import {extractFormErrors, formatDateFromTs, getQueryParams, saveCsrfToken} from "../../../utils/helpers.js";
    import Button from "$lib/Button.svelte";
    import WebauthnRequest from "../../../components/webauthn/WebauthnRequest.svelte";
    import {scale} from 'svelte/transition';
    import Input from "$lib/inputs/Input.svelte";
    import PasswordInput from "$lib/inputs/PasswordInput.svelte";
    import BrowserCheck from "../../../components/BrowserCheck.svelte";
    import WithI18n from "$lib/WithI18n.svelte";
    import LangSelector from "$lib/LangSelector.svelte";

    let t = {};

    let clientId;
    let clientLogo;
    let clientName = '';
    let redirectUri = '';
    let nonce = '';
    let scopes = [];
    let passwordInput;

    let state;
    let challenge;
    let challengeMethod;
    let csrf = '';
    let refresh = false;
    let existingMfaUser;
    let providers = [];
    // let webauthnData = {
    // 	code: "asdjknfasdjklfnasdlkjf",
    //   header_csrf: "askjdfgnsdfjklgn",
    //   user_id: "asdkfjnasdjkn",
    //   email: "admin@localhost.de",
    //   exp: 60,
    // };
    let webauthnData;

    let isLoading = false;
    let err = '';
    let needsPassword = false;
    let clientMfaForce = false;
    let showReset = false;
    let showResetRequest = false;
    let emailSuccess = false;
    let tooManyRequests = false;
    let emailAfterSubmit = '';

    let formValues = {email: '', password: ''};
    let formErrors = {};

    let schema = {};
    $: if (t) {
        schema = yup.object().shape({
            email: yup.string().required(t.emailRequired).email(t.emailBadFormat),
        });
    }

    $: if (refresh && clientId?.length > 0 && redirectUri?.length > 0) {
        isLoading = true
        const req = {
            client_id: clientId,
            redirect_uri: redirectUri,
            state: state,
            code_challenge: challenge,
            code_challenge_method: challengeMethod,
            nonce: nonce,
            scopes
        };

        // make sure loading has been set to prevent a chrome bug with too fast redirect inside authorizeRefresh
        tick().then(() => authorizeRefresh(req, csrf).then(res => handleAuthRes(res)));
    }

    $: if (existingMfaUser) {
        formValues.email = existingMfaUser;
    }

    $: if (emailSuccess) {
        setTimeout(() => {
            emailSuccess = false;
            showReset = false;
            showResetRequest = false;
        }, 3000);
    }

    $: if (clientId) {
        fetchClientLogo(clientId);
    }

    $: if (passwordInput) {
        passwordInput.focus();
    }

    onMount(async () => {
        clientName = window.document.getElementsByName('rauthy-data')[0].id

        const action = window.document.getElementsByName('rauthy-action')[0].id
        if ('Refresh' === action) {
            refresh = true;
        } else if (action?.startsWith('MfaLogin ')) {
            existingMfaUser = action.replace('MfaLogin ', '');
        }

        csrf = window.document.getElementsByName('rauthy-csrf-token')[0].id
        saveCsrfToken(csrf);

        // demo value for testing
        // providers = JSON.parse('[{"id": "z6rC5VvymQOev50Pwq0oL0KD", "name": "dev-test"},{"id": "z6rC5VvymQOev50Pwq0oL0Kd", "name": "dev-test2"}]');
        providers = JSON.parse(document.getElementsByTagName('template').namedItem('auth_providers').innerHTML);

        const params = getQueryParams();
        clientId = params.client_id;
        redirectUri = params.redirect_uri;
        nonce = params.nonce;
        scopes = params.scope.split(' ');
        state = params.state;
        challenge = params.code_challenge;
        challengeMethod = params.code_challenge_method;

        if (params.login_hint) {
            formValues.email = params.login_hint;
        }
    })

    async function fetchClientLogo(id) {
        let res = await getClientLogo(id);
        if (res.ok) {
            clientLogo = await res.text();
        } else {
            let body = await res.json();
            err = body.message;
        }
    }

    function handleShowReset() {
        err = '';
        showReset = true;
        formValues.password = '';
    }

    async function onSubmit() {
        err = '';

        try {
            await schema.validate(formValues, {abortEarly: false});
            formErrors = {};
        } catch (err) {
            formErrors = extractFormErrors(err);
            return;
        }

        const req = {
            email: formValues.email,
            client_id: clientId,
            redirect_uri: redirectUri,
            state: state,
            code_challenge: challenge,
            code_challenge_method: challengeMethod,
            nonce: nonce,
            scopes,
        };

        if (needsPassword && formValues.email !== existingMfaUser) {
            if (!formValues.password) {
                formErrors.password = t.passwordRequired;
                return;
            }
            req.password = formValues.password;
        }

        isLoading = true;
        let res = await authorize(req, csrf);
        await handleAuthRes(res);
    }

    async function handleAuthRes(res) {
        if (res.status === 202) {
            // -> all good
            window.location.replace(res.headers.get('location'));
        } else if (res.status === 200) {
            // -> all good, but needs additional passkey validation
            err = '';
            webauthnData = await res.json();
        } else if (res.status === 406) {
            // 406 -> client forces MFA while the user has none
            err = t.clientForceMfa;
            clientMfaForce = true;
        } else if (res.status === 429) {
            // 429 -> too many failed logins
            let notBefore = Number.parseInt(res.headers.get('x-retry-not-before'));
            let nbfDate = formatDateFromTs(notBefore);
            let diff = notBefore * 1000 - new Date().getTime();

            tooManyRequests = true;
            err = `${t.http429} ${nbfDate}`;

            formValues.email = '';
            formValues.password = '';
            needsPassword = false;

            setTimeout(() => {
                tooManyRequests = false;
                err = '';
            }, diff);
        } else if (!needsPassword) {
            // this will happen always if the user does the first try with a password-only account
            // the good thing about this is, that it is a prevention against autofill passwords from the browser
            needsPassword = true;
            emailAfterSubmit = formValues.email;
        } else {
            err = t.invalidCredentials;
            showResetRequest = true;
        }
        isLoading = false;
    }

    function onEmailInput() {
        // this will basically remove the password input again if the user was asked to provide
        // a password and afterward changes his email again
        if (needsPassword && emailAfterSubmit !== formValues.email) {
            needsPassword = false;
            formValues.password = '';
            err = '';
        }
    }

    async function providerLogin(id) {
        console.error('TODO - providerLogin - id: ' + id);
    }

    function onWebauthnError() {
        // If there is any error with the key, the user should start a new login process
        webauthnData = undefined;
    }

    function onWebauthnSuccess(res) {
        if (res) {
            window.location.replace(res.loc);
        }
    }

    async function requestReset() {
        try {
            await schema.validate(formValues, {abortEarly: false});
            formErrors = {};
        } catch (err) {
            formErrors = extractFormErrors(err);
            return;
        }

        let req = {
            email: formValues.email,
        };

        isLoading = true;

        let res = await postPasswordResetRequest(req);
        if (res.ok) {
            emailSuccess = true;
        } else {
            let body = await res.json();
            err = body.message;
        }

        isLoading = false;
    }

</script>

<svelte:head>
    {#if clientName}
        <title>Login {clientName}</title>
    {:else}
        <title>Login {clientId}</title>
    {/if}
</svelte:head>

<BrowserCheck>
    <WithI18n bind:t content="authorize">
        <div class="container">
            <div class="head">
                <div class="logo">
                    {#if clientLogo}
                        <img class="logo" src="{clientLogo}" alt="Client Logo"/>
                    {/if}
                </div>
            </div>

            <div class="name">
                <h2>{clientName}</h2>
            </div>

            {#if webauthnData}
                <WebauthnRequest
                        bind:t
                        bind:data={webauthnData}
                        onSuccess={onWebauthnSuccess}
                        onError={onWebauthnError}
                />
            {/if}

            {#if !clientMfaForce}
                <Input
                        type="email"
                        name="rauthyEmail"
                        bind:value={formValues.email}
                        bind:error={formErrors.email}
                        autocomplete="email"
                        placeholder={t.email}
                        disabled={tooManyRequests || clientMfaForce}
                        on:enter={onSubmit}
                        on:input={onEmailInput}
                >
                    {t.email?.toUpperCase()}
                </Input>

                {#if needsPassword && existingMfaUser !== formValues.email && !showReset}
                    <PasswordInput
                            bind:bindThis={passwordInput}
                            name="rauthyPassword"
                            bind:value={formValues.password}
                            bind:error={formErrors.password}
                            autocomplete="current-password"
                            placeholder={t.password}
                            disabled={tooManyRequests || clientMfaForce}
                            on:enter={onSubmit}
                    >
                        {t.password?.toUpperCase()}
                    </PasswordInput>

                    {#if showResetRequest && !tooManyRequests}
                        <div
                                role="button"
                                tabindex="0"
                                class="forgotten"
                                transition:scale|global
                                on:click={handleShowReset}
                                on:keypress={handleShowReset}
                        >
                            {t.passwordForgotten}
                        </div>
                    {/if}
                {/if}

                {#if !tooManyRequests && !clientMfaForce}
                    {#if showReset}
                        <div class="btn flex-col">
                            <Button on:click={requestReset}>
                                {t.passwordRequest?.toUpperCase()}
                            </Button>
                        </div>
                    {:else}
                        <div class="btn flex-col">
                            <Button on:click={onSubmit} bind:isLoading>
                                {t.login?.toUpperCase()}
                            </Button>
                        </div>
                    {/if}
                {/if}
            {/if}

            {#if providers}
                <div class="providers flex-col">
                    {#each providers as provider (provider.id)}
                        <Button on:click={() => providerLogin(provider.id)} level={3}>
                            {provider.name}
                        </Button>
                    {/each}
                </div>
            {/if}

            {#if err}
                <div class="errMsg errMsgApi">
                    {err}
                </div>
            {/if}

            {#if emailSuccess}
                <div class="success">
                    {t.emailSentMsg}
                </div>
            {/if}

            {#if clientMfaForce}
                <div class="btn flex-col">
                    <Button on:click={() => window.location.href = '/auth/v1/account'}>
                        ACCOUNT LOGIN
                    </Button>
                </div>
            {/if}
        </div>

        <LangSelector absolute/>
    </WithI18n>
</BrowserCheck>

<style>
    .btn {
        margin: 5px 0;
        display: flex;
    }

    .container {
        display: flex;
        flex-direction: column;
        justify-content: center;
        max-width: 19rem;
        padding: 20px;
        border: 1px solid var(--col-gmid);
        border-radius: 5px;
        box-shadow: 5px 5px 5px rgba(128, 128, 128, .1);
    }

    .errMsg {
        max-width: 15rem;
        margin: -5px 10px 0 5px;
        color: var(--col-err)
    }

    .flex-col {
        display: flex;
        flex-direction: column;
    }

    .forgotten {
        margin: 0 5px;
        cursor: pointer;
    }

    .forgotten:hover {
        color: var(--col-ok);
    }

    .head {
        display: flex;
        justify-content: space-between;
        padding-right: 35px;
    }

    .name {
        margin: -10px 5px 0 5px;
    }

    .logo {
        width: 84px;
        height: 84px;
    }

    .providers {
        margin-top: .66rem;
    }

    .success {
        color: var(--col-ok);
    }
</style>
