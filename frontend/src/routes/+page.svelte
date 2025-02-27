<script lang="ts">
    import Button from "$lib5/button/Button.svelte";
    import Main from "$lib5/Main.svelte";
    import ContentCenter from "$lib5/ContentCenter.svelte";
    import {useI18n} from "$state/i18n.svelte";
    import Template from "$lib5/Template.svelte";
    import {TPL_IS_REG_OPEN} from "$utils/constants.ts";
    import ThemeSwitch from "$lib5/ThemeSwitch.svelte";
    import LangSelector from "$lib5/LangSelector.svelte";

    const btnWidth = "9rem";

    let t = useI18n();
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
    <title>Rauthy</title>
</svelte:head>

<Template id={TPL_IS_REG_OPEN} bind:value={isRegOpen}/>

<Main>
    <ContentCenter>
        <div class="btn">
            <Button onclick={redirectToAccount} width={btnWidth}>
                {t.index.accountLogin}
            </Button>
            {#if isRegOpen}
                <Button level={2} onclick={redirectToReg} width={btnWidth}>
                    {t.index.register}
                </Button>
            {/if}
            <Button level={3} onclick={redirectToAdmin} width={btnWidth}>
                {t.index.adminLogin}
            </Button>
        </div>
    </ContentCenter>

    <ThemeSwitch absolute/>
    <LangSelector absolute/>
</Main>

<style>
    .btn {
        display: flex;
        flex-direction: column;
        gap: .5rem;
    }
</style>
