<script>
    import {onMount} from "svelte";
    import {getSessionInfo, getUser, getUserWebIdData} from "../../utils/dataFetching.js";
    import Loading from "../../components/Loading.svelte";
    import AccMain from "../../components/account/AccMain.svelte";
    import {redirectToLogin} from "../../utils/helpers.js";
    import BrowserCheck from "../../components/BrowserCheck.svelte";
    import WithI18n from "$lib/WithI18n.svelte";

    let t = $state();
    let sessionInfo = $state();
    let user = $state();
    let webIdData = $state();
    let isReady = $state(false);

    onMount(async () => {
        let res = await getSessionInfo();
        if (res.ok) {
            sessionInfo = await res.json();

            res = await getUser(sessionInfo.user_id);
            if (res.ok) {
                user = await res.json();
            } else {
                redirectToLogin('account');
            }

            res = await getUserWebIdData(sessionInfo.user_id);
            if (res.ok) {
                webIdData = await res.json();
            } else if (res.status === 404) {
                webIdData = {
                    user_id: sessionInfo.user_id,
                    is_open: false,
                };
            } else {
                // now it can only be a 405 -> webid is not enabled -> do nothing
            }

            isReady = true;
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
            <AccMain bind:t bind:sessionInfo bind:user bind:webIdData />
        {/if}
    </WithI18n>
</BrowserCheck>
