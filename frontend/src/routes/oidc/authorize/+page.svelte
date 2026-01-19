<script lang="ts">
    import { formatDateFromTs, saveCsrfToken, saveProviderToken } from '$utils/helpers';
    import Button from '$lib5/button/Button.svelte';
    import WebauthnRequest from '$lib5/WebauthnRequest.svelte';
    import Input from '$lib5/form/Input.svelte';
    import LangSelector from '$lib5/LangSelector.svelte';
    import {
        IS_DEV,
        PKCE_VERIFIER_UPSTREAM,
        TPL_AUTH_PROVIDERS,
        TPL_CLIENT_LOGO_UPDATED,
        TPL_CLIENT_NAME,
        TPL_CLIENT_URL,
        TPL_CSRF_TOKEN,
        TPL_IS_REG_OPEN,
        TPL_LOGIN_ACTION,
        TPL_ATPROTO_ID,
    } from '$utils/constants.js';
    import IconHome from '$icons/IconHome.svelte';
    import Main from '$lib5/Main.svelte';
    import ContentCenter from '$lib5/ContentCenter.svelte';
    import { useI18n } from '$state/i18n.svelte';
    import Template from '$lib5/Template.svelte';
    import { useParam } from '$state/param.svelte';
    import ThemeSwitch from '$lib5/ThemeSwitch.svelte';
    import type { AuthProviderTemplate } from '$api/templates/AuthProvider.ts';
    import InputPassword from '$lib5/form/InputPassword.svelte';
    import type { MfaPurpose, WebauthnAdditionalData } from '$webauthn/types.ts';
    import { fetchGet, fetchPost, type IResponse } from '$api/fetch';
    import type {
        CodeChallengeMethod,
        LoginRefreshRequest,
        LoginRequest,
        RequestResetRequest,
        WebauthnLoginResponse,
    } from '$api/types/authorize.ts';
    import Form from '$lib5/form/Form.svelte';
    import ButtonAuthProvider from '$lib5/ButtonAuthProvider.svelte';
    import { onMount } from 'svelte';
    import type { SessionInfoResponse } from '$api/types/session.ts';
    import ClientLogo from '$lib5/ClientLogo.svelte';
    import type { ProviderLoginRequest } from '$api/types/auth_provider.ts';
    import { fetchSolvePow } from '$utils/pow';
    import { generatePKCE } from '$utils/pkce';
    import { PATTERN_ATPROTO_ID } from '$utils/patterns';
    import type { ToSAwaitLoginResponse, ToSLatestResponse } from '$api/types/tos';
    import TosAccept from '$lib/TosAccept.svelte';

    const inputWidth = '18rem';

    let t = useI18n();

    let authorizeUrl = $derived(IS_DEV ? '/auth/v1/dev/authorize' : '/auth/v1/oidc/authorize');

    let clientId = useParam('client_id').get();
    let clientName = $state('');
    // we can't use undefined to avoid a JSON error in the Template component
    let clientLogoUpdated = $state(-1);
    let clientUri = $state(IS_DEV ? '/auth/v1' : '');
    let redirectUri = useParam('redirect_uri').get();
    let nonce = useParam('nonce').get();
    let scopes = useParam('scope').get()?.split(' ') || [];

    let refEmail: undefined | HTMLInputElement = $state();
    let refPassword: undefined | HTMLInputElement = $state();

    let stateParam = useParam('state').get();
    let stateEncoded = $derived(stateParam ? encodeURIComponent(stateParam) : undefined);
    let challenge = useParam('code_challenge').get();
    let challengeMethod: CodeChallengeMethod = useParam(
        'code_challenge_method',
    ).get() as CodeChallengeMethod;
    let existingMfaUser: undefined | string = $state();
    let providers: AuthProviderTemplate[] = $state([]);
    let mfaPurpose: undefined | MfaPurpose = $state();

    let isLoading = $state(false);
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

    let atprotoId = $state('');
    let atprotoHandle = $state('');

    let isAtproto = $state(false);

    let email = $state(useParam('login_hint').get() || '');
    let password = $state('');
    let userId = $state('');
    let showPasswordInput = $derived(
        needsPassword && existingMfaUser !== email && !showReset && !isAtproto,
    );

    let tos: undefined | ToSLatestResponse = $state();
    let tosAcceptCode = $state('');

    onMount(() => {
        if (!needsPassword) {
            refEmail?.focus();
        }
    });

    $effect(() => {
        if (
            'Refresh' === loginAction &&
            clientId &&
            clientId.length > 0 &&
            redirectUri &&
            redirectUri.length > 0
        ) {
            onRefresh();
        }
    });

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
        if (loginAction?.startsWith('MfaLogin ')) {
            let mfaUser = loginAction.replace('MfaLogin ', '');
            email = mfaUser;
            existingMfaUser = mfaUser;
        }
    });

    $effect(() => {
        if (IS_DEV) {
            // Make sure to create a session manually during dev.
            // In prod, it will be handled automatically during the GET already.
            createSessionDev();
            browserIdDev();
        } else if (csrfToken) {
            saveCsrfToken(csrfToken);
        }
    });

    async function createSessionDev() {
        let res = await fetchPost<SessionInfoResponse>('/auth/v1/oidc/session');
        if (res.body?.csrf_token) {
            saveCsrfToken(res.body.csrf_token);
        } else {
            console.error(res.error);
        }
    }

    async function browserIdDev() {
        await fetchPost('/auth/v1/dev/browser_id');
    }

    async function fetchTos() {
        if (!tos) {
            let res = await fetchGet<ToSLatestResponse>('/auth/v1/tos/latest');
            if (res.body) {
                tos = res.body;
            } else if (res.status === 204) {
                console.error('No ToS exists when the user should update');
            }
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

        isLoading = true;

        const payload: LoginRefreshRequest = {
            client_id: clientId,
            redirect_uri: redirectUri,
            state: stateEncoded,
            nonce: nonce,
            scopes,
        };
        if (
            challenge &&
            challengeMethod &&
            (challengeMethod === 'plain' || challengeMethod === 'S256')
        ) {
            payload.code_challenge = challenge;
            payload.code_challenge_method = challengeMethod;
        }

        let res = await fetchPost<undefined | WebauthnLoginResponse>(
            '/auth/v1/oidc/authorize/refresh',
            payload,
        );
        await handleAuthRes(res);
    }

    async function onSubmit(form?: HTMLFormElement, params?: URLSearchParams) {
        if (isAtproto) {
            return providerLogin(atprotoId);
        }

        err = '';

        if (!clientId) {
            console.error('clientId is undefined');
            return;
        }
        if (!redirectUri) {
            console.error('redirectUri is undefined');
            return;
        }

        isLoading = true;

        let pow = (await fetchSolvePow()) || '';

        const payload: LoginRequest = {
            email,
            pow,
            client_id: clientId,
            redirect_uri: redirectUri,
            state: stateEncoded,
            nonce: nonce,
            scopes,
        };
        if (
            challenge &&
            challengeMethod &&
            (challengeMethod === 'plain' || challengeMethod === 'S256')
        ) {
            payload.code_challenge = challenge;
            payload.code_challenge_method = challengeMethod;
        }

        if (needsPassword && email !== existingMfaUser) {
            if (!password) {
                err = t.authorize.passwordRequired;
                isLoading = false;
                return;
            }
            if (password.length > 256) {
                err = 'max 256';
                isLoading = false;
                return;
            }
            payload.password = password;
        }

        let url = '/auth/v1/oidc/authorize';
        if (IS_DEV) {
            url = '/auth/v1/dev/authorize';
        }

        let res = await fetchPost<undefined | WebauthnLoginResponse | ToSAwaitLoginResponse>(
            url,
            payload,
            'json',
            'noRedirect',
        );
        await handleAuthRes(res);
    }

    async function handleAuthRes(
        res?: IResponse<undefined | WebauthnLoginResponse | ToSAwaitLoginResponse>,
    ) {
        isLoading = false;

        if (!res) {
            console.error('no result in handleAuthRes');
            return;
        }

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
                userId = body.user_id as string;
                mfaPurpose = { Login: body.code as string };
            } else {
                console.error('did not receive a proper WebauthnLoginResponse after HTTP200');
            }
        } else if (res.status === 206) {
            // login successful, but the user needs to accept updated ToS
            let body = res.body as ToSAwaitLoginResponse;
            userId = body.user_id;
            tosAcceptCode = body.tos_await_code;
            await fetchTos();
        } else if (res.status === 400) {
            err = res.error?.message || '';
        } else if (res.status === 403) {
            if (res.error?.message.includes('contact your Administrator')) {
                err = t.authorize.clientGroupPrefixForbidden;
            } else {
                // This should really never happen. Forbidden is only returned for group prefix mismatch.
                err = res.error?.message || '';
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
    }

    function isProviderAtProto(providerId: string): boolean {
        return providerId === atprotoId;
    }

    function onEmailInput() {
        if (isAtproto) {
            return;
        }

        // this will basically remove the password input again if the user was asked to provide
        // a password and afterward changes his email again
        if (needsPassword && emailAfterSubmit !== email) {
            needsPassword = false;
            password = '';
            err = '';
        }
    }

    function providerLogin(id: string) {
        generatePKCE().then(pkce => {
            if (pkce) {
                localStorage.setItem(PKCE_VERIFIER_UPSTREAM, pkce.verifier);
                providerLoginPkce(id, pkce.challenge);
            }
        });
    }

    async function onToSCancel() {
        password = '';
        userId = '';
        tosAcceptCode = '';
        tos = undefined;
        isLoading = false;
        mfaPurpose = undefined;
    }

    async function providerLoginPkce(id: string, pkce_challenge: string) {
        // make sure to reset input fields to not trigger a failing validation
        email = '';
        password = '';

        if (!clientId) {
            console.error('clientId is undefined');
            return;
        }
        if (!redirectUri) {
            console.error('redirectUri is undefined');
            return;
        }

        isLoading = true;
        let pow = (await fetchSolvePow()) || '';

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
            pow,
            ...(isAtproto && { handle: atprotoHandle }),
        };

        let res = await fetchPost<string>('/auth/v1/providers/login', payload);
        isLoading = false;

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
        if (!data) {
            return;
        }

        if ('loc' in data) {
            window.location.replace(data.loc as string);
        } else if ('tos_await_code' in data) {
            // login successful, but the user needs to accept updated ToS
            tosAcceptCode = data.tos_await_code as string;
            mfaPurpose = undefined;
            fetchTos();
        }
    }

    async function requestReset() {
        isLoading = true;
        let pow = (await fetchSolvePow()) || '';

        let payload: RequestResetRequest = { email, pow };
        if (clientUri) {
            payload.redirect_uri = encodeURI(clientUri);
        }

        let res = await fetchPost('/auth/v1/users/request_reset', payload);
        if (res.error) {
            err = res.error.message;
        } else {
            emailSuccess = true;
        }

        isLoading = false;
    }

    function toggleAtproto() {
        isAtproto = !isAtproto;
    }
