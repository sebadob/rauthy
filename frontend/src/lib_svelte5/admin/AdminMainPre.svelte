<script lang="ts">
    import Button from "$lib5/button/Button.svelte";
    import AdminMain from "../../components/admin/AdminMain.svelte";
    import {useSession} from "$state/session.svelte.ts";
    import {fetchGet} from "$api/fetch.ts";
    import Main from "$lib5/Main.svelte";
    import ContentCenter from "$lib5/ContentCenter.svelte";
    import type {ADMIN_PAGES} from "$lib5/nav/props.ts";
    import NavSide from "$lib5/nav/NavSide.svelte";
    import Events from "./events/Events.svelte";
    import ContentWrapper from "../../components/admin/ContentWrapper.svelte";
    import Sessions from "./sessions/Sessions.svelte";
    import Providers from "./providers/Providers.svelte";
    import Config from "./config/Config.svelte";
    import Documentation from "./documentation/Documentation.svelte";
    import Users from "../../components/admin/users/Users.svelte";
    import Scopes from "./scopes/Scopes.svelte";
    import ApiKeys from "./api_keys/ApiKeys.svelte";
    import Roles from "../../components/admin/roles/Roles.svelte";
    import Attr from "../../components/admin/userAttr/Attr.svelte";
    import Groups from "./groups/Groups.svelte";
    import Clients from "../../components/admin/clients/Clients.svelte";
    import EventsArchive from "./events/EventsArchive.svelte";
    import Blacklist from "./blacklist/Blacklist.svelte";

    let {
        selected = $bindable('Users'),
    }: {
        selected: ADMIN_PAGES,
    } = $props();

    let session = useSession('admin');

    let isAdmin = $state(false);
    let needsAdminRole = $state(false);
    let mfaReqErr = $state(false);

    let eventsCollapsed = $state(true);
    let eventsWide = $state(false);

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

<Main>
    <ContentCenter>
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
            <!--            <AdminMain bind:selected/>-->

            <div class="inner">
                <ContentWrapper bind:eventsWide bind:eventsCollapsed>
                    {#if 'Users' === selected}
                        <Users/>
                    {:else if 'Attributes' === selected}
                        <Attr/>
                    {:else if 'Clients' === selected}
                        <Clients/>
                    {:else if 'Roles' === selected}
                        <Roles/>
                    {:else if 'Groups' === selected}
                        <Groups/>
                    {:else if 'Scopes' === selected}
                        <Scopes/>
                    {:else if 'Sessions' === selected}
                        <Sessions/>
                    {:else if 'Events' === selected}
                        <EventsArchive/>
                    {:else if 'Blacklist' === selected}
                        <Blacklist/>
                    {:else if 'ApiKeys' === selected}
                        <ApiKeys/>
                    {:else if 'Providers' === selected}
                        <Providers/>
                    {:else if 'Config' === selected}
                        <Config/>
                    {:else if 'Docs' === selected}
                        <Documentation/>
                    {/if}
                </ContentWrapper>

                {#if innerWidth !== undefined}
                    <Events collapsed={eventsCollapsed} wide={eventsWide}/>
                {/if}
            </div>

        {/if}
    </ContentCenter>
</Main>

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
