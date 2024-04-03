<script>
    import {
        getQueryParams,
        getProviderToken,
        getVerifierUpstreamFromStorage,
    } from "../../../utils/helpers.js";
    import {postProviderCallback} from "../../../utils/dataFetching.js";
    import {onMount} from "svelte";
    import WebauthnRequest from "../../../components/webauthn/WebauthnRequest.svelte";
    import BrowserCheck from "../../../components/BrowserCheck.svelte";
    import WithI18n from "$lib/WithI18n.svelte";
    import LangSelector from "$lib/LangSelector.svelte";
    import Button from "$lib/Button.svelte";
    import {sleepAwait} from "$lib/utils/helpers.js";

    // will contain the same translations as /oidc/authorize
    let t = {};
    let clientMfaForce = false;
    let error = '';
    let webauthnData;

    onMount(async () => {
        const query = getQueryParams();
        let data = {
            state: query.state,
            code: query.code,
            pkce_verifier: getVerifierUpstreamFromStorage(),
            xsrf_token: getProviderToken(),
        };
        let res = await postProviderCallback(data);

        if (res.status === 202) {
            // -> all good
            window.location.replace(res.headers.get('location'));
        } else if (res.status === 200) {
            // -> all good, but needs additional passkey validation
            error = '';
            webauthnData = await res.json();
        } else if (res.status === 403) {
            // TODO we will get a forbidden if for instance the user already exists but without
            // any upstream provider link (or the wrong one)
            // TODO add i18n for these cases
            let body = await res.json();
            console.error(body);
            error = body.message;
        } else if (res.status === 406) {
            // 406 -> client forces MFA while the user has none
            error = t.clientForceMfa;
            clientMfaForce = true;
        } else {
            error = `Uncovered HTTP return status '${res.status}'. This should never happen, please report this bug.`;
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
    <WithI18n bind:t content="authorize">
        {#if webauthnData}
            <WebauthnRequest
                    bind:t
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
            <!--{:else}-->
            <!--            <Loading/>-->
        {/if}

        <LangSelector absolute/>
    </WithI18n>
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
