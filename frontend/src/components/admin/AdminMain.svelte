<script>
    import Button from "$lib/Button.svelte";
    import {redirectToLogout} from "../../utils/helpers.js";
    import Nav from "$lib/nav/Nav.svelte";
    import NavEntry from "$lib/nav/NavEntry.svelte";
    import Users from "./users/Users.svelte";
    import ContentWrapper from "./ContentWrapper.svelte";
    import Roles from "./roles/Roles.svelte";
    import Groups from "./groups/Groups.svelte";
    import Scopes from "./scopes/Scopes.svelte";
    import Clients from "./clients/Clients.svelte";
    import Sessions from "./sessions/Sessions.svelte";
    import Attr from "./userAttr/Attr.svelte";
    import Config from "./config/Config.svelte";
    import IconWrenchScrew from "$lib/icons/IconWrenchScrew.svelte";
    import IconUser from "$lib/icons/IconUser.svelte";
    import IconOffice from "$lib/icons/IconOffice.svelte";
    import IconUsers from "$lib/icons/IconUsers.svelte";
    import IconDocText from "$lib/icons/IconDocText.svelte";
    import IconShieldCheck from "$lib/icons/IconShieldCheck.svelte";
    import IconLogout from "$lib/icons/IconLogout.svelte";
    import IconId from "$lib/icons/IconId.svelte";
    import RauthyLogo from "$lib/icons/RauthyLogo.svelte";
    import Documentation from "./documentation/Documentation.svelte";
    import IconBookOpen from "$lib/icons/IconBookOpen.svelte";
    import {onMount} from "svelte";
    import Events from "./events/Events.svelte";

    export let sessionInfo = {};
    export let selected = 'Users';

    let title = 'Rauthy Admin';
    let isExpanded;
    let eventsCollapsed;
    let eventsWide;
    let innerWidth;

    $: if (innerWidth) {
        isExpanded = innerWidth > 1050;
        eventsCollapsed = innerWidth < 1050;
        eventsWide = innerWidth > 1450;
    }

    $: if (selected) {
        switch (selected) {
            case 'Users': {
                window.history.pushState('Users', '', '/auth/v1/admin/users');
                title = 'Users';
                break;
            }
            case 'Attributes': {
                window.history.pushState('Attributes', '', '/auth/v1/admin/attributes');
                title = 'Attributes';
                break;
            }
            case 'Clients': {
                window.history.pushState('Clients', '', '/auth/v1/admin/clients');
                title = 'Clients';
                break;
            }
            case 'Roles': {
                window.history.pushState('Roles', '', '/auth/v1/admin/roles');
                title = 'Roles';
                break;
            }
            case 'Groups': {
                window.history.pushState('Groups', '', '/auth/v1/admin/groups');
                title = 'Groups';
                break;
            }
            case 'Scopes': {
                window.history.pushState('Scopes', '', '/auth/v1/admin/scopes');
                title = 'Scopes';
                break;
            }
            case 'Sessions': {
                window.history.pushState('Sessions', '', '/auth/v1/admin/sessions');
                title = 'Sessions';
                break;
            }
            case 'Config': {
                window.history.pushState('Config', '', '/auth/v1/admin/config');
                title = 'Config';
                break;
            }
            case 'Docs': {
                window.history.pushState('Docs', '', '/auth/v1/admin/docs');
                title = 'Docs';
                break;
            }
            case 'Logout':
                redirectToLogout();
                break;
        }
    }

    onMount(() => {
        window.addEventListener('popstate', (event) => {
            selected = event.state;
        });

        calcWidths(window.innerWidth);

        return () => window.removeEventListener('popstate');
    });

    function calcWidths(innerWidth) {
        isExpanded = innerWidth > 1050;
        eventsCollapsed = innerWidth < 1050;
        eventsWide = innerWidth > 1450;
    }

</script>

<svelte:window bind:innerWidth/>

<svelte:head>
    <title>{title}</title>
