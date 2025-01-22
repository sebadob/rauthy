<script>
    import {
        getProviderToken,
        getVerifierUpstreamFromStorage,
    } from "../../../utils/helpers";
    import {postProviderCallback} from "../../../utils/dataFetching.js";
    import {onMount} from "svelte";
    import WebauthnRequest from "../../../components/webauthn/WebauthnRequest.svelte";
    import BrowserCheck from "../../../components/BrowserCheck.svelte";
    import LangSelector from "$lib5/LangSelector.svelte";
    import Button from "$lib/Button.svelte";
    import {useI18n} from "$state/i18n.svelte";
    import {useParam} from "$state/param.svelte";

    // will contain the same translations as /oidc/authorize
    let t = useI18n();
    let clientMfaForce = $state(false);
    let error = $state('');
    let webauthnData = $state();

    onMount(async () => {
        let pErr = useParam('error').get();
        if (pErr) {
            // if we have any error, do not proceed like normal and only show the error
            let desc = useParam('error_description').get();
            error = `${pErr}: ${desc}`;
            return;
        }

        let data = {
            state: useParam('state').get(),
            code: useParam('code').get(),
            pkce_verifier: getVerifierUpstreamFromStorage(),
            xsrf_token: getProviderToken(),
        };
        let res = await postProviderCallback(data);

        if (res.status === 202) {
            // -> all good
            window.location.replace(res.headers.get('location') || '/auth/v1/account');
        } else if (res.status === 200) {
            // -> all good, but needs additional passkey validation
            error = '';
            webauthnData = await res.json();
        } else if (res.status === 204) {
            // in case of a 204, we have done a user federation on an existing account -> just redirect
            window.location.replace('/auth/v1/account');
        } else if (res.status === 403) {
            // we will get a forbidden if for instance the user already exists but without
            // any upstream provider link (or the wrong one)
            let body = await res.json();
            console.error(body);
            error = body.message;
        } else if (res.status === 406) {
            // 406 -> client forces MFA while the user has none
            error = t.authorize.clientForceMfa;
            clientMfaForce = true;
        } else {
            let body = await res.text();
            error = `HTTP ${res.status}: ${body}`;
        }
    });

    function onWebauthnError(err) {
        error = err || 'ERROR';
        webauthnData = undefined;
    }

    function onWebauthnSuccess(res) {
        if (res) {
            window.location.replace(res.loc);
        }
    }

</script>

<svelte:head>
    <title>Callback</title>
</svelte:head>

<BrowserCheck>
    {#if webauthnData}
        <WebauthnRequest
                bind:data={webauthnData}
                onSuccess={onWebauthnSuccess}
                onError={onWebauthnError}
        />
    {:else if clientMfaForce}
        <div class="btn flex-col">
            <Button on:click={() => window.location.href = '/auth/v1/account'}>
                ACCOUNT LOGIN
            </Button>
        </div>
    {:else if error}
        <div class="error">
            {error}
        </div>
    {/if}

    <LangSelector absolute/>
</BrowserCheck>

<style>
    .btn {
        margin: 5px 0;
        display: flex;
    }

    .error {
        color: var(--col-err)
    }

    .flex-col {
        display: flex;
        flex-direction: column;
    }
</style>
