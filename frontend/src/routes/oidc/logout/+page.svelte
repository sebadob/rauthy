<script>
    import {onMount} from "svelte";
    import {getQueryParams, purgeStorage, saveCsrfToken} from "../../../utils/helpers.js";
    import {logout} from "../../../utils/dataFetching.js";
    import Button from "$lib/Button.svelte";
    import Loading from "$lib/Loading.svelte";

    let err = '';
    let postLogoutUri = '';
    let isLoading = false;

    onMount(async () => {
        const params = getQueryParams();
        postLogoutUri = params.post_logout_redirect_uri;
        const token = params.id_token_hint;
        const state = params.state;

        const csrf = window.document.getElementsByName('rauthy-csrf-token')[0].id
        saveCsrfToken(csrf);

        const immediateLogout = window.document.getElementsByName('rauthy-action')[0].id
        if ('true' === immediateLogout) {
            isLoading = true;

            const req = {
                id_token_hint: token,
                post_logout_redirect_uri: postLogoutUri,
                state: state,
            };

            let res = await logout(req);
            await handleRes(res);
        }
    });

    async function handleCancel() {
        window.location.href = '/auth/v1';
    }

    async function handleLogout() {
        isLoading = true;
        let res = await logout({});
        await handleRes(res);
    }

    async function handleRes(res) {
        if (res.ok) {
            purgeStorage();
            window.location.href = postLogoutUri;
        } else {
            const body = await res.json();
            err = body.message;
            isLoading = false;
        }
    }

</script>

<svelte:head>
    <title>Logout</title>
</svelte:head>

{#if isLoading}
    <Loading/>
{/if}

<div class="container">
    <h1>Logout</h1>

    <div>
        Do you really want to logout and end your session?
    </div>

    <div class="btn">
        <Button on:click={handleLogout} level={2}>LOGOUT</Button>
        <Button on:click={handleCancel} level={4}>CANCEL</Button>
    </div>

    {#if err}
        <div class:err>
            {err}
        </div>
    {/if}
</div>

<style>
    .btn {
        margin: 10px;
    }

    .container {
        display: flex;
        flex-direction: column;
        justify-content: center;
        align-items: center;
    }

    .err {
        margin: 10px;
        color: var(--col-err)
    }
</style>
