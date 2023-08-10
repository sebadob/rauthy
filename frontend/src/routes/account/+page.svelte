<script>
    import {onMount} from "svelte";
    import {getSessionInfo, getUser} from "../../utils/dataFetching.js";
    import Loading from "../../components/Loading.svelte";
    import AccMain from "../../components/account/AccMain.svelte";
    import {redirectToLogin} from "../../utils/helpers.js";
    import BrowserCheck from "../../components/BrowserCheck.svelte";
    import WithI18n from "$lib/WithI18n.svelte";

    let t;
    let sessionInfo;
    let user;
    let isReady = false;

    onMount(async () => {
        let res = await getSessionInfo();
        if (res.ok) {
            sessionInfo = await res.json();

            res = await getUser(sessionInfo.user_id);
            if (res.ok) {
                user = await res.json();
                isReady = true;
            } else {
                redirectToLogin('account');
            }
        } else {
            redirectToLogin('account');
        }
    });
</script>

<svelte:head>
    <title>{t?.account || 'Account'} {user?.email}</title>
</svelte:head>

<BrowserCheck>
    <WithI18n bind:t content="account">
        {#if !isReady}
            <Loading/>
        {:else}
            <AccMain bind:t bind:sessionInfo bind:user/>
        {/if}
    </WithI18n>
</BrowserCheck>
