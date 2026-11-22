<script lang="ts">
    import PublicPage from '#lib/PublicPage.svelte';
    import { getProviderToken, getVerifierUpstreamFromStorage } from '#utils/helpers.ts';
    import { onMount } from 'svelte';
    import WebauthnRequest from '#lib/WebauthnRequest.svelte';
    import Button from '#lib/button/Button.svelte';
    import { useI18n } from '#state/i18n.svelte.ts';
    import { useParam } from '#state/param.svelte.ts';
    import type { WebauthnAdditionalData } from '#mfa/webauthn/types.ts';
    import { fetchGet, fetchPost, type IResponse } from '#api/fetch.ts';
    import type {
        ActiveOtp,
        OtpLoginResponse,
        WebauthnLoginResponse,
    } from '#api/types/authorize.ts';
    import type { ProviderCallbackRequest } from '#api/types/auth_provider.ts';
    import { IS_DEV, PROVIDER_LOGIN_PAGE } from '#utils/constants.ts';
    import type { ToSAwaitLoginResponse, ToSLatestResponse } from '#api/types/tos.ts';
    import TosAccept from '#lib/TosAccept.svelte';
    import type { MfaPurpose } from '#api/types/mfa.ts';
    import OtpRequest from '#lib/OtpRequest.svelte';
    import type { OtpAdditionalData } from '#mfa/otp/types.ts';

    let t = useI18n();
    let clientMfaForce = $state(false);
    let error = $state('');

    let mfaPurpose: undefined | MfaPurpose = $state();
    let mfaKind: undefined | 'webauthn' | 'otp' = $state();
    let activeOtps: undefined | ActiveOtp[] = $state();

    let tos: undefined | ToSLatestResponse = $state();
    let tosAcceptCode = $state('');
    let tosForceAccept = $state(false);
    let needsValuesUpdate = $state(false);

    onMount(async () => {
        let pErr = useParam('error').get();
        if (pErr) {
            // if we have any error, do not proceed like normal and only show the error
            let desc = useParam('error_description').get();
            error = `${pErr}: ${desc}`;
            return;
        }

        let state = useParam('state').get();
        if (!state) {
            error = "'state' is missing in URL";
            return;
        }
        let code = useParam('code').get();
        if (!code) {
            error = "'code' is missing in URL";
            return;
        }
        let iss = useParam('iss').get();

        let payload: ProviderCallbackRequest = {
            state,
            code,
            pkce_verifier: getVerifierUpstreamFromStorage(),
            xsrf_token: getProviderToken(),
            iss_atproto: iss,
        };

        let url = '/auth/v1/providers/callback';
        if (IS_DEV) {
            url = '/auth/v1/dev/providers_callback';
        }
        let res = await fetchPost<
            undefined | WebauthnLoginResponse | ToSAwaitLoginResponse | OtpLoginResponse
        >(url, payload);

        await handleAuthRes(res);
    });

    async function handleAuthRes(
        res?: IResponse<
            undefined | WebauthnLoginResponse | ToSAwaitLoginResponse | OtpLoginResponse
        >,
    ) {
        if (!res) {
            return;
        }

        if (res.status === 202) {
            // -> all good
            window.location.replace(res.headers.get('location') || '/auth/v1/account');
        } else if (res.status === 200) {
            // -> all good, but needs additional passkey validation
            error = '';
            let body = res.body;
            if (body && 'code' in body) {
                mfaPurpose = { Login: body.code as string };
                if ('active_otps' in body) {
                    activeOtps = body.active_otps;
                    mfaKind = 'otp';
                } else {
                    mfaKind = 'webauthn';
                }
            } else {
                console.error(
                    'did not receive a proper OtpLoginResponse or WebauthnLoginResponse after HTTP200',
                );
            }
        } else if (res.status === 204) {
            // in case of a 204, we have done a user federation on an existing account -> just redirect
            window.location.replace('/auth/v1/account');
        } else if (res.status === 205) {
            // -> all good, password only account, user needs to update some values
            needsValuesUpdate = true;
        } else if (res.status === 206) {
            // login successful, but the user needs to accept updated ToS
            let body = res.body as ToSAwaitLoginResponse;
            tosAcceptCode = body.tos_await_code;
            tosForceAccept = body.force_accept || false;
            await fetchTos();
        } else if (res.status === 403) {
            // we will get a forbidden if for instance the user already exists but without
            // any upstream provider link (or the wrong one)
            error = res.error?.message || 'HTTP 403 Forbidden';
        } else if (res.status === 404) {
            error = 'User not found';
            setTimeout(() => {
                window.location.replace('/auth/v1/');
            }, 5000);
        } else if (res.error?.error === 'consent_required') {
            // The consent is missing or changed meanwhile. The login page asks for it again with the
            // original request.
            let loginPage = sessionStorage.getItem(PROVIDER_LOGIN_PAGE);
            sessionStorage.removeItem(PROVIDER_LOGIN_PAGE);
            if (loginPage?.startsWith(`${window.location.origin}/`)) {
                window.location.replace(loginPage);
            } else {
                error = t.authorize.consent.changed;
            }
        } else if (res.status === 406) {
            // 406 -> client forces MFA while the user has none
            error = t.authorize.clientForceMfa;
            clientMfaForce = true;
        } else {
            error = `HTTP ${res.status}: ${res.error?.message}`;
        }
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

    function onToSCancel() {
        setTimeout(() => {
            window.location.replace('/auth/v1/');
        }, 1000);
    }

    function onMfaError(err: string) {
        error = err;
        mfaPurpose = undefined;
        mfaKind = undefined;
    }

    function onMfaSuccess(data?: WebauthnAdditionalData | OtpAdditionalData) {
        if (!data) {
            // will be empty if the user needs to update values
            mfaPurpose = undefined;
            mfaKind = undefined;
            needsValuesUpdate = true;
            return;
        }

        if (data && 'loc' in data) {
            window.location.replace(data.loc as string);
        }
    }
</script>

<svelte:head>
    <title>Callback</title>
</svelte:head>

<PublicPage>
    {#if mfaPurpose}
        {#if mfaKind == 'webauthn'}
            <WebauthnRequest purpose={mfaPurpose} onSuccess={onMfaSuccess} onError={onMfaError} />
        {:else if mfaKind == 'otp' && activeOtps}
            <OtpRequest
                {activeOtps}
                purpose={mfaPurpose}
                onSuccess={onMfaSuccess}
                onError={onMfaError}
            />
        {/if}
    {:else if clientMfaForce}
        <div class="public-card">
            <Button onclick={() => (window.location.href = '/auth/v1/account')}>Account</Button>
        </div>
    {:else if needsValuesUpdate}
        <div class="public-card ui-section-stack">
            <p>{t.authorize.needsUserUpdate}</p>
            <Button onclick={() => window.location.replace('/auth/v1/account')}>
                {t.authorize.navigateToAccount}
            </Button>
        </div>
    {:else if tos}
        <TosAccept
            {tos}
            forceAccept={tosForceAccept}
            {tosAcceptCode}
            onToSAccept={handleAuthRes}
            {onToSCancel}
        />
    {:else if error}
        <div class="public-card err">
            {error}
        </div>
    {/if}
</PublicPage>
