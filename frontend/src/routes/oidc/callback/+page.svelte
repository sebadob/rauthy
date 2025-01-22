<script>
    import {
        deleteVerifierFromStorage,
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
    import {useParam} from "$state/param.svelte.js";

    let pCode = useParam('code');
    let pState = useParam('state');

    onMount(async () => {
        const data = new URLSearchParams();
        let redirectUri = REDIRECT_URI_SUCCESS;

        let state = pState.get();
        if (state) {
            if (state === 'account') {
                redirectUri = REDIRECT_URI_SUCCESS_ACC;
            } else if (state.startsWith('device')) {
                redirectUri = `/auth/v1/${state}`;
            }
        }

        let code = pCode.get();
        if (!code) {
            console.error('no `code` given');
            return;
        }

        data.append('grant_type', 'authorization_code');
        data.append('code', code);
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

        window.location.replace(redirectUri);
    });
</script>

<svelte:head>
    <title>Login</title>
</svelte:head>
