<script lang="ts">
    import {
        deleteVerifierFromStorage,
        getVerifierFromStorage,
        saveCsrfToken,
    } from "$utils/helpers.js";
    import {onMount} from "svelte";
    import {
        CLIENT_ID,
        REDIRECT_URI_SUCCESS,
        REDIRECT_URI_SUCCESS_ACC,
    } from "$utils/constants.js";
    import {useParam} from "$state/param.svelte";

    let err = $state('');

    let pCode = useParam('code');
    let pState = useParam('state');

    onMount(async () => {
        let code = pCode.get();
        if (!code) {
            console.error('no `code` given');
            return;
        }

        const data = new URLSearchParams();
        data.append('grant_type', 'authorization_code');
        data.append('code', code);
        data.append('redirect_uri', `${window.location.origin}/auth/v1/oidc/callback`);
        data.append('client_id', CLIENT_ID);
        data.append('code_verifier', getVerifierFromStorage());

        // get and save the tokens
        let resToken = await fetch('/auth/v1/oidc/token', {
            method: 'POST',
            headers: {
                'Content-type': 'application/x-www-form-urlencoded',
                'Accept': 'application/json',
            },
            body: data,
        });
        let body = await resToken.json();
        if (body.error) {
            err = body.error.toString();
        } else if (body.access_token) {
            // // Save the access token in case we need to fetch a fresh CSRF token
            // saveAccessToken(body.access_token);
            // // ID Token is saved for the automatic logout with 'token_hint'
            // if (body.id_token) {
            //     saveIdToken(body.id_token);
            // }

            let resXsrf = await fetch('/auth/v1/oidc/sessioninfo/xsrf', {
                method: 'GET',
                headers: {
                    'Content-Type': 'application/json',
                    'Accept': 'application/json',
                    'Authorization': `Bearer ${body.access_token}`,
                },
            });
            let info = await resXsrf.json();
            if (info.csrf_token) {
                saveCsrfToken(info.csrf_token);
            } else {
                err = info.toString();
                return;
            }

            deleteVerifierFromStorage();

            let redirectTo = window.location.origin + REDIRECT_URI_SUCCESS;
            let state = pState.get();
            if (state) {
                if (state === 'account') {
                    redirectTo = window.location.origin + REDIRECT_URI_SUCCESS_ACC;
                } else if (state.startsWith('device')) {
                    redirectTo = `${window.location.origin}/auth/v1/${state}`;
                }
            }
            window.location.replace(redirectTo);
        } else {
            err = '';
        }
    });
</script>

<svelte:head>
    <title>Callback</title>
</svelte:head>

{#if err}
    <div class="err">
        {err}
    </div>
{/if}
