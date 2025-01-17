<script lang="ts">
    import {onMount} from "svelte";
    import Button from "$lib/Button.svelte";
    import LangSelector from "$lib5/LangSelector.svelte";
    import AppVersion from "../components/AppVersion.svelte";
    import Main from "$lib5/Main.svelte";
    import ContentCenter from "$lib5/ContentCenter.svelte";
    import {useIsDev} from "$state/is_dev.svelte";
    import {useI18n} from "$state/i18n.svelte";
    import Template from "$lib5/Template.svelte";

    const btnWidth = "9rem";

    let t = useI18n();
    let renderReg = $state(false);

    onMount(() => {
        let isDev = useIsDev();
        const data = window.document.getElementsByName('rauthy-data')[0].id
        if ('true' === data || isDev) {
            renderReg = true;
        }
    });

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

<Main>
    <ContentCenter>
        <div class="btn">
            {#if renderReg}
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
