<script>
    import {onMount} from "svelte";
    import Button from "$lib/Button.svelte";
    import BrowserCheck from "../../components/BrowserCheck.svelte";

    const btnWidth = "9rem";

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
    <div class="btn">
        {#if renderReg}
            <Button on:click={redirectToReg} width={btnWidth}>REGISTER</Button>
        {/if}
        <Button on:click={redirectToAccount} width={btnWidth}>ACCOUNT LOGIN</Button>
        <Button on:click={redirectToAdmin} width={btnWidth}>ADMIN LOGIN</Button>
    </div>
</BrowserCheck>

<style>
    .btn {
        display: flex;
        flex-direction: column;
    }
</style>
