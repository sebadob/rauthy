<script lang="ts">
    import {purgeStorage, saveCsrfToken} from "../../../utils/helpers";
    import {logout} from "../../../utils/dataFetching.js";
    import Button from "$lib/Button.svelte";
    import {useI18n} from "$state/i18n.svelte";
    import Main from "$lib5/Main.svelte";
    import ContentCenter from "$lib5/ContentCenter.svelte";
    import LangSelector from "$lib5/LangSelector.svelte";
    import type {LogoutParams} from "$api/query_params/logout.ts";
    import Template from "$lib5/Template.svelte";
    import {TPL_CSRF_TOKEN} from "../../../utils/constants";
    import {useParam} from "$state/param.svelte.ts";

    let t = useI18n();
    let err = '';
    let isLoading = $state(false);

    let csrfToken = $state('');
    let logoutData: LogoutParams = $state({
        post_logout_redirect_uri: useParam('post_logout_redirect_uri').get(),
        id_token_hint: useParam('id_token_hint').get(),
        state: useParam('state').get(),
    });
    $inspect('logoutData', logoutData);

    // TODO remove the csrfToken from this component completely after finishing
    // [#692](https://github.com/sebadob/rauthy/issues/692)
    $effect(() => {
        if (csrfToken) {
            saveCsrfToken(csrfToken);
        }
    });

    $effect(() => {
        if (logoutData.id_token_hint) {
            handleLogout();
        }
    });

    async function handleCancel() {
        window.location.replace('/auth/v1');
    }

    async function handleLogout() {
        isLoading = true;

        let res = await logout(logoutData);
        purgeStorage();
        let loc = res.headers.get('location');
        if (loc) {
            window.location.replace(loc);
        } else {
            await handleCancel();
        }
    }
</script>

<svelte:head>
    <title>{t?.logout.logout || 'Logout'}</title>
</svelte:head>

<Template id={TPL_CSRF_TOKEN} bind:value={csrfToken}/>

<Main>
    <ContentCenter>
        <div class="container">
            <h1>{t.logout.logout}</h1>
            <p>
                {t.logout.confirmMsg}
            </p>

            <div class="btn">
                <Button on:click={handleLogout} level={2} bind:isLoading>{t.logout.logout}</Button>
                <Button on:click={handleCancel} level={4}>{t.common.cancel}</Button>
            </div>

            {#if err}
                <div class:err>
                    {err}
                </div>
            {/if}
        </div>

        <LangSelector absolute/>
    </ContentCenter>
</Main>

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
