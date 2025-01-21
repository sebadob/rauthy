<script lang="ts">
    import Button from "$lib/Button.svelte";
    import LangSelector from "$lib5/LangSelector.svelte";
    import AppVersion from "../components/AppVersion.svelte";
    import Main from "$lib5/Main.svelte";
    import ContentCenter from "$lib5/ContentCenter.svelte";
    import {useI18n} from "$state/i18n.svelte";
    import Template from "$lib5/Template.svelte";
    import {TPL_IS_REG_OPEN} from "../utils/constants";

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
            {#if isRegOpen}
                <Button on:click={redirectToReg} width={btnWidth}>{t.index.register}</Button>
            {/if}
            <Button on:click={redirectToAccount} width={btnWidth}>{t.index.accountLogin}</Button>
            <Button on:click={redirectToAdmin} width={btnWidth}>{t.index.adminLogin}</Button>
        </div>
    </ContentCenter>

    <LangSelector absolute/>
    <div class="version">
        <AppVersion/>
    </div>
</Main>

<style>
    .btn {
        display: flex;
        flex-direction: column;
    }

    .version {
        position: absolute;
        right: 0;
        top: calc(100dvh - 1.2rem);
    }
</style>
