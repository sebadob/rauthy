<script>
    import {onMount} from "svelte";
    import {getQueryParams, purgeStorage, saveCsrfToken} from "../../../utils/helpers.js";
    import {logout} from "../../../utils/dataFetching.js";
    import Button from "$lib/Button.svelte";
    import Loading from "$lib/Loading.svelte";
    import WithI18n from "$lib/WithI18n.svelte";

    let t = $state();
    let err = '';
    let postLogoutUri = '';
    let isLoading = $state(false);

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
        purgeStorage();
        if (res.ok) {
            if (!res.headers) { 
               // no header present
               window.location.href = postLogoutUri;
            }
            else {
                if (!res.headers.get('location')) {
                    window.location.href = postLogoutUri;
                }
                else {
                    window.location.href = res.headers.get('location');
                }
            }
        } else {
            await handleCancel();
        }
    }

</script>

<svelte:head>
    <title>{t?.logout || 'Logout'}</title>
</svelte:head>

<WithI18n bind:t content="logout">
    <div class="container">
        <h1>{t.logout}</h1>

        <p>
            {t.confirmMsg}
        </p>

        <div class="btn">
            <Button on:click={handleLogout} level={2} bind:isLoading>{t.logout.toUpperCase()}</Button>
            <Button on:click={handleCancel} level={4}>{t.cancel.toUpperCase()}</Button>
        </div>

        {#if err}
            <div class:err>
                {err}
            </div>
        {/if}
    </div>
</WithI18n>

<style>
    .btn {
        margin: 10px;
    }

    .container {
        display: flex;
        flex-direction: column;
        justify-content: center;
        align-items: center;
        padding: 1rem;
    }

    .err {
        margin: 10px;
        color: var(--col-err)
    }
</style>
