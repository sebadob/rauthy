<script>
    import {onMount} from "svelte";
    import {checkAdminAccess, getSessionInfo} from "../../utils/dataFetching.js";
    import {redirectToLogin} from "../../utils/helpers.js";
    import BrowserCheck from "../BrowserCheck.svelte";
    import Loading from "$lib/Loading.svelte";
    import Button from "$lib/Button.svelte";
    import AdminMain from "./AdminMain.svelte";

    /**
     * @typedef {Object} Props
     * @property {string} [selected]
     */

    /** @type {Props} */
    let { selected = $bindable('Users') } = $props();

    let sessionInfo = $state();
    let isAdmin = $state(false);
    let mfaReqErr = $state(false);

    onMount(async () => {
        let res = await getSessionInfo();
        if (res.ok) {
            let body = await res.json();

            let resAdmin = await checkAdminAccess();
            if (resAdmin.ok) {
                sessionInfo = body;
                isAdmin = true;
            } else if (resAdmin.status === 406) {
                mfaReqErr = true;
                sessionInfo = {};
            } else {
                sessionInfo = {};
            }
        } else {
            redirectToLogin();
        }
    });

</script>

<svelte:head>
    <title>Admin</title>
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
            <Button on:click={() => window.location.href = '/auth/v1/account'}>ACCOUNT</Button>
        </div>
    {:else if !isAdmin}
        <div class="noAdmin">
            <div class="text">
                You are not assigned to the <b>rauthy_admin</b> role.<br/>
                You do not have access to the admin panel.
            </div>
            <Button on:click={() => window.location.href = '/auth/v1/'}>GO BACK</Button>
        </div>
    {:else}
        <AdminMain bind:selected/>
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
