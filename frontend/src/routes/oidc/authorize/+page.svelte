<script lang="ts">
    import {
        formatDateFromTs,
        saveCsrfToken,
        saveProviderToken,
    } from "$utils/helpers";
    import Button from "$lib5/button/Button.svelte";
    import WebauthnRequest from "$lib5/WebauthnRequest.svelte";
    import Input from "$lib5/form/Input.svelte";
    import LangSelector from "$lib5/LangSelector.svelte";
    import getPkce from "oauth-pkce";
    import {
        PKCE_VERIFIER_UPSTREAM,
        TPL_AUTH_PROVIDERS, TPL_CLIENT_LOGO_UPDATED,
        TPL_CLIENT_NAME,
        TPL_CLIENT_URL,
        TPL_CSRF_TOKEN,
        TPL_IS_REG_OPEN,
        TPL_LOGIN_ACTION
    } from "$utils/constants.js";
    import IconHome from "$icons/IconHome.svelte";
    import Main from "$lib5/Main.svelte";
    import ContentCenter from "$lib5/ContentCenter.svelte";
    import {useI18n} from "$state/i18n.svelte";
    import Template from "$lib5/Template.svelte";
    import {useParam} from "$state/param.svelte";
    import ThemeSwitch from "$lib5/ThemeSwitch.svelte";
    import type {AuthProviderTemplate} from "$api/templates/AuthProvider.ts";
    import InputPassword from "$lib5/form/InputPassword.svelte";
    import type {MfaPurpose, WebauthnAdditionalData} from "$webauthn/types.ts";
    import {fetchPost, type IResponse} from "$api/fetch.ts";
    import {useIsDev} from "$state/is_dev.svelte.ts";
    import type {
        CodeChallengeMethod,
        LoginRefreshRequest,
        LoginRequest,
        RequestResetRequest,
        WebauthnLoginResponse
    } from "$api/types/authorize.ts";
    import Form from "$lib5/form/Form.svelte";
    import ButtonAuthProvider from "$lib5/ButtonAuthProvider.svelte";
    import {onMount} from "svelte";
    import type {SessionInfoResponse} from "$api/types/session.ts";
    import ClientLogo from "$lib5/ClientLogo.svelte";
    import type {ProviderLoginRequest} from "$api/types/auth_provider.ts";

    const inputWidth = "18rem";

    let t = useI18n();
    let isDev = useIsDev().get();

    let authorizeUrl = $derived(isDev ? '/auth/v1/dev/authorize' : '/auth/v1/oidc/authorize');

    let clientId = useParam('client_id').get();
    let clientName = $state('');
    // we can't use undefined to avoid a JSON error in the Template component
    let clientLogoUpdated = $state(-1);
    let clientUri = $state(isDev ? '/auth/v1' : '');
    let redirectUri = useParam('redirect_uri').get();
    let nonce = useParam('nonce').get();
    let scopes = useParam('scope').get()?.split(' ') || [];

    let refEmail: undefined | HTMLInputElement = $state();
    let refPassword: undefined | HTMLInputElement = $state();

    let stateParam = useParam('state').get();
    let challenge = useParam('code_challenge').get();
    let challengeMethod: CodeChallengeMethod = useParam('code_challenge_method').get() as CodeChallengeMethod;
    let refresh = false;
    let existingMfaUser: undefined | string = $state();
    let providers: AuthProviderTemplate[] = $state([]);
    let mfaPurpose: undefined | MfaPurpose = $state();

    let isLoading = $state(false);
    // let err = $state('some error message that it longer over multiple lines');
    let err = $state('');
    let loginAction = $state('');
    let csrfToken = $state('');
    let needsPassword = $state(false);
    let clientMfaForce = $state(false);
    let showReset = $state(false);
    let showResetRequest = $state(false);
    let emailSuccess = $state(false);
    let tooManyRequests = $state(false);
    let emailAfterSubmit = $state('');
    let isRegOpen = $state(false);

    let email = $state(useParam('login_hint').get() || '');
    let password = $state('');
    let userId = $state('');
    let showPasswordInput = $derived(needsPassword && existingMfaUser !== email && !showReset);

    onMount(() => {
        if (!needsPassword) {
            refEmail?.focus();
        }
    });

    $effect(() => {
        if (refresh && clientId?.length || 0 > 0 && redirectUri?.length || 0 > 0) {
            onRefresh();
        }
    })

    $effect(() => {
        if (emailSuccess) {
            setTimeout(() => {
                emailSuccess = false;
                showReset = false;
                showResetRequest = false;
            }, 3000);
        }
    });

    $effect(() => {
        refPassword?.focus();
    });

    $effect(() => {
        if ('Refresh' === loginAction) {
            refresh = true;
        } else if (loginAction?.startsWith('MfaLogin ')) {
            let mfaUser = loginAction.replace('MfaLogin ', '');
            email = mfaUser;
            existingMfaUser = mfaUser;
        }
    });

    $effect(() => {
        if (isDev) {
            // Make sure to create a session manually during dev.
            // In prod, it will be handled automatically during the GET already.
            createSessionDev();
        } else if (csrfToken) {
            saveCsrfToken(csrfToken);
        }
    });

    async function createSessionDev() {
        let res = await fetchPost<SessionInfoResponse>('/auth/v1/oidc/session');
        if (res.body?.csrf_token) {
            saveCsrfToken(res.body.csrf_token)
        } else {
            console.error(res.error);
        }
    }

    function handleShowReset() {
        err = '';
        showReset = true;
        password = '';
    }

    async function onRefresh() {
        if (!clientId) {
            console.error('clientId is undefined');
            return;
        }
        if (!redirectUri) {
            console.error('redirectUri is undefined');
            return;
        }

        isLoading = true

        const payload: LoginRefreshRequest = {
            client_id: clientId,
            redirect_uri: redirectUri,
            state: stateParam,
            nonce: nonce,
            scopes
        };
        if (challenge && challengeMethod && (challengeMethod === 'plain' || challengeMethod === 'S256')) {
            payload.code_challenge = challenge;
            payload.code_challenge_method = challengeMethod;
        }

        let res = await fetchPost<undefined | WebauthnLoginResponse>('/auth/v1/oidc/authorize/refresh', payload);
        await handleAuthRes(res);
    }

    async function onSubmit(form?: HTMLFormElement, params?: URLSearchParams) {
        err = '';

        if (!clientId) {
            console.error('clientId is undefined');
            return;
        }
        if (!redirectUri) {
            console.error('redirectUri is undefined');
            return;
        }

        const payload: LoginRequest = {
            email,
            client_id: clientId,
            redirect_uri: redirectUri,
            state: stateParam,
            nonce: nonce,
            scopes,
        };
        if (challenge && challengeMethod && (challengeMethod === 'plain' || challengeMethod === 'S256')) {
            payload.code_challenge = challenge;
            payload.code_challenge_method = challengeMethod;
        }

        if (needsPassword && email !== existingMfaUser) {
            if (!password) {
                err = t.authorize.passwordRequired;
                return;
            }
            if (password.length > 256) {
                err = 'max 256';
                return;
            }
            payload.password = password;
        }

        isLoading = true;

        let url = '/auth/v1/oidc/authorize';
        if (useIsDev().get()) {
            url = '/auth/v1/dev/authorize';
        }

        let res = await fetchPost<undefined | WebauthnLoginResponse>(url, payload, 'json', 'noRedirect');
        await handleAuthRes(res);
    }

    async function handleAuthRes(res: IResponse<undefined | WebauthnLoginResponse>) {
        if (res.status === 202) {
            // -> all good
            let loc = res.headers.get('location');
            if (!loc) {
                console.error('location header missing');
                return;
            }
            window.location.replace(loc);
        } else if (res.status === 200) {
            // -> all good, but needs additional passkey validation
            err = '';
            let body = res.body;
            if (body && 'user_id' in body && 'code' in body) {
                // TODO is there a nicer way of making TS happy with type checking?
                userId = body.user_id as string;
                mfaPurpose = {Login: body.code as string};
            } else {
                console.error('did not receive a proper WebauthnLoginResponse after HTTP200');
            }
        } else if (res.status === 406) {
            // 406 -> client forces MFA while the user has none
            err = t.authorize.clientForceMfa;
            clientMfaForce = true;
        } else if (res.status === 429) {
            // 429 -> too many failed logins
            let nbf = res.headers.get('x-retry-not-before');
            if (!nbf) {
                console.error('x-retry-not-before header missing');
                return;
            }
            let notBefore = Number.parseInt(nbf);
            let nbfDate = formatDateFromTs(notBefore);
            let diff = notBefore * 1000 - new Date().getTime();

            tooManyRequests = true;
            err = `${t.authorize.http429} ${nbfDate}`;

            email = '';
            password = '';
            needsPassword = false;

            setTimeout(() => {
                tooManyRequests = false;
                err = '';
            }, diff);
        } else if (!needsPassword) {
            // this will happen always if the user does the first try with a password-only account
            // the good thing about this is, that it is a prevention against autofill passwords from the browser
            needsPassword = true;
            emailAfterSubmit = email;
        } else {
            err = t.authorize.invalidCredentials;
            showResetRequest = true;
        }
        isLoading = false;
    }

    function onEmailInput() {
        // this will basically remove the password input again if the user was asked to provide
        // a password and afterward changes his email again
        if (needsPassword && emailAfterSubmit !== email) {
            needsPassword = false;
            password = '';
            err = '';
        }
    }

    function providerLogin(id: string) {
        getPkce(64, (error, {challenge, verifier}) => {
            if (!error) {
                localStorage.setItem(PKCE_VERIFIER_UPSTREAM, verifier);
                providerLoginPkce(id, challenge);
            }
        });
    }

    async function providerLoginPkce(id: string, pkce_challenge: string) {
        if (!clientId) {
            console.error('clientId is undefined');
            return;
        }
        if (!redirectUri) {
            console.error('redirectUri is undefined');
            return;
        }

        let payload: ProviderLoginRequest = {
            email: email || undefined,
            client_id: clientId,
            redirect_uri: redirectUri,
            scopes: scopes,
            state: stateParam,
            nonce: nonce,
            code_challenge: challenge,
            code_challenge_method: challengeMethod,
            provider_id: id,
            pkce_challenge,
        };

        let res = await fetchPost<string>('/auth/v1/providers/login', payload);
        if (res.text) {
            saveProviderToken(res.text);

            let loc = res.headers.get('location');
            if (!loc) {
                console.error('no location header set for provider login');
                return;
            }
            window.location.href = loc;
        } else {
            err = res.error?.message || 'Error';
        }
    }

    function onWebauthnError(error: string) {
        // If there is any error with the key, the user should start a new login process
        mfaPurpose = undefined;
        err = error;
    }

    function onWebauthnSuccess(data?: WebauthnAdditionalData) {
        if (data && 'loc' in data) {
            window.location.replace(data.loc as string);
        }
    }

    async function requestReset() {
        let payload: RequestResetRequest = {email};
        if (clientUri) {
            payload.redirect_uri = encodeURI(clientUri);
        }

        isLoading = true;

        let res = await fetchPost('/auth/v1/users/request_reset', payload);
        if (res.error) {
            err = res.error.message;
        } else {
            emailSuccess = true;
        }

        isLoading = false;
    }

