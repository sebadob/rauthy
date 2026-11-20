<script lang="ts">
    import { setContext, tick, type Snippet } from 'svelte';
    import { afterNavigate } from '$app/navigation';
    import { page } from '$app/state';
    import { MediaQuery } from 'svelte/reactivity';
    import Main from '#lib/Main.svelte';
    import IconBell from '#icons/IconBell.svelte';
    import Button from '#lib/button/Button.svelte';
    import NavSide from '#lib/nav/NavSide.svelte';
    import Events from '#lib/admin/events/Events.svelte';
    import { useI18n } from '#state/i18n.svelte.ts';
    import { useI18nAdmin } from '#state/i18n_admin.svelte.ts';

    let { children }: { children: Snippet } = $props();

    setContext('rauthy-ui', true);

    const ta = useI18nAdmin();
    const t = useI18n();
    const mobile = new MediaQuery('(width < 800px)', false);
    const compactByDefault = new MediaQuery('(width < 1280px)', false);
    const eventsAvailable = new MediaQuery('(width >= 1024px)', false);
    const eventsHaveRoom = new MediaQuery('(width >= 1200px)', false);

    let drawerOpen = $state(false);
    let compactPreference: boolean | undefined = $state();
    let sidebarToggle: HTMLButtonElement | undefined = $state();
    let drawerClose: HTMLButtonElement | undefined = $state();
    let compact = $derived(!mobile.current && (compactPreference ?? compactByDefault.current));

    // At medium widths the panel starts closed. An explicit open lasts until the viewport
    // crosses another breakpoint. On wider screens restore the saved preference.
    let preferEvents = $state(false);
    let mediumEvents = $state(false);
    let eventsOpen = $derived(
        eventsAvailable.current && (eventsHaveRoom.current ? preferEvents : mediumEvents),
    );

    $effect(() => {
        try {
            preferEvents = localStorage.getItem('adminEventsPanel') === 'true';
        } catch {
            // The panel starts closed when storage is unavailable.
        }
    });

    $effect(() => {
        if (eventsHaveRoom.current || !eventsAvailable.current) {
            mediumEvents = false;
        }
    });

    function toggleEvents() {
        preferEvents = !eventsOpen;
        if (!eventsHaveRoom.current) {
            mediumEvents = preferEvents;
        }
        try {
            localStorage.setItem('adminEventsPanel', String(preferEvents));
        } catch {
            // The current view still remembers the choice.
        }
    }

    $effect(() => {
        if (!mobile.current) {
            drawerOpen = false;
        }
    });

    function closeDrawer() {
        if (drawerOpen) {
            drawerOpen = false;
            sidebarToggle?.focus();
        }
    }

    afterNavigate(closeDrawer);

    async function toggleNavigation() {
        if (!mobile.current) {
            compactPreference = !compact;
            return;
        }
        if (drawerOpen) {
            closeDrawer();
        } else {
            drawerOpen = true;
            await tick();
            drawerClose?.focus();
        }
    }

    function onKeydown(event: KeyboardEvent) {
        if (event.key === 'Escape' && !event.defaultPrevented && drawerOpen) {
            closeDrawer();
        }
    }

    let sectionTitle = $derived.by(() => {
        const section = page.url.pathname.split('/admin/')[1]?.split('/')[0] || 'users';
        const labels: Record<string, string> = {
            users: ta.nav.users,
            clients: ta.nav.clients,
            roles: ta.nav.roles,
            groups: ta.nav.groups,
            attributes: ta.nav.attributes,
            scopes: ta.nav.scopes,
            pam: 'PAM',
            sessions: ta.nav.sessions,
            events: ta.nav.events,
            blacklist: ta.nav.blacklist,
            api_keys: ta.nav.apiKeys,
            providers: ta.nav.providers,
            kv: 'KV Store',
            config: ta.nav.config,
            docs: ta.nav.docs,
        };
        return labels[section] || 'Rauthy';
    });
</script>

<svelte:window onkeydown={onKeydown} />

