<!-- @migration-task Error while migrating Svelte code: can't migrate `let t = {};` to `$state` because there's a variable named state.
     Rename the variable and try again or migrate by hand. -->
<script>
    import {onMount, tick} from "svelte";
    import {
        authorize,
        authorizeRefresh,
        postPasswordResetRequest,
        postProviderLogin
    } from "../../../utils/dataFetching.js";
    import * as yup from 'yup';
    import {
        extractFormErrors,
        formatDateFromTs,
        getQueryParams,
        saveCsrfToken,
        saveProviderToken,
    } from "../../../utils/helpers.js";
    import Button from "$lib/Button.svelte";
    import WebauthnRequest from "../../../components/webauthn/WebauthnRequest.svelte";
    import {scale} from 'svelte/transition';
    import Input from "$lib/inputs/Input.svelte";
    import PasswordInput from "$lib/inputs/PasswordInput.svelte";
    import BrowserCheck from "../../../components/BrowserCheck.svelte";
    import WithI18n from "$lib/WithI18n.svelte";
    import LangSelector from "$lib/LangSelector.svelte";
    import getPkce from "oauth-pkce";
    import {PKCE_VERIFIER_UPSTREAM} from "../../../utils/constants.js";
    import IconHome from "$lib/icons/IconHome.svelte";

    let t = {};

    let clientId;
    let clientName = '';
    let clientUri = '';
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
    // dummy data for testing
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
    let isRegOpen = false;

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

    $: if (passwordInput) {
        passwordInput.focus();
    }

    onMount(async () => {
        const data = window.document.getElementsByName('rauthy-data')[0].id.split('\n');
        clientName = data[0];
        clientUri = data[1];
        isRegOpen = data[2] === "true";

        const action = window.document.getElementsByName('rauthy-action')[0].id;
        if ('Refresh' === action) {
            refresh = true;
        } else if (action?.startsWith('MfaLogin ')) {
            existingMfaUser = action.replace('MfaLogin ', '');
        }

        csrf = window.document.getElementsByName('rauthy-csrf-token')[0].id;
        saveCsrfToken(csrf);

        // demo value for testing - only un-comment in local dev, not for production build
        // const provider_tpl = document.getElementsByTagName('template').namedItem('auth_providers').innerHTML || '[{"id": "z6rC5VvymQOev50Pwq0oL0KD", "name": "dev-test", "use_pkce": true}]';
        const providerTpl = document.getElementsByTagName('template').namedItem('auth_providers').innerHTML;
        if (providerTpl) {
            providers = JSON.parse(providerTpl);
        }

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
            if (formValues.password.length > 256) {
                formErrors.password = 'max 256';
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

    function providerLogin(id) {
        getPkce(64, (error, {challenge, verifier}) => {
            if (!error) {
                localStorage.setItem(PKCE_VERIFIER_UPSTREAM, verifier);
                providerLoginPkce(id, challenge);
            }
        });
    }

    async function providerLoginPkce(id, pkce_challenge) {
        let data = {
            email: formValues.email || null,
            client_id: clientId,
            redirect_uri: redirectUri,
            scopes: scopes,
            state: state,
            nonce: nonce,
            code_challenge: challenge,
            code_challenge_method: challengeMethod,
            provider_id: id,
            pkce_challenge,
        };
        let res = await postProviderLogin(data);
        if (res.ok) {
            const xsrfToken = await res.text();
            saveProviderToken(xsrfToken);

            window.location.href = res.headers.get('location');
        } else {
            let body = await res.json();
            err = body.message;
        }
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
        if (clientUri) {
            req.redirect_uri = encodeURI(clientUri);
        }

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
    <title>Login {clientName || clientId}</title>
</svelte:head>

<BrowserCheck>
    <WithI18n bind:t content="authorize">
        <div class="container">
            <div class="head">
                <div class="logo">
                    {#if clientId}
                        <img src="{`/auth/v1/clients/${clientId}/logo`}" alt="No Logo Available"/>
                    {/if}
                </div>
                {#if clientUri}
                    <a class="home" href={clientUri}>
                        <IconHome opacity={0.5}/>
                    </a>
                {/if}
            </div>

            <div class="name">
                <h2>{clientName || clientId}</h2>
            </div>

            {#if webauthnData}
                <WebauthnRequest
                        {t}
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
                            error={formErrors.password}
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
                            <div class="flex-inline">
                                <img src="{`/auth/v1/providers/${provider.id}/img`}" alt="" width="20" height="20"/>
                                <span class="providerName">{provider.name}</span>
                            </div>
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

            {#if isRegOpen && !clientMfaForce && !showResetRequest && !tooManyRequests}
                {#if clientUri}
                    <a class="reg" href="/auth/v1/users/register?redirect_uri={clientUri}" target="_blank">
                        {t.signUp}
                    </a>
                {:else}
                    <a class="reg" href="/auth/v1/users/register" target="_blank">
                        {t.signUp}
                    </a>
                {/if}
            {/if}

            {#if clientMfaForce}
                <div class="btn flex-col">
                    <Button on:click={() => window.location.href = '/auth/v1/account'}>
                        ACCOUNT
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
        margin: -10px 10px 5px 5px;
        color: var(--col-err)
    }

    .flex-col {
        display: flex;
        flex-direction: column;
    }

    .flex-inline {
        display: inline-flex;
        align-items: center;
        gap: .5rem;
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
    }

    .home {
        margin-right: 5px;
        cursor: pointer;
    }

    .name {
        margin: -10px 5px 0 5px;
    }

    .logo {
        margin: 0 .25rem;
        width: 84px;
        height: 84px;
        display: flex;
        justify-content: center;
        align-items: center;
    }

    .providers {
        margin-top: .66rem;
    }

    .providerName {
        /*margin-bottom: -14px;*/
        margin-top: 4px;
    }

    .reg {
        margin-left: 5px;
    }

    .success {
        margin: 0 5px;
        color: var(--col-ok);
    }
</style>
