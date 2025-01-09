<script>
    import {onMount} from "svelte";
    import Button from "$lib/Button.svelte";
    import WithI18n from "$lib/WithI18n.svelte";
    import LangSelector from "$lib/LangSelector.svelte";
    import AppVersion from "../../components/AppVersion.svelte";
    import Main from "$lib5/Main.svelte";
    import ContentCenter from "$lib5/ContentCenter.svelte";
    import {useIsDev} from "$state/is_dev.svelte";

    const btnWidth = "9rem";

    let t = $state();
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

<WithI18n bind:t content="index">
    <Main>
        <ContentCenter>
            <div class="btn">
                {#if renderReg}
                    <Button on:click={redirectToReg} width={btnWidth}>{t.register.toUpperCase()}</Button>
                {/if}
                <Button on:click={redirectToAccount} width={btnWidth}>{t.accountLogin.toUpperCase()}</Button>
                <Button on:click={redirectToAdmin} width={btnWidth}>{t.adminLogin.toUpperCase()}</Button>
            </div>
        </ContentCenter>

        <LangSelector absolute/>
        <div class="version">
            <AppVersion/>
        </div>
    </Main>
</WithI18n>

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
