<script lang="ts">
    import Button from '$lib5/button/Button.svelte';
    import Main from '$lib5/Main.svelte';
    import ContentCenter from '$lib5/ContentCenter.svelte';
    import { useI18n } from '$state/i18n.svelte';
    import Template from '$lib5/Template.svelte';
    import { TPL_ADMIN_BUTTON_HIDE, TPL_IS_REG_OPEN } from '$utils/constants';
    import ThemeSwitch from '$lib5/ThemeSwitch.svelte';
    import LangSelector from '$lib5/LangSelector.svelte';

    const btnWidth = '9rem';

    let t = useI18n();
    let hideAdminButton = $state(true);
    let isRegOpen = $state(false);

    function redirectToAdmin() {
        window.location.href = '/auth/v1/admin';
    }

    function redirectToAccount() {
        window.location.href = '/auth/v1/account';
    }

    function redirectToReg() {
        window.location.href = '/auth/v1/users/register';
    }
</script>

<svelte:head>
    <title>Rauthy Home</title>
</svelte:head>

<Template id={TPL_ADMIN_BUTTON_HIDE} bind:value={hideAdminButton} />
<Template id={TPL_IS_REG_OPEN} bind:value={isRegOpen} />

<Main>
    <ContentCenter>
        <div class="btn">
            <Button ariaLabel={t.index.accountLogin} onclick={redirectToAccount} width={btnWidth}>
                {t.index.accountLogin}
            </Button>
            {#if isRegOpen}
                <Button
                    ariaLabel={t.index.register}
                    level={2}
                    onclick={redirectToReg}
                    width={btnWidth}
                >
                    {t.index.register}
                </Button>
            {/if}
            {#if !hideAdminButton}
                <Button
                    ariaLabel={t.index.adminLogin}
                    level={3}
                    onclick={redirectToAdmin}
                    width={btnWidth}
                >
                    {t.index.adminLogin}
                </Button>
            {/if}
        </div>
    </ContentCenter>

    <ThemeSwitch absolute />
    <LangSelector absolute />
</Main>

<style>
    .btn {
        height: 7rem;
        display: flex;
        flex-direction: column;
        gap: 0.5rem;
    }
</style>
