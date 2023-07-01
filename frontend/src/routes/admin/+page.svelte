<script>
    import {onMount} from "svelte";
    import {checkAdminAccess, getSessionInfo} from "../../utils/dataFetching.js";
    import Loading from "../../components/Loading.svelte";
    import {redirectToLogin} from "../../utils/helpers.js";
    import AdminMain from "../../components/admin/AdminMain.svelte";
    import Button from "$lib/Button.svelte";
    import BrowserCheck from "../../components/BrowserCheck.svelte";

    let sessionInfo;
    let isAdmin = false;
    let mfaReqErr = false;

    onMount(async () => {
        let res = await getSessionInfo();
        if (res.ok) {
            let body = await res.json();

            let resAdmin = await checkAdminAccess();
            if (resAdmin.ok) {
                sessionInfo = body;
                isAdmin = true;
            } else {
                mfaReqErr = true;
                sessionInfo = {};
            }
        } else {
            redirectToLogin();
        }
    });

</script>

<svelte:head>
    <title>Rauthy Admin</title>
</svelte:head>

<BrowserCheck>
    {#if !sessionInfo && !mfaReqErr}
        <Loading/>
    {:else if mfaReqErr}
        <div class="noAdmin">
            <div class="text">
                A rauthy admin account must have <b>MFA enabled.</b><br>
                Please navigate to your <b>account</b> and activate MFA.<br>
                Afterwards, you need to do a logout and log back in.
            </div>
            <Button on:click={() => window.location.href = '/auth/v1/account.html'}>ACCOUNT</Button>
        </div>
    {:else if !isAdmin}
        <div class="noAdmin">
            <div class="text">
                You are not assigned to the <b>rauthy_admin</b> role and have no access to the admin panel.
            </div>
            <Button on:click={() => window.location.href = '/auth/v1/'}>GO BACK</Button>
        </div>
    {:else}
        <AdminMain bind:sessionInfo/>
    {/if}
</BrowserCheck>

<style>
    .noAdmin {
        display: flex;
        flex-direction: column;
        justify-content: center;
        align-items: center;
    }

    .text {
        margin: 10px;
    }
</style>