<div class="admin-shell ui-shell">
    {#if drawerOpen}<button
            class="admin-drawer-backdrop"
            aria-label={t.common.close}
            onclick={closeDrawer}
            tabindex="-1"
        ></button>{/if}
    <NavSide {compact} collapsed={mobile.current && !drawerOpen} {closeDrawer} bind:drawerClose />
    <Main>
        <header class="admin-header">
            <div class="admin-location">
                <Button
                    bind:ref={sidebarToggle}
                    ariaLabel={mobile.current
                        ? ta.common.administration
                        : ta.common.compactNavigation}
                    ariaControls="mainNav"
                    ariaExpanded={mobile.current ? drawerOpen : !compact}
                    invisible
                    onclick={toggleNavigation}
                >
                    <svg
                        width="20"
                        height="20"
                        viewBox="0 0 24 24"
                        fill="none"
                        stroke="currentColor"
                        stroke-width="1.7"
                        ><rect x="3" y="4" width="18" height="16" rx="3" /><path d="M9 4v16" /></svg
                    >
                </Button>
                <span>Rauthy</span><span aria-hidden="true">/</span>
                <h1>{sectionTitle}</h1>
            </div>
            {#if eventsAvailable.current}
                <button
                    class="admin-events-toggle"
                    aria-expanded={eventsOpen}
                    aria-controls="admin-events-panel"
                    onclick={toggleEvents}
                >
                    <IconBell width="1rem" />{ta.nav.events}
                </button>
            {/if}
        </header>
        <div class="admin-workspace" inert={drawerOpen}>
            <div class="content">
                {@render children()}
            </div>
            <div id="admin-events-panel" class="events" hidden={!eventsOpen}>
                {#if eventsOpen}<Events />{/if}
            </div>
        </div>
    </Main>
</div>

<style>
    .content {
        flex: 1;
        display: flex;
        position: relative;
    }
    .events {
        min-width: 0;
        flex-shrink: 0;
    }

    .admin-shell {
        display: flex;
        width: 100%;
        height: 100dvh;
        font-family: -apple-system, BlinkMacSystemFont, 'Segoe UI', Roboto, sans-serif;
        font-size: 0.875rem;
        line-height: 1.5;
        color: var(--ui-muted);
        background: var(--ui-container);
    }
    .admin-shell > :global(main) {
        flex-direction: column;
        min-width: 0;
        overflow: hidden;
    }
    .admin-header {
        display: flex;
        align-items: center;
        justify-content: space-between;
        height: 80px;
        min-height: 80px;
        padding: 0 28px 0 12px;
    }
    .admin-location {
        display: flex;
        align-items: center;
        gap: 12px;
    }
    .admin-location > span {
        font-size: 12px;
        color: var(--ui-muted);
    }
    .admin-header h1 {
        font-size: 15px;
        font-weight: 600;
        margin: 0;
        color: var(--ui-text);
    }
    .admin-events-toggle {
        display: inline-flex;
        align-items: center;
        justify-content: center;
        gap: 8px;
        height: 36px;
        padding: 0 14px;
        border: 1px solid var(--ui-outline);
        border-radius: var(--border-radius);
        background: transparent;
        color: var(--ui-muted);
        font: inherit;
        font-size: 12px;
        cursor: pointer;
    }
    .admin-events-toggle:hover,
    .admin-events-toggle[aria-expanded='true'] {
        background: var(--ui-tonal);
        color: var(--ui-primary);
    }
    .admin-workspace {
        display: flex;
        flex: 1;
        min-height: 0;
        min-width: 0;
        margin: 0 var(--ui-space) var(--ui-space) 0;
        border-radius: var(--border-radius);
        overflow: hidden;
        background: var(--ui-surface);
    }
    .admin-workspace > .content {
        min-width: 0;
    }
    @media (width < 800px) {
        .admin-header {
            height: 72px;
            min-height: 72px;
        }
        .admin-location > span {
            display: none;
        }
        .admin-workspace {
            margin: 0;
            border-radius: var(--border-radius) var(--border-radius) 0 0;
        }
    }
    .admin-location > :global(.admin-control) {
        margin-right: 4px;
    }
    .admin-drawer-backdrop {
        position: fixed;
        inset: 0;
        z-index: 100;
        background: #0006;
        border: 0;
        padding: 0;
        width: 100%;
        height: 100%;
    }
</style>
