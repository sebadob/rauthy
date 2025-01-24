<script lang="ts">
    import {onMount} from "svelte";
    import AccMain from "../../components/account/AccMain.svelte";
    import {redirectToLogin} from "$utils/helpers.ts";
    import Main from "$lib5/Main.svelte";
    import ContentCenter from "$lib5/ContentCenter.svelte";
    import {useI18n} from "$state/i18n.svelte";
    import {type SessionResponse} from "$api/types/session.ts";
    import {type UserResponse} from "$api/types/user.ts";
    import ThemeSwitch from "$lib5/ThemeSwitch.svelte";
    import {fetchGet} from "$api/fetch.ts";
    import type {WebIdResponse} from "$api/types/web_id.ts";
    import LangSelector from "$lib5/LangSelector.svelte";
    import {useSession} from "$state/session.svelte.ts";

    let t = useI18n();
    let session = useSession();

    let user: undefined | UserResponse = $state();
    let webIdData: undefined | WebIdResponse = $state();
    let isReady = $state(false);

    onMount(async () => {
        let res = await fetchGet<SessionResponse>('/auth/v1/oidc/sessioninfo');
        if (res.body) {
            session.set(res.body);
            if (!session) {
                console.error('did not receive valid session response');
                return;
            }

            const userId = res.body.user_id;
            if (userId) {
                let res = await Promise.all([
                    fetchGet<UserResponse>(`/auth/v1/users/${userId}`),
                    fetchGet<WebIdResponse>(`/auth/v1/users/${userId}/webid/data`),
                ]);

                if (res[0].body) {
                    user = res[0].body;
                } else {
                    redirectToLogin('account');
                }

                if (res[1].body) {
                    webIdData = res[1].body
                } else if (res[1].status === 404) {
                    webIdData = {
                        user_id: userId,
                        expose_email: false,
                    };
                } else {
                    // now it can only be a 405 -> webid is not enabled -> do nothing
                }

                isReady = true;
            } else {
                console.error('no user_id in session');
            }
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
            <AccMain bind:user bind:webIdData/>
        {/if}
        <ThemeSwitch absolute/>
        <LangSelector absolute/>
    </ContentCenter>
</Main>