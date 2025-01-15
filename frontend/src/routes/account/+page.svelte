<script>
    import {onMount} from "svelte";
    import {getSessionInfo, getUser, getUserWebIdData} from "../../utils/dataFetching.js";
    import Loading from "../../components/Loading.svelte";
    import AccMain from "../../components/account/AccMain.svelte";
    import {redirectToLogin} from "../../utils/helpers";
    import Main from "$lib5/Main.svelte";
    import ContentCenter from "$lib5/ContentCenter.svelte";
    import {useI18n} from "$state/i18n.svelte";

    let t = useI18n();

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
    <!--
    for some reason, t can be undefined in the svelte:head section even though it never is
    inside the component, even during init and before the very first render
    -->
    <title>{t?.account.account || 'Account'} {user?.email}</title>
    <!--    <title>{t.account.account} {user.account.email}</title>-->
</svelte:head>

<Main>
    <ContentCenter>
        {#if !isReady}
            <Loading/>
        {:else}
            <AccMain {sessionInfo} bind:user bind:webIdData/>
        {/if}
    </ContentCenter>
</Main>
