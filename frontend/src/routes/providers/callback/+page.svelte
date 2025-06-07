<script lang="ts">
    import {
        getProviderToken,
        getVerifierUpstreamFromStorage,
    } from "$utils/helpers";
    import {onMount} from "svelte";
    import WebauthnRequest from "$lib5/WebauthnRequest.svelte";
    import LangSelector from "$lib5/LangSelector.svelte";
    import Button from "$lib5/button/Button.svelte";
    import {useI18n} from "$state/i18n.svelte";
    import {useParam} from "$state/param.svelte";
    import ThemeSwitch from "$lib5/ThemeSwitch.svelte";
    import type {MfaPurpose, WebauthnAdditionalData} from "$webauthn/types.ts";
    import {fetchPost} from "$api/fetch";
    import type {WebauthnLoginResponse} from "$api/types/authorize.ts";
    import type {ProviderCallbackRequest} from "$api/types/auth_provider.ts";
    import {IS_DEV} from "$utils/constants";

    let t = useI18n();
    let clientMfaForce = $state(false);
    let error = $state('');

    let userId: undefined | string = $state();
    let mfaPurpose: undefined | MfaPurpose = $state();

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
            error = "'state' is missing in URL"
            return;
        }
        let code = useParam('code').get();
        if (!code) {
            error = "'code' is missing in URL"
            return;
        }

        let payload: ProviderCallbackRequest = {
            state,
            code,
            pkce_verifier: getVerifierUpstreamFromStorage(),
            xsrf_token: getProviderToken(),
        };

        let url = '/auth/v1/providers/callback';
        if (IS_DEV) {
            url = '/auth/v1/dev/providers_callback';
        }
        let res = await fetchPost<undefined | WebauthnLoginResponse>(url, payload);

        if (res.status === 202) {
            // -> all good
            window.location.replace(res.headers.get('location') || '/auth/v1/account');
        } else if (res.status === 200) {
            // -> all good, but needs additional passkey validation
            error = '';
            let body = res.body;
            if (body && 'user_id' in body && 'code' in body) {
                // TODO is there a nicer way of making TS happy with type checking?
                userId = body.user_id as string;
                mfaPurpose = {Login: body.code as string};
            } else {
                console.error('did not receive a proper WebauthnLoginResponse after HTTP200');
            }
        } else if (res.status === 204) {
            // in case of a 204, we have done a user federation on an existing account -> just redirect
            window.location.replace('/auth/v1/account');
        } else if (res.status === 403) {
            // we will get a forbidden if for instance the user already exists but without
            // any upstream provider link (or the wrong one)
            error = res.error?.message || 'HTTP 403 Forbidden';
        } else if (res.status === 406) {
            // 406 -> client forces MFA while the user has none
            error = t.authorize.clientForceMfa;
            clientMfaForce = true;
        } else {
            error = `HTTP ${res.status}: ${res.error?.message}`;
        }
    });

    function onWebauthnError(err: string) {
        error = err;
        mfaPurpose = undefined;
    }

    function onWebauthnSuccess(data?: WebauthnAdditionalData) {
        if (data && 'loc' in data) {
            window.location.replace(data.loc as string);
        }
    }

</script>

<svelte:head>
    <title>Callback</title>
</svelte:head>

{#if mfaPurpose && userId}
    <WebauthnRequest
            {userId}
            purpose={mfaPurpose}
            onSuccess={onWebauthnSuccess}
            onError={onWebauthnError}
    />
{:else if clientMfaForce}
    <div class="btn flex-col">
        <Button onclick={() => window.location.href = '/auth/v1/account'}>
            Account
        </Button>
    </div>
{:else if error}
    <div class="err">
        {error}
    </div>
{/if}

<ThemeSwitch absolute/>
<LangSelector absolute/>

<style>
    .btn {
        margin: 5px 0;
        display: flex;
    }

    .flex-col {
        display: flex;
        flex-direction: column;
    }
</style>
