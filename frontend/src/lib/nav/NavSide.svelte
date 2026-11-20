<script lang="ts">
    import Button from '#lib/button/Button.svelte';
    import { useI18n } from '#state/i18n.svelte.ts';
    import ThemeSwitch from '#lib/ThemeSwitch.svelte';
    import LangSelector from '#lib/LangSelector.svelte';
    import { page } from '$app/state';
    import AppVersion from '../AppVersion.svelte';
    import Popover from '#lib/Popover.svelte';
    import IconInfo from '#icons/IconInfo.svelte';
    import RauthyLogo from '#lib/RauthyLogo.svelte';
    import NavLink from '#lib/nav/NavLink.svelte';
    import AccountActions from '#lib/account/AccountActions.svelte';
    import IconUserGroup from '#icons/IconUserGroup.svelte';
    import IconDocText from '#icons/IconDocText.svelte';
    import IconOffice from '#icons/IconOffice.svelte';
    import IconShieldCheck from '#icons/IconShieldCheck.svelte';
    import IconUser from '#icons/IconUser.svelte';
    import IconId from '#icons/IconId.svelte';
    import IconBell from '#icons/IconBell.svelte';
    import IconStop from '#icons/IconStop.svelte';
    import IconKey from '#icons/IconKey.svelte';
    import IconCloud from '#icons/IconCloud.svelte';
    import IconWrenchScrew from '#icons/IconWrenchScrew.svelte';
    import IconBookOpen from '#icons/IconBookOpen.svelte';
    import IconCheckBadge from '#icons/IconCheckBadge.svelte';
    import { useI18nAdmin } from '#state/i18n_admin.svelte.ts';
    import IconCommandLine from '#icons/IconCommandLine.svelte';
    import IconCircleStack from '#icons/IconCircleStack.svelte';
    import { useSession } from '#state/session.svelte.ts';

    let ta = useI18nAdmin();
    let t = useI18n();

    let session = useSession('admin');
    // Delegated group admins only get the user-centric, read-only sections.
    // Everything that manages clients, roles, groups, scopes, system config, etc. stays
    // full-admin only. The backend enforces this regardless; here we just hide it. The
    // principal acts as a group admin when it reached the admin UI without the full role.
    let isGroupAdmin = $derived(!session.isAdmin());

    let {
        collapsed,
        compact,
        closeDrawer,
        drawerClose = $bindable(),
    }: {
        collapsed: boolean;
        compact: boolean;
        closeDrawer: () => void;
        drawerClose?: HTMLButtonElement;
    } = $props();

    let params = $derived(page.url.search);
</script>

