<script lang="ts">
    import {onMount} from "svelte";
    import {getSessionInfo, getUser, getUserWebIdData} from "../../utils/dataFetching.js";
    import AccMain from "../../components/account/AccMain.svelte";
    import {redirectToLogin} from "../../utils/helpers";
    import Main from "$lib5/Main.svelte";
    import ContentCenter from "$lib5/ContentCenter.svelte";
    import {useI18n} from "$state/i18n.svelte";
    import {type SessionResponse} from "$api/response/common/session.ts";
    import {type UserResponse} from "$api/response/common/user.ts";
    import Loading from "$lib5/Loading.svelte";

    let t = useI18n();

    let session: undefined | SessionResponse = $state();
    let user: undefined | UserResponse = $state();
    let webIdData = $state();
    let isReady = $state(false);

    onMount(async () => {
        let res = await getSessionInfo();
        if (res.ok) {
            session = await res.json();
            if (!session) {
                console.error('did not receive valid session response');
                return;
            }

            res = await getUser(session.user_id);
            if (res.ok) {
                user = await res.json();
            } else {
                redirectToLogin('account');
            }

            res = await getUserWebIdData(session.user_id);
            if (res.ok) {
                webIdData = await res.json();
            } else if (res.status === 404) {
                webIdData = {
                    user_id: session.user_id,
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
        {#if isReady && session && user}
            <AccMain {session} bind:user bind:webIdData/>
        {/if}
    </ContentCenter>
</Main>
