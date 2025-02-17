<script>
    import {run} from 'svelte/legacy';
    import {redirectToLogout} from "$utils/helpers";
    import Nav from "$lib/nav/Nav.svelte";
    import NavEntry from "$lib/nav/NavEntry.svelte";
    import Users from "./users/Users.svelte";
    import ContentWrapper from "./ContentWrapper.svelte";
    import Roles from "./roles/Roles.svelte";
    import Groups from "./groups/Groups.svelte";
    import Scopes from "$lib5/admin/scopes/Scopes.svelte";
    import Clients from "./clients/Clients.svelte";
    import Sessions from "$lib5/admin/sessions/Sessions.svelte";
    import Attr from "./userAttr/Attr.svelte";
    import Config from "$lib5/admin/config/Config.svelte";
    import IconWrenchScrew from "$lib/icons/IconWrenchScrew.svelte";
    import IconUser from "$lib/icons/IconUser.svelte";
    import IconOffice from "$lib/icons/IconOffice.svelte";
    import IconUsers from "$lib/icons/IconUsers.svelte";
    import IconDocText from "$lib/icons/IconDocText.svelte";
    import IconShieldCheck from "$lib/icons/IconShieldCheck.svelte";
    import IconLogout from "$lib/icons/IconLogout.svelte";
    import IconId from "$lib/icons/IconId.svelte";
    import RauthyLogo from "$lib/icons/RauthyLogo.svelte";
    import Documentation from "$lib5/admin/documentation/Documentation.svelte";
    import IconBookOpen from "$lib/icons/IconBookOpen.svelte";
    import {onMount} from "svelte";
    import Events from "$lib5/admin/events/Events.svelte";
    import IconStop from "$lib/icons/IconStop.svelte";
    import Blacklist from "$lib5/admin/blacklist/Blacklist.svelte";
    import IconKey from "$lib/icons/IconKey.svelte";
    import ApiKeys from "$lib5/admin/api_keys/ApiKeys.svelte";
    import EventsArchive from "$lib5/admin/events/EventsArchive.svelte";
    import IconBellAlert from "$lib/icons/IconBellAlert.svelte";
    import IconCloud from "$lib/icons/IconCloud.svelte";
    import Providers from "$lib5/admin/providers/Providers.svelte";

    /**
     * @typedef {Object} Props
     * @property {string} [selected]
     */

    /** @type {Props} */
    let {selected = $bindable('Users')} = $props();

    let title = $state('Rauthy Admin');
    let isExpanded = $state(true);
    let eventsCollapsed = $state(true);
    let eventsWide = $state(false);
    let innerWidth = $state();

    run(() => {
        if (innerWidth) {
            isExpanded = innerWidth > 1050;
            eventsCollapsed = innerWidth < 1050;
            eventsWide = innerWidth > 1450;
        }
    });

    run(() => {
        if (selected) {
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
                case 'Events': {
                    window.history.pushState('Sessions', '', '/auth/v1/admin/events');
                    title = 'Events';
                    break;
                }
                case 'Blacklist': {
                    window.history.pushState('Blacklist', '', '/auth/v1/admin/blacklist');
                    title = 'Blacklist';
                    break;
                }
                case 'ApiKeys': {
                    window.history.pushState('ApiKeys', '', '/auth/v1/admin/api_keys');
                    title = 'ApiKeys';
                    break;
                }
                case 'Providers': {
                    window.history.pushState('Providers', '', '/auth/v1/admin/providers');
                    title = 'Providers';
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
    });

    onMount(() => {
        window.addEventListener('popstate', setSelected);
        calcWidths(window.innerWidth);

        return () => window.removeEventListener('popstate', setSelected);
    });

    function calcWidths(innerWidth) {
        isExpanded = innerWidth > 1050;
        eventsCollapsed = innerWidth < 1050;
        eventsWide = innerWidth > 1450;
    }

    function setSelected(event) {
        selected = event.state;
    }

</script>

<svelte:window bind:innerWidth/>

<svelte:head>
    <title>{title}</title>
</svelte:head>

<main>
    <Nav bind:selected bind:isExpanded widthExpanded={150} widthCollapsed={70}>
        {#snippet logo()}
            <div>
                <div
                        style:width={isExpanded ? '120px' : '55px'}
                        style:margin-top={isExpanded ? '32px' : '40px'}
                        style:margin-bottom={isExpanded ? '13px' : '22px'}
                >
                    <RauthyLogo/>
                </div>
            </div>
        {/snippet}

        {#snippet entries()}
            <div>
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

                <NavEntry label="Events">
                    <IconBellAlert/>
                </NavEntry>

                <NavEntry label="Blacklist">
                    <IconStop width={24} color="currentColor"/>
                </NavEntry>

                <NavEntry label="ApiKeys">
                    <IconKey/>
                </NavEntry>

                <NavEntry label="Providers">
                    <IconCloud/>
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
        {/snippet}
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
        {:else if 'Events' === selected}
            <ContentWrapper bind:eventsWide bind:eventsCollapsed>
                <EventsArchive/>
            </ContentWrapper>
        {:else if 'Blacklist' === selected}
            <ContentWrapper bind:eventsWide bind:eventsCollapsed>
                <Blacklist/>
            </ContentWrapper>
        {:else if 'ApiKeys' === selected}
            <ContentWrapper bind:eventsWide bind:eventsCollapsed>
                <ApiKeys/>
            </ContentWrapper>
        {:else if 'Providers' === selected}
            <ContentWrapper bind:eventsWide bind:eventsCollapsed>
                <Providers/>
            </ContentWrapper>
        {:else if 'Config' === selected}
            <ContentWrapper bind:eventsWide bind:eventsCollapsed>
                <Config/>
            </ContentWrapper>
        {:else if 'Docs' === selected}
            <ContentWrapper bind:eventsWide bind:eventsCollapsed>
                <Documentation/>
            </ContentWrapper>
        {/if}

        {#if innerWidth !== undefined}
            <Events collapsed={eventsCollapsed} wide={eventsWide}/>
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