</script>

<svelte:head>
    <title>Login {clientName || clientId}</title>
</svelte:head>

<Template id={TPL_AUTH_PROVIDERS} bind:value={providers}/>
<Template id={TPL_CLIENT_NAME} bind:value={clientName}/>
<Template id={TPL_CLIENT_URL} bind:value={clientUri}/>
<Template id={TPL_CLIENT_LOGO_UPDATED} bind:value={clientLogoUpdated}/>
<Template id={TPL_CSRF_TOKEN} bind:value={csrfToken}/>
<Template id={TPL_LOGIN_ACTION} bind:value={loginAction}/>
<Template id={TPL_IS_REG_OPEN} bind:value={isRegOpen}/>

<Main>
    <div class="outer">
        <ContentCenter>
            <div class="container">
                <div class="head">
                    {#if clientId}
                        <ClientLogo {clientId} updated={clientLogoUpdated > -1 ? clientLogoUpdated : undefined}/>
                    {/if}
                    {#if clientUri}
                        <a class="home" href={clientUri} aria-label="Client Home Page">
                            <IconHome color="hsla(var(--text) / .4)"/>
                        </a>
                    {/if}
                </div>

                <div class="name">
                    <h2>{clientName || clientId}</h2>
                </div>

                {#if mfaPurpose && userId}
                    <!--
                    TODO we could pass in an optional loginCodeExp and make sure
                    it fits inside the exp returned inside the WebauthnRequest later on
                    to output proper logs in case of misconfiguration.
                    Another approach would be to check this in the backend and emit warning logs.
                    -->
                    <WebauthnRequest
                            {userId}
                            purpose={mfaPurpose}
                            onSuccess={onWebauthnSuccess}
                            onError={onWebauthnError}
                    />
                {/if}

                {#if !clientMfaForce}
                    <Form action={authorizeUrl} {onSubmit}>
                        <div class:emailMinHeight={!showPasswordInput}>
                            <Input
                                    bind:ref={refEmail}
                                    typ="email"
                                    name="email"
                                    bind:value={email}
                                    autocomplete="email"
                                    label={t.common.email}
                                    placeholder={t.common.email}
                                    errMsg={t.authorize.validEmail}
                                    disabled={tooManyRequests || clientMfaForce}
                                    onInput={onEmailInput}
                                    width={inputWidth}
                                    required
                            />
                        </div>

                        {#if showPasswordInput}
                            <InputPassword
                                    bind:ref={refPassword}
                                    name="password"
                                    bind:value={password}
                                    autocomplete="current-password"
                                    label={t.common.password}
                                    placeholder={t.common.password}
                                    maxLength={256}
                                    disabled={tooManyRequests || clientMfaForce}
                                    width={inputWidth}
                                    required
                            />

                            {#if showResetRequest && !tooManyRequests}
                                <div class="forgotten">
                                    <Button invisible onclick={handleShowReset}>
                                        {t.authorize.passwordForgotten}
                                    </Button>
                                </div>
                            {/if}
                        {/if}

                        {#if !tooManyRequests && !clientMfaForce}
                            {#if showReset}
                                <div class="btn flex-col">
                                    <Button onclick={requestReset}>
                                        {t.authorize.passwordRequest}
                                    </Button>
                                </div>
                            {:else}
                                <div class="btn flex-col">
                                    <Button type="submit" {isLoading}>
                                        {t.authorize.login}
                                    </Button>
                                </div>
                            {/if}
                        {/if}
                    </Form>

                    {#if isRegOpen && !showResetRequest && !tooManyRequests}
                        {#if clientUri}
                            <a class="reg" href="/auth/v1/users/register?redirect_uri={clientUri}" target="_blank">
                                {t.authorize.signUp}
                            </a>
                        {:else}
                            <a class="reg" href="/auth/v1/users/register" target="_blank">
                                {t.authorize.signUp}
                            </a>
                        {/if}
                    {/if}
                {/if}

                {#if err}
                    <div class="errMsg">
                        {err}
                    </div>
                {/if}

                {#if emailSuccess}
                    <div class="success">
                        {t.authorize.emailSentMsg}
                    </div>
                {/if}

                {#if clientMfaForce}
                    <div class="btn flex-col">
                        <Button onclick={() => window.location.href = '/auth/v1/account'}>
                            Account
                        </Button>
                    </div>
                {/if}

                <!--{#if !clientMfaForce && providers.length > 0}-->
                <div class="providers flex-col">
                    <div class="providersSeparator">
                        <div class="separator"></div>
                        <div class="loginWith">
                            <div>
                                {t.authorize.orLoginWith}
                            </div>
                        </div>
                    </div>
                    {#each providers as provider (provider.id)}
                        <ButtonAuthProvider
                                ariaLabel={`Login: ${provider.name}`}
                                {provider}
                                onclick={providerLogin}
                        />
                    {/each}
                </div>
                <!--{/if}-->
            </div>

            <ThemeSwitch absolute/>
            <LangSelector absolute/>
        </ContentCenter>
    </div>
</Main>

<style>
    .btn {
        margin: 5px 0;
        display: flex;
    }

    .outer {
        width: 100dvw;
        height: 100dvh;
        background: hsla(var(--bg-high) / .25);
    }

    .container {
        display: flex;
        flex-direction: column;
        justify-content: center;
        max-width: 21rem;
        padding: 20px;
        border-radius: 5px;
        border: 1px solid hsl(var(--bg-high));
        background: hsl(var(--bg));
    }

    .emailMinHeight {
        min-height: 4.8rem;
    }

    .errMsg {
        margin: .5rem 0;
        max-width: 18rem;
        text-wrap: wrap;
        color: hsl(var(--error));
    }

    .flex-col {
        display: flex;
        flex-direction: column;
    }

    .forgotten {
        margin: 0 0 .5rem .5rem;
    }

    .head {
        display: flex;
        justify-content: space-between;
    }

    .home {
        margin-right: 5px;
        cursor: pointer;
    }

    .loginWith {
        display: flex;
        justify-content: center;
        margin-top: -.9rem;
    }

    .loginWith > div {
        padding: 0 .5rem;
        font-size: .8rem;
        color: hsla(var(--text) / .6);
        background: hsl(var(--bg));
    }

    .name {
        margin: 0 .5rem;
    }

    .providersSeparator {
        margin-top: 1rem;
        margin-bottom: .5rem;
    }

    .separator {
        height: 1px;
        background: hsla(var(--bg-high) / .8);
    }

    .providers {
        margin-top: .66rem;
    }

    .reg {
        margin-top: .5rem;
        color: hsla(var(--text) / .85);
    }

    .success {
        margin: 0 5px;
        color: hsl(var(--action));
    }
</style>