</script>

<svelte:head>
    <title>Login: {clientName || clientId}</title>
</svelte:head>

<Template id={TPL_AUTH_PROVIDERS} bind:value={providers} />
<Template id={TPL_ATPROTO_ID} bind:value={atprotoId} />
<Template id={TPL_CLIENT_NAME} bind:value={clientName} />
<Template id={TPL_CLIENT_URL} bind:value={clientUri} />
<Template id={TPL_CLIENT_LOGO_UPDATED} bind:value={clientLogoUpdated} />
<Template id={TPL_CSRF_TOKEN} bind:value={csrfToken} />
<Template id={TPL_LOGIN_ACTION} bind:value={loginAction} />
<Template id={TPL_IS_REG_OPEN} bind:value={isRegOpen} />

<Main>
    <div class="outer">
        <ContentCenter>
            <div class="container">
                <div class="head">
                    {#if clientId}
                        <ClientLogo
                            {clientId}
                            updated={clientLogoUpdated > -1 ? clientLogoUpdated : undefined}
                        />
                    {/if}
                    {#if clientUri}
                        <a class="home" href={clientUri} aria-label="Client Home Page">
                            <IconHome color="hsla(var(--text) / .9)" />
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
                            {#if isAtproto}
                                <Input
                                    name="handle"
                                    bind:value={atprotoHandle}
                                    label="Handle / DID"
                                    placeholder="Handle / DID"
                                    pattern={PATTERN_ATPROTO_ID}
                                    disabled={tooManyRequests}
                                    width={inputWidth}
                                    required
                                />
                            {:else}
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
                            {/if}
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
                                    <Button
                                        ariaLabel={t.authorize.passwordForgotten}
                                        invisible
                                        onclick={handleShowReset}
                                    >
                                        {t.authorize.passwordForgotten}
                                    </Button>
                                </div>
                            {/if}
                        {/if}

                        {#if !tooManyRequests && !clientMfaForce}
                            {#if showReset && !isAtproto}
                                <div class="btn flex-col">
                                    <Button
                                        ariaLabel={t.authorize.passwordRequest}
                                        onclick={requestReset}
                                    >
                                        {t.authorize.passwordRequest}
                                    </Button>
                                </div>
                            {:else}
                                <div class="btn flex-col">
                                    <Button
                                        type="submit"
                                        ariaLabel={t.authorize.login}
                                        onclick={() => onSubmit()}
                                        {isLoading}
                                    >
                                        {t.authorize.login}
                                    </Button>
                                </div>
                                {#if isAtproto}
                                    <div class="btn flex-col">
                                        <Button
                                            ariaLabel={t.common.cancel}
                                            level={2}
                                            onclick={toggleAtproto}
                                        >
                                            {t.common.cancel}
                                        </Button>
                                    </div>
                                {/if}
                            {/if}
                        {/if}
                    </Form>

                    {#if isRegOpen && !showResetRequest && !tooManyRequests && !isAtproto}
                        {#if clientUri}
                            <a
                                class="reg"
                                href="/auth/v1/users/register?redirect_uri={clientUri}"
                                target="_blank"
                            >
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
                        <Button
                            ariaLabel="Account"
                            onclick={() => (window.location.href = '/auth/v1/account')}
                        >
                            Account
                        </Button>
                    </div>
                {/if}

                {#if tos}
                    <TosAccept {tos} {tosAcceptCode} onToSAccept={handleAuthRes} {onToSCancel} />
                {/if}

                {#if !clientMfaForce && providers.length > 0 && !isAtproto}
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
                                onclick={isProviderAtProto(provider.id)
                                    ? toggleAtproto
                                    : providerLogin}
                                {isLoading}
                            />
                        {/each}
                    </div>
                {/if}
            </div>

            <ThemeSwitch absolute />
            <LangSelector absolute borderless />
        </ContentCenter>
    </div>
</Main>

<style>
    .btn {
        margin: 0.8rem 0 0.5rem 0;
        display: flex;
    }

    .outer {
        width: 100dvw;
        height: 100dvh;
        background: hsla(var(--bg-high) / 0.25);
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
        margin: 0.5rem 0;
        max-width: 18rem;
        text-wrap: wrap;
        color: hsl(var(--error));
    }

    .flex-col {
        display: flex;
        flex-direction: column;
    }

    .forgotten {
        margin: 0 0 0.5rem 0.5rem;
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
        margin-top: -0.9rem;
    }

    .loginWith > div {
        padding: 0 0.5rem;
        font-size: 0.8rem;
        color: hsla(var(--text) / 0.6);
        background: hsl(var(--bg));
    }

    .name {
        margin: 0 0.5rem;
    }

    .providersSeparator {
        margin-top: 1rem;
        margin-bottom: 0.5rem;
    }

    .separator {
        height: 1px;
        background: hsla(var(--bg-high) / 0.8);
    }

    .providers {
        margin-top: 0.66rem;
    }

    .reg {
        margin-top: 0.5rem;
        color: var(--text);
    }

    .success {
        margin: 0 5px;
        color: hsl(var(--action));
    }
</style>