</svelte:head>

<main>
    <Nav bind:selected bind:isExpanded widthExpanded={140} widthCollapsed={60}>
        <div slot="logo">
            <div
                    style:width={isExpanded ? '120px' : '55px'}
                    style:margin-top={isExpanded ? '32px' : '40px'}
                    style:margin-bottom={isExpanded ? '13px' : '22px'}
            >
                <RauthyLogo/>
            </div>
        </div>

        <div slot="entries">
            <NavEntry label="Users">
                <IconUser/>
            </NavEntry>

            <NavEntry label="Attributes">
                <IconDocText/>
            </NavEntry>

            <NavEntry label="Clients">
                <IconOffice/>
            </NavEntry>

            <NavEntry label="Roles">
                <IconShieldCheck/>
            </NavEntry>

            <NavEntry label="Groups">
                <IconUsers/>
            </NavEntry>

            <NavEntry label="Scopes">
                <IconId/>
            </NavEntry>

            <NavEntry label="Sessions">
                <IconShieldCheck/>
            </NavEntry>

            <NavEntry label="Config">
                <IconWrenchScrew/>
            </NavEntry>

            <NavEntry label="Docs">
                <IconBookOpen/>
            </NavEntry>

            <NavEntry label="Logout">
                <IconLogout/>
            </NavEntry>
        </div>
    </Nav>

    <div class="inner">
        {#if 'Users' === selected}
            <ContentWrapper bind:eventsWide bind:eventsCollapsed>
                <Users/>
            </ContentWrapper>
        {:else if 'Attributes' === selected}
            <ContentWrapper bind:eventsWide bind:eventsCollapsed>
                <Attr/>
            </ContentWrapper>
        {:else if 'Clients' === selected}
            <ContentWrapper bind:eventsWide bind:eventsCollapsed>
                <Clients/>
            </ContentWrapper>
        {:else if 'Roles' === selected}
            <ContentWrapper bind:eventsWide bind:eventsCollapsed>
                <Roles/>
            </ContentWrapper>
        {:else if 'Groups' === selected}
            <ContentWrapper bind:eventsWide bind:eventsCollapsed>
                <Groups/>
            </ContentWrapper>
        {:else if 'Scopes' === selected}
            <ContentWrapper bind:eventsWide bind:eventsCollapsed>
                <Scopes/>
            </ContentWrapper>
        {:else if 'Sessions' === selected}
            <ContentWrapper bind:eventsWide bind:eventsCollapsed>
                <Sessions/>
            </ContentWrapper>
        {:else if 'Config' === selected}
            <ContentWrapper bind:eventsWide bind:eventsCollapsed>
                <Config/>
            </ContentWrapper>
        {:else if 'Docs' === selected}
            <ContentWrapper bind:eventsWide bind:eventsCollapsed>
                <Documentation/>
            </ContentWrapper>
        {:else}
            <ContentWrapper bind:selected>
                <h1>ADMIN PAGE</h1>
                <div>
                    <h3>Info:</h3>
                    {#if sessionInfo}
                        <div>
                            {sessionInfo.id}
                        </div>
                        <div>
                            {sessionInfo.user_id}
                        </div>
                        <div>
                            {sessionInfo.roles}
                        </div>
                        <div>
                            {sessionInfo.groups}
                        </div>
                        <div>
                            {sessionInfo.exp}
                        </div>
                    {/if}

                    <Button on:click={redirectToLogout}>Logout</Button>
                </div>
            </ContentWrapper>
        {/if}

        {#if innerWidth !== undefined }
            <Events collapsed={eventsCollapsed} wide={eventsWide} />
        {/if}
    </div>
</main>

<style>
    main {
        width: 100vw;
        display: flex;
        align-items: center;
        flex-direction: row;
        overflow-y: auto;
        overflow-x: hidden;
    }

    .inner {
        display: flex;
        width: 100vw;
        justify-content: space-between;
    }
</style>
