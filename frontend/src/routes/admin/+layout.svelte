<script lang="ts">
    import type {Snippet} from "svelte";
    import Main from "$lib5/Main.svelte";
    import Button from "$lib5/Button.svelte";
    import NavSide from "$lib5/nav/NavSide.svelte";
    import {useSession} from "$state/session.svelte.ts";
    import {fetchGet} from "$api/fetch.ts";
    import Events from "../../components/admin/events/Events.svelte";

    let {
        children,
    }: {
        children: Snippet,
    } = $props();

    let session = useSession('admin');

    let isAdmin = $state(false);
    let needsAdminRole = $state(false);
    let mfaReqErr = $state(false);

    $effect(() => {
        let s = session.get();
        if (s) {
            let isAdm = !!s?.roles?.includes('rauthy_admin');
            if (isAdm) {
                isAdmin = true;
                // async check for admin access speeds up FCP and is still fast enough for a good UX
                checkAdminAccess();
            } else {
                needsAdminRole = true;
            }
        }
    });

    async function checkAdminAccess() {
        let res = await fetchGet('/auth/v1/auth_check_admin');
        if (res.status === 406) {
            mfaReqErr = true;
        }
    }
</script>

<svelte:head>
    <title>Rauthy Admin</title>
</svelte:head>

{#if mfaReqErr}
    <div class="noAdmin">
        <div class="text">
            A Rauthy admin account must have <b>MFA enabled.</b><br>
            Please navigate to your <b>account</b> and activate MFA.<br>
            Afterwards, you need to do a logout and log back in.
        </div>
        <Button onclick={() => window.location.href = '/auth/v1/account'}>Account</Button>
    </div>
{:else if needsAdminRole}
    <div class="noAdmin">
        <div class="text">
            You are not assigned to the <b>rauthy_admin</b> role.<br/>
            You do not have access to the admin panel.
        </div>
        <Button onclick={() => window.location.href = '/auth/v1/'}>Go Back</Button>
    </div>
{:else if isAdmin}
    <NavSide/>
    <Main>
        <div class="content">
            {@render children()}
        </div>
        <div class="events">
            <!-- TODO make self-contained without state bindings -->
            <Events collapsed={false}/>
        </div>
    </Main>
{/if}

<style>
    .noAdmin {
        display: flex;
        flex-direction: column;
        justify-content: center;
        align-items: center;
    }

    .text {
        margin: .8rem;
    }
</style>
