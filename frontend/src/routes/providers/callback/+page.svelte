<script>
    import Loading from "$lib/Loading.svelte";
    import {
        getQueryParams,
        getVerifierFromStorage,
        getProviderToken,
    } from "../../../utils/helpers.js";
    import {postProviderCallback} from "../../../utils/dataFetching.js";
    import {onMount} from "svelte";

    let error = '';

    onMount(async () => {
        const query = getQueryParams();

        // TODO instead of the normal callback, we do not want to get a token,
        // but actually finish the provider login flow, which then will start with
        // Rauthy's default login flow.

        let data = {
            state: query.state,
            code: query.code,
            pkce_verifier: getVerifierFromStorage(),
            xsrf_token: getProviderToken(),
        };
        console.log(data);
        let res = await postProviderCallback(data);
        let body = await res.json();
        if (res.ok) {
            console.log(body);
        } else {
            error = body.message;
        }

        console.error('TODO finish implementing callback logic');
        return;

        // const data = new URLSearchParams();
        // let redirectUri = REDIRECT_URI_SUCCESS;
        // if (query.state && query.state === 'account') {
        //     redirectUri = REDIRECT_URI_SUCCESS_ACC;
        // }
        //
        // data.append('grant_type', 'authorization_code');
        // data.append('code', query.code);
        // data.append('redirect_uri', redirectUri);
        // data.append('client_id', CLIENT_ID);
        // data.append('code_verifier', getVerifierFromStorage());
        //
        // // get and save the tokens
        // let res = await getToken(data);
        // let body = await res.json();
        // // Save the access token in case we need to fetch a fresh CSRF token
        // saveAccessToken(body.access_token);
        // // ID Token is saved for the automatic logout with 'token_hint'
        // saveIdToken(body.id_token);
        //
        // res = await getSessionInfoXsrf(body.access_token);
        // body = await res.json();
        // saveCsrfToken(body.csrf_token);
        //
        // // clean up
        // deleteVerifierFromStorage();
        //
        // // all good -> redirect now
        // window.location.href = redirectUri;
    });

    //
</script>

<svelte:head>
    <title>Callback</title>
</svelte:head>

{#if error}
    {error}
{:else}
    <Loading/>
{/if}
<!-- Placeholder for SSR of provider callback errors -->
