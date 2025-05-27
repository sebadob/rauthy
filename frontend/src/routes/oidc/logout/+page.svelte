<script lang="ts">
    import {getCsrfToken, purgeStorage, saveCsrfToken} from "$utils/helpers";
    import Button from "$lib5/button/Button.svelte";
    import {useI18n} from "$state/i18n.svelte";
    import Main from "$lib5/Main.svelte";
    import ContentCenter from "$lib5/ContentCenter.svelte";
    import LangSelector from "$lib5/LangSelector.svelte";
    import type {LogoutParams} from "$api/types/logout.ts";
    import Template from "$lib5/Template.svelte";
    import {TPL_CSRF_TOKEN} from "$utils/constants";
    import {useParam} from "$state/param.svelte";
    import ThemeSwitch from "$lib5/ThemeSwitch.svelte";
    import {formDataFromObj} from "$api/fetch";
    import {useIsDev} from "$state/is_dev.svelte";

    let t = useI18n();
    let err = '';
    let isLoading = $state(false);

    let csrfToken = $state('');
    let logoutData: LogoutParams = $state({
        post_logout_redirect_uri: useParam('post_logout_redirect_uri').get(),
        id_token_hint: useParam('id_token_hint').get(),
        state: useParam('state').get(),
    });

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

    function handleCancel() {
        window.location.replace('/auth/v1/');
    }

    async function handleLogout() {
        isLoading = true;
        let csrfToken = getCsrfToken();
        purgeStorage();

        let isDev = useIsDev().get();

        let url = '/auth/v1/oidc/logout';
        if (isDev) {
            url = '/auth/v1/dev/logout';
        }

        await fetch(url, {
            method: 'POST',
            headers: {
                'Content-type': 'application/x-www-form-urlencoded',
                'x-csrf-token': csrfToken,
            },
            // manual necessary during dev to avoid CORS issues when using the DEV vs static UI
            redirect: isDev ? 'manual' : 'follow',
            body: formDataFromObj(logoutData),
        });

        // the fetch should always return a 302 and redirect automatically on success
        handleCancel();
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
            <p>{t.logout.confirmMsg}</p>

            <div class="btn">
                <Button onclick={handleLogout} {isLoading}>
                    {t.logout.logout}
                </Button>
                <Button level={3} onclick={handleCancel}>
                    {t.common.cancel}
                </Button>
            </div>

            {#if err}
                <div class:err>
                    {err}
                </div>
            {/if}
        </div>

        <ThemeSwitch absolute/>
        <LangSelector absolute/>
    </ContentCenter>
</Main>

<style>
    .btn {
        margin-top: 1rem;
        display: flex;
        align-items: center;
        gap: 1rem;
    }

    .container {
        display: flex;
        flex-direction: column;
        justify-content: center;
        align-items: center;
        padding: 1rem;
    }

    .err {
        margin-top: .66rem;
        color: hsl(var(--error));
    }
</style>
