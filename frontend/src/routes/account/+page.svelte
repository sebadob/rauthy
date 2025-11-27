<script lang="ts">
    import AccMain from '$lib5/account/AccMain.svelte';
    import { redirectToLogin } from '$utils/helpers';
    import Main from '$lib5/Main.svelte';
    import ContentCenter from '$lib5/ContentCenter.svelte';
    import { useI18n } from '$state/i18n.svelte';
    import { type UserResponse } from '$api/types/user';
    import ThemeSwitch from '$lib5/ThemeSwitch.svelte';
    import { fetchGet } from '$api/fetch';
    import type { WebIdResponse } from '$api/types/web_id.ts';
    import LangSelector from '$lib5/LangSelector.svelte';
    import { useSession } from '$state/session.svelte';
    import type { SessionInfoResponse } from '$api/types/session.ts';

    let t = useI18n();
    let session = useSession('account');

    let user: undefined | UserResponse = $state();
    let webIdData: undefined | WebIdResponse = $state();
    let isReady = $state(false);

    $effect(() => {
        let s = session.get();
        if (s) {
            fetchInfo(s);
        }
    });

    async function fetchInfo(s: SessionInfoResponse) {
        const userId = s.user_id;
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
                webIdData = res[1].body;
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
    }
</script>

<svelte:head>
    <!--
    for some reason, t can be undefined in the svelte:head section even though it never is
    inside the component, even during init and before the very first render
    -->
    <title>{t?.account.account || 'Account'} {user?.email}</title>
</svelte:head>

<Main>
    <ContentCenter>
        {#if isReady && session && user}
            <AccMain
                bind:user
                bind:webIdData
            />
        {/if}
        <ThemeSwitch absolute />
        <LangSelector absolute />
    </ContentCenter>
</Main>
