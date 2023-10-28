<script>
    import {onMount} from "svelte";
    import Button from "$lib/Button.svelte";
    import BrowserCheck from "../../components/BrowserCheck.svelte";
    import WithI18n from "$lib/WithI18n.svelte";
    import LangSelector from "$lib/LangSelector.svelte";
    import AppVersion from "../../components/AppVersion.svelte";

    const btnWidth = "9rem";

    let t;
    let renderReg = false;

    onMount(() => {
        const data = window.document.getElementsByName('rauthy-data')[0].id
        if ('true' === data) {
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

<BrowserCheck>
    <WithI18n bind:t content="index">
        <div class="btn">
            {#if renderReg}
                <Button on:click={redirectToReg} width={btnWidth}>{t.register.toUpperCase()}</Button>
            {/if}
            <Button on:click={redirectToAccount} width={btnWidth}>{t.accountLogin.toUpperCase()}</Button>
            <Button on:click={redirectToAdmin} width={btnWidth}>{t.adminLogin.toUpperCase()}</Button>
        </div>
        <LangSelector absolute />

        <div class="version">
            <AppVersion />
        </div>
    </WithI18n>
</BrowserCheck>

<style>
    .btn {
        display: flex;
        flex-direction: column;
    }

    .version {
        position: absolute;
        bottom: 0;
        right: 0;
    }
</style>
