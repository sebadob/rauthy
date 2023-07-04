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
    import RauthyLogo from "../icons/RauthyLogo.svelte";
    import Documentation from "./documentation/Documentation.svelte";
    import IconBookOpen from "../icons/IconBookOpen.svelte";

    export let sessionInfo = {};

    let isExpanded;
    let selected = 'Users';
    let innerWidth;

    $: if (innerWidth) {
        isExpanded = innerWidth > 1050;
    }

    $: if (selected === 'Logout') {
        redirectToLogout();
    }
</script>

<svelte:window bind:innerWidth/>

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

    {#if 'Users' === selected}
        <ContentWrapper>
            <Users/>
        </ContentWrapper>
    {:else if 'Attributes' === selected}
        <ContentWrapper>
            <Attr/>
        </ContentWrapper>
    {:else if 'Clients' === selected}
        <ContentWrapper>
            <Clients/>
        </ContentWrapper>
    {:else if 'Roles' === selected}
        <ContentWrapper>
            <Roles/>
        </ContentWrapper>
    {:else if 'Groups' === selected}
        <ContentWrapper>
            <Groups/>
        </ContentWrapper>
    {:else if 'Scopes' === selected}
        <ContentWrapper>
            <Scopes/>
        </ContentWrapper>
    {:else if 'Sessions' === selected}
        <ContentWrapper>
            <Sessions/>
        </ContentWrapper>
    {:else if 'Config' === selected}
        <ContentWrapper>
            <Config/>
        </ContentWrapper>
    {:else if 'Docs' === selected}
        <ContentWrapper>
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
</style>
