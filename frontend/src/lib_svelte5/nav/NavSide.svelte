<script lang="ts">
    import ThemeSwitch from "$lib5/ThemeSwitch.svelte";
    import Button from "$lib5/Button.svelte";
    import LangSelector from "$lib5/LangSelector.svelte";
    import {page} from "$app/state";
    import {useI18n} from "$state/i18n.svelte.ts";
    import {useSession} from "$state/session.svelte.ts";
    import AppVersion from "../../components/AppVersion.svelte";
    import RauthyLogo from "$lib/icons/RauthyLogo.svelte";
    import NavLink from "$lib5/nav/NavLink.svelte";
    import IconBurger from "$icons/IconBurger.svelte";
    import IconArrowPathSquare from "$icons/IconArrowPathSquare.svelte";
    import IconUserGroup from "$icons/IconUserGroup.svelte";
    import IconDocText from "$icons/IconDocText.svelte";
    import IconOffice from "$icons/IconOffice.svelte";
    import IconShieldCheck from "$icons/IconShieldCheck.svelte";
    import IconUser from "$icons/IconUser.svelte";
    import IconId from "$icons/IconId.svelte";
    import IconBell from "$icons/IconBell.svelte";
    import IconStop from "$icons/IconStop.svelte";
    import IconKey from "$icons/IconKey.svelte";
    import IconCloud from "$icons/IconCloud.svelte";
    import IconWrenchScrew from "$icons/IconWrenchScrew.svelte";
    import IconBookOpen from "$icons/IconBookOpen.svelte";
    import IconCheckBadge from "$icons/IconCheckBadge.svelte";

    let t = useI18n();
    let session = useSession('admin');
    let timeout: undefined | number;

    let optsButtons = $state(false);
    let params = $state('')
    let compact = $state(false);
    let collapsed = $state(true);

    let innerWidth: undefined | number = $state();
    let widthLogo = $derived(compact ? '2rem' : '5.5rem');

    // cannot be derived because of static pre-rendering
    $effect(() => {
        params = page.url.search;
    })

    $effect(() => {
        if (innerWidth && innerWidth < 600 && page.url) {
            collapsed = true;
        }
    })

    $effect(() => {
        collapsed = (innerWidth && innerWidth < 800) || false;
    });

    $effect(() => {
        compact = (innerWidth && innerWidth < 1280) || false;
    });

    // function login() {
    //     fetchGet('/api/session/check');
    // }

    function showOpts() {
        if (timeout) {
            clearTimeout(timeout);
        }
        optsButtons = true
    }

    function hideOpts() {
        setTimeout(() => {
            optsButtons = false;
        }, 1000);
    }

    function toggleCollapsed() {
        collapsed = !collapsed;
    }

    function toggleCompact() {
        compact = !compact;
    }
</script>

<svelte:window bind:innerWidth></svelte:window>

<nav
        id="mainNav"
        aria-hidden={collapsed}
        onmouseenter={showOpts}
        onmouseleave={hideOpts}
        data-compact={compact}
