<script>
    import {
        deleteVerifierFromStorage,
        getQueryParams,
        getVerifierFromStorage,
        saveAccessToken,
        saveCsrfToken,
        saveIdToken
    } from "../../../utils/helpers";
    import {onMount} from "svelte";
    import {
        CLIENT_ID,
        REDIRECT_URI_SUCCESS,
        REDIRECT_URI_SUCCESS_ACC,
    } from "../../../utils/constants.js";
    import {getSessionInfoXsrf, getToken} from "../../../utils/dataFetching.js";

    onMount(async () => {
        const query = getQueryParams();

        const data = new URLSearchParams();
        let redirectUri = REDIRECT_URI_SUCCESS;

        if (query.state) {
            if (query.state === 'account') {
                redirectUri = REDIRECT_URI_SUCCESS_ACC;
            } else if (query.state.startsWith('device')) {
                redirectUri = `/auth/v1/${query.state}`;
            }
        }

        data.append('grant_type', 'authorization_code');
        data.append('code', query.code);
        data.append('redirect_uri', redirectUri);
        data.append('client_id', CLIENT_ID);
        data.append('code_verifier', getVerifierFromStorage());

        // get and save the tokens
        let res = await getToken(data);
        let body = await res.json();
        // Save the access token in case we need to fetch a fresh CSRF token
        saveAccessToken(body.access_token);
        // ID Token is saved for the automatic logout with 'token_hint'
        saveIdToken(body.id_token);

        res = await getSessionInfoXsrf(body.access_token);
        body = await res.json();
        saveCsrfToken(body.csrf_token);

        // clean up
        deleteVerifierFromStorage();

        // all good -> redirect now
        window.location.href = redirectUri;
    });

    //
</script>

<svelte:head>
    <title>Login</title>
</svelte:head>
