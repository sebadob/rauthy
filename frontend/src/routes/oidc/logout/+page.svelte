<script lang="ts">
    import { getCsrfToken, purgeStorage, saveCsrfToken } from '#utils/helpers.ts';
    import Button from '#lib/button/Button.svelte';
    import { useI18n } from '#state/i18n.svelte.ts';
    import PublicPage from '#lib/PublicPage.svelte';
    import type { LogoutParams } from '#api/types/logout.ts';
    import Template from '#lib/Template.svelte';
    import { IS_DEV, TPL_CSRF_TOKEN } from '#utils/constants.ts';
    import { useParam } from '#state/param.svelte.ts';
    import { formDataFromObj } from '#api/fetch.ts';

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

        let url = '/auth/v1/oidc/logout';
        if (IS_DEV) {
            url = '/auth/v1/dev/logout';
        }

        await fetch(url, {
            method: 'POST',
            headers: {
                'Content-type': 'application/x-www-form-urlencoded',
                'x-csrf-token': csrfToken,
            },
            // manual necessary during dev to avoid CORS issues when using the DEV vs static UI
            redirect: IS_DEV ? 'manual' : 'follow',
            body: formDataFromObj(logoutData),
        });

        // the fetch should always return a 302 and redirect automatically on success
        handleCancel();
    }
</script>

<svelte:head>
    <title>{t?.logout.logout || 'Logout'}</title>
</svelte:head>

<Template id={TPL_CSRF_TOKEN} bind:value={csrfToken} />

<PublicPage>
    <div class="public-card">
        <h1>{t.logout.logout}</h1>
        <p>{t.logout.confirmMsg}</p>

        <div class="btn">
            <Button onclick={handleLogout} {isLoading}>
                {t.logout.logout}
            </Button>
            <Button level={2} onclick={handleCancel}>
                {t.common.cancel}
            </Button>
        </div>

        {#if err}
            <div class:err>
                {err}
            </div>
        {/if}
    </div>
</PublicPage>

<style>
    .btn {
        margin-top: 1rem;
        display: flex;
        align-items: center;
        gap: 0.5rem;
        flex-wrap: wrap;
    }

    .err {
        margin-top: 0.66rem;
        color: hsl(var(--error));
    }
</style>