<nav id="mainNav" data-collapsed={collapsed} data-compact={compact}>
    <div class="nav-container">
        <div class="scrollable-content">
            {#if !collapsed}
                <div class="logo">
                    <RauthyLogo width="2rem" />
                    {#if !compact}<strong class="brand-name"
                            >Rauthy<span>{ta.common.administration}</span></strong
                        >{/if}
                    <div class="admin-drawer-close">
                        <Button
                            ariaLabel={t.common.close}
                            invisible
                            onclick={closeDrawer}
                            bind:ref={drawerClose}
                            ><IconStop color="currentColor" width="20px" /></Button
                        >
                    </div>
                </div>

                <div class="menu">
                    <NavLink {compact} {params} route="/users">
                        {#snippet icon(width: string)}
                            <IconUser {width} />
                        {/snippet}
                        {ta.nav.users}
                    </NavLink>

                    {#if !isGroupAdmin}
                        <NavLink {compact} {params} route="/clients">
                            {#snippet icon(width: string)}
                                <IconOffice {width} />
                            {/snippet}
                            {ta.nav.clients}
                        </NavLink>

                        <NavLink {compact} {params} route="/roles">
                            {#snippet icon(width: string)}
                                <IconCheckBadge {width} />
                            {/snippet}
                            {ta.nav.roles}
                        </NavLink>

                        <NavLink {compact} {params} route="/groups">
                            {#snippet icon(width: string)}
                                <IconUserGroup {width} />
                            {/snippet}
                            {ta.nav.groups}
                        </NavLink>

                        <NavLink {compact} {params} route="/attributes">
                            {#snippet icon(width: string)}
                                <IconDocText {width} />
                            {/snippet}
                            {ta.nav.attributes}
                        </NavLink>

                        <NavLink {compact} {params} route="/scopes">
                            {#snippet icon(width: string)}
                                <IconId {width} />
                            {/snippet}
                            {ta.nav.scopes}
                        </NavLink>

                        <NavLink {compact} {params} route="/pam">
                            {#snippet icon(width: string)}
                                <IconCommandLine {width} />
                            {/snippet}
                            PAM
                        </NavLink>
                    {/if}

                    <NavLink {compact} {params} route="/sessions">
                        {#snippet icon(width: string)}
                            <IconShieldCheck {width} />
                        {/snippet}
                        {ta.nav.sessions}
                    </NavLink>

                    <NavLink {compact} {params} route="/events">
                        {#snippet icon(width: string)}
                            <IconBell {width} />
                        {/snippet}
                        {ta.nav.events}
                    </NavLink>

                    <NavLink {compact} {params} route="/blacklist">
                        {#snippet icon(width: string)}
                            <IconStop {width} color="currentColor" />
                        {/snippet}
                        {ta.nav.blacklist}
                    </NavLink>

                    {#if !isGroupAdmin}
                        <NavLink {compact} {params} route="/api_keys">
                            {#snippet icon(width: string)}
                                <IconKey {width} />
                            {/snippet}
                            {ta.nav.apiKeys}
                        </NavLink>

                        <NavLink {compact} {params} route="/providers">
                            {#snippet icon(width: string)}
                                <IconCloud {width} />
                            {/snippet}
                            {ta.nav.providers}
                        </NavLink>

                        <NavLink {compact} {params} route="/kv">
                            {#snippet icon(width: string)}
                                <IconCircleStack {width} />
                            {/snippet}
                            {ta.nav.kvStore}
                        </NavLink>

                        <NavLink
                            {compact}
                            {params}
                            route="/config/policy"
                            highlightIncludes="/config/"
                        >
                            {#snippet icon(width: string)}
                                <IconWrenchScrew {width} />
                            {/snippet}
                            {ta.nav.config}
                        </NavLink>
                    {/if}

                    <NavLink {compact} {params} route="/docs">
                        {#snippet icon(width: string)}
                            <IconBookOpen {width} />
                        {/snippet}
                        {ta.nav.docs}
                    </NavLink>
                </div>
            {/if}
        </div>

        {#if !collapsed}
            <div class="bottom">
                <AccountActions admin adminLabel={ta.common.account} {compact} />
                <div class="admin-preference-controls">
                    <ThemeSwitch /><LangSelector openTop borderless uppercase />
                    <Popover ariaLabel={ta.common.about} btnInvisible anchored lazy>
                        {#snippet button()}<IconInfo width="1.125rem" />{/snippet}
                        <div class="about">
                            <strong>{ta.common.about}</strong>
                            <AppVersion />
                        </div>
                    </Popover>
                </div>
            </div>
        {/if}
    </div>
</nav>

<style>
    nav {
        width: 14rem;
        height: 100dvh;
        background: hsl(var(--bg));
        z-index: 101;
        transition: width 150ms;
        overflow: hidden;
    }

    @media (width < 800px) {
        nav {
            position: absolute;
            top: 0;
            left: 0;
        }
    }

    .admin-drawer-close {
        display: none;
    }

    .nav-container {
        width: 100%;
        height: 100%;
        padding: 1rem 1rem 0.5rem 1rem;
        display: flex;
        flex-direction: column;
        justify-content: space-between;
        background: hsla(var(--bg-high) / 0.25);
        transition: all 150ms;
    }

    .scrollable-content {
        flex: 1;
        overflow-y: auto;
        overflow-x: hidden;
        min-height: 0;
        margin-inline: calc(-1 * var(--nav-inset));
        padding-inline: var(--nav-inset);
        scrollbar-gutter: stable both-edges;
    }

    nav[data-collapsed='true'] .scrollable-content,
    nav[data-collapsed='true'] .bottom {
        display: none;
    }

    nav[data-compact='true'] {
        width: 4rem;
    }

    nav[data-compact='true'] .nav-container {
        padding: 0.5rem 0.2rem 0 0.2rem;
        text-align: center;
        line-height: 0.8rem;
    }

    nav[data-collapsed='true'] {
        width: 0;
        overflow: visible;
    }

    nav[data-collapsed='true'] .nav-container {
        padding: 0;
    }

    .bottom {
        display: flex;
        flex-direction: column;
        justify-content: space-between;
        line-height: 1.5rem;
        margin-top: 0.4rem;
        margin-bottom: 0.4rem;
    }

    .logo {
        display: flex;
        align-items: center;
        justify-content: center;
        border-radius: var(--border-radius);
        overflow: clip;
    }

    .menu {
        max-height: 85dvh;
        display: flex;
        flex-direction: column;
        gap: 0.5rem;
        overflow-y: auto;
    }

    #mainNav {
        width: 12.25rem;
        flex-shrink: 0;
        border: 0;
        background: transparent;
    }
    #mainNav .nav-container {
        --nav-inset: 12px;
        padding: 0 var(--nav-inset) 8px;
        background: transparent;
    }
    #mainNav .logo {
        height: var(--admin-header-height);
        flex-shrink: 0;
        justify-content: center;
        gap: 12px;
        padding: 0 14px;
    }
    .brand-name {
        font-size: 20px;
        font-weight: 650;
        letter-spacing: -0.04em;
        color: var(--ui-text);
        line-height: 24px;
    }
    .brand-name :global(span) {
        display: block;
        font-size: 11px;
        font-weight: 400;
        letter-spacing: 0.02em;
        color: var(--ui-muted);
        line-height: 18px;
    }
    #mainNav .menu {
        gap: 2px;
        padding: 0 0 0.5rem;
        max-height: none;
        overflow: visible;
    }
    #mainNav .menu > :global(a) {
        display: flex;
        align-items: center;
        min-height: 34px;
        padding: 0 12px;
        border: 0;
        border-radius: var(--border-radius);
        color: var(--ui-muted);
        text-decoration: none;
        font-size: 13px;
        font-weight: 500;
    }
    #mainNav .menu > :global(a:hover) {
        background: var(--ui-hover);
    }
    #mainNav .menu > :global(a[aria-current='page']) {
        background: var(--ui-tonal);
        color: var(--ui-primary);
        font-weight: 650;
    }
    #mainNav :global(.wide) {
        display: flex;
        align-items: center;
        gap: 14px;
    }
    #mainNav :global(.iconWide) {
        display: flex;
        align-items: center;
        transform: none;
    }
    #mainNav .menu :global(svg) {
        width: 20px;
        height: 20px;
        display: block;
    }
    #mainNav .menu > :global(a[href*='/sessions']),
    #mainNav .menu > :global(a[href*='/api_keys']),
    #mainNav .menu > :global(a[href*='/config/']) {
        margin-top: 6px;
    }
    #mainNav .bottom {
        border-top: 1px solid var(--ui-outline);
        margin: 4px 0 0;
        padding-top: 4px;
        gap: 0;
    }
    #mainNav .bottom > :global(.flex) {
        gap: 0;
        justify-content: space-between;
    }
    .about {
        display: grid;
        gap: 0.375rem;
        min-width: 10rem;
        padding: 0.75rem;
        color: var(--ui-text);
    }

    .about :global(.ver) {
        margin: 0;
        white-space: nowrap;
    }

    #mainNav[data-compact='true'] {
        width: 84px;
    }
    #mainNav[data-compact='true'] .nav-container {
        --nav-inset: 8px;
    }
    #mainNav[data-compact='true'] .logo {
        justify-content: center;
        padding: 0;
    }
    #mainNav[data-compact='true'] .menu > :global(a) {
        min-height: 39px;
        padding: 4px 0;
        justify-content: center;
        border-radius: var(--border-radius);
    }
    #mainNav[data-compact='true'] :global(.compact) {
        font-size: 10px;
        line-height: 13px;
        gap: 2px;
    }
    #mainNav[data-compact='true'] :global(.compact) > :global(span) {
        color: inherit;
    }
    #mainNav[data-collapsed='true'] {
        width: 0;
    }
    @media (width < 800px) {
        #mainNav {
            z-index: 110;
            background: var(--ui-surface);
        }

        #mainNav .logo {
            justify-content: flex-start;
            gap: 6px;
            padding-inline: 4px 38px;
        }

        .brand-name {
            min-width: 0;
        }

        .brand-name :global(span) {
            overflow: hidden;
            text-overflow: ellipsis;
            white-space: nowrap;
        }

        .admin-drawer-close {
            display: block;
        }
    }
    .admin-preference-controls {
        display: flex;
        align-items: center;
        justify-content: flex-start;
    }

    #mainNav[data-compact='true'] .admin-preference-controls {
        justify-content: center;
        margin-inline: -6px;
        --ui-button-padding: 0;
        --ui-option-button-border: 0;
        --ui-option-padding: 0;
    }

    #mainNav[data-compact='true'] .admin-preference-controls :global(> div > .admin-control),
    #mainNav[data-compact='true'] .admin-preference-controls :global(> div > div > .admin-control) {
        min-width: 24px;
    }

    #mainNav[data-compact='true'] .admin-preference-controls :global(.admin-select-trigger) {
        gap: 2px;
        border: 0;
    }

    @media (height < 850px) {
        #mainNav[data-compact='true'] .menu > :global(a) {
            min-height: 35px;
            padding-block: 2px;
        }
    }

    #mainNav .logo {
        position: relative;
    }
    .admin-drawer-close {
        position: absolute;
        right: 0;
        top: 50%;
        transform: translateY(-50%);
    }
</style>