>
    <div>
        <div>
            <div class="relative">
                <div class="absolute flex space-between navmod" aria-hidden={!collapsed && !optsButtons}>
                    <div>
                        <Button ariaControls="mainNav" invisible onclick={toggleCollapsed}>
                            <span class="inner">
                                <IconBurger/>
                            </span>
                        </Button>
                    </div>

                    {#if !collapsed}
                        <div>
                            <Button invisible onclick={toggleCompact}>
                            <span class="inner">
                                <IconArrowPathSquare/>
                            </span>
                            </Button>
                        </div>
                    {/if}
                </div>
            </div>

            {#if !collapsed}
                <div class="logo">
                    <!-- TODO
                    should we maybe fetch it from the background
                    or always keep the Rauthy branding here?
                     -->
                    <RauthyLogo/>
                </div>

                <div class="menu">
                    <NavLink {compact} {params} route="/users">
                        {#snippet icon(width: string)}
                            <IconUser {width}/>
                        {/snippet}
                        Users
                    </NavLink>

                    <NavLink {compact} {params} route="/attributes">
                        {#snippet icon(width: string)}
                            <IconDocText {width}/>
                        {/snippet}
                        Attributes
                    </NavLink>

                    <NavLink {compact} {params} route="/clients">
                        {#snippet icon(width: string)}
                            <IconOffice {width}/>
                        {/snippet}
                        Clients
                    </NavLink>

                    <NavLink {compact} {params} route="/roles">
                        {#snippet icon(width: string)}
                            <IconCheckBadge {width}/>
                        {/snippet}
                        Roles
                    </NavLink>

                    <NavLink {compact} {params} route="/groups">
                        {#snippet icon(width: string)}
                            <IconUserGroup {width}/>
                        {/snippet}
                        Groups
                    </NavLink>

                    <NavLink {compact} {params} route="/scopes">
                        {#snippet icon(width: string)}
                            <IconId {width}/>
                        {/snippet}
                        Scopes
                    </NavLink>

                    <NavLink {compact} {params} route="/sessions">
                        {#snippet icon(width: string)}
                            <IconShieldCheck {width}/>
                        {/snippet}
                        Sessions
                    </NavLink>

                    <NavLink {compact} {params} route="/events">
                        {#snippet icon(width: string)}
                            <IconBell {width}/>
                        {/snippet}
                        Events
                    </NavLink>

                    <NavLink {compact} {params} route="/blacklist">
                        {#snippet icon(width: string)}
                            <IconStop {width} color="currentColor"/>
                        {/snippet}
                        Blacklist
                    </NavLink>

                    <NavLink {compact} {params} route="/api_keys">
                        {#snippet icon(width: string)}
                            <IconKey {width}/>
                        {/snippet}
                        API Keys
                    </NavLink>

                    <NavLink {compact} {params} route="/providers">
                        {#snippet icon(width: string)}
                            <IconCloud {width}/>
                        {/snippet}
                        Providers
                    </NavLink>

                    <NavLink {compact} {params} route="/config">
                        {#snippet icon(width: string)}
                            <IconWrenchScrew {width}/>
                        {/snippet}
                        Config
                    </NavLink>

                    <NavLink {compact} {params} route="/docs">
                        {#snippet icon(width: string)}
                            <IconBookOpen {width}/>
                        {/snippet}
                        Docs
                    </NavLink>
                </div>
            {/if}
        </div>

        {#if !collapsed}
            {#if compact}
                <div class="bottom">
                    <AppVersion/>
                    <span class="themeSwitch">
                    <ThemeSwitch/>
                </span>
                    <LangSelector openTop/>
                </div>
            {:else}
                <div class="bottom">
                    <div class="flex gap-05">
                        <ThemeSwitch/>
                        <LangSelector openTop/>
                    </div>
                    <div>
                        <AppVersion/>
                    </div>
                </div>
            {/if}
        {/if}
    </div>
</nav>

<style>
    nav {
        width: 12.5rem;
        height: 100dvh;
        background: hsl(var(--bg));
        z-index: 101;
        transition: width 150ms;
    }

    @media (max-width: 800px) {
        nav {
            position: absolute;
            top: 0;
            left: 0;
        }
    }

    nav > div {
        width: 100%;
        height: 100%;
        padding: 1rem 1rem .5rem 1rem;
        display: flex;
        flex-direction: column;
        justify-content: space-between;
        background: hsla(var(--bg-high) / .15);
        transition: all 150ms;
    }

    nav[data-compact="true"] {
        width: 4rem;
    }

    nav[data-compact="true"] > div {
        padding: .5rem .2rem 0 .2rem;
        text-align: center;
        line-height: .8rem;
    }

    nav[aria-hidden="true"] {
        width: 0;
    }

    nav[aria-hidden="true"] > div {
        padding: 0;
    }

    .bottom {
        display: flex;
        justify-content: space-between;
    }

    nav[data-compact="true"] .bottom {
        flex-direction: column;
        align-items: center;
        gap: .2rem;
    }

    nav[data-compact="true"] .themeSwitch {
        position: relative;
        margin: .25rem 0 -.2rem 0;
    }

    .logo {
        display: flex;
        align-items: center;
        justify-content: center;
        margin-bottom: 1rem;
        border-radius: var(--border-radius);
        overflow: clip;
    }

    .menu {
        max-height: 80dvh;
        display: flex;
        flex-direction: column;
        gap: .5rem;
        overflow-y: auto;
    }

    nav[data-compact="true"] .menu {
        max-height: 73dvh;
    }

    .navmod {
        opacity: 0;
        margin: -1rem 0 0 -1rem;
        width: calc(100% + 1.8rem);
        transition: all 150ms ease-in-out;
    }

    nav[aria-hidden="true"] .navmod {
        opacity: 1;
        margin: 0 0 0 .25rem;
    }

    nav[data-compact="true"] .navmod {
        margin: -.2rem 0 0 -.2rem;
        width: calc(100% + .4rem);
    }

    .navmod[aria-hidden="false"], .navmod:hover {
        opacity: 1;
    }

    .navmod .inner {
        color: hsla(var(--text) / .5);
        transition: all 150ms ease-in-out;
    }

    nav[aria-hidden="true"] .navmod .inner {
        color: hsla(var(--text) / .7);
    }

    nav[aria-hidden="true"] .navmod .inner:hover,
    .navmod .inner:hover {
        color: hsl(var(--action));
    }
</style>
