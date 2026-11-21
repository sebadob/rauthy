<script lang="ts">
    import PublicPage from '#lib/PublicPage.svelte';
    import RauthyLogo from '#lib/RauthyLogo.svelte';
    import Template from '#lib/Template.svelte';
    import IconChevronRight from '#icons/IconChevronRight.svelte';
    import IconShieldCheck from '#icons/IconShieldCheck.svelte';
    import IconUser from '#icons/IconUser.svelte';
    import IconUserCircle from '#icons/IconUserCircle.svelte';
    import { useI18n } from '#state/i18n.svelte.ts';
    import { TPL_ADMIN_BUTTON_HIDE, TPL_IS_REG_OPEN } from '#utils/constants.ts';

    let t = useI18n();
    let hideAdminButton = $state(true);
    let isRegOpen = $state(false);
</script>

<svelte:head>
    <title>Rauthy Home</title>
</svelte:head>

<Template id={TPL_ADMIN_BUTTON_HIDE} bind:value={hideAdminButton} />
<Template id={TPL_IS_REG_OPEN} bind:value={isRegOpen} />

<PublicPage>
    <section class="public-card home-card" aria-labelledby="rauthy-heading">
        <header>
            <div aria-hidden="true"><RauthyLogo width="3.5rem" /></div>
            <h1 id="rauthy-heading">Rauthy</h1>
        </header>

        <nav aria-labelledby="rauthy-heading">
            <a class="action account" href="/auth/v1/account" data-sveltekit-reload>
                <IconUserCircle width="1.25rem" />
                <span>{t.index.accountLogin}</span>
                <IconChevronRight width="1rem" />
            </a>
            {#if isRegOpen}
                <a class="action" href="/auth/v1/users/register" data-sveltekit-reload>
                    <IconUser width="1.25rem" />
                    <span>{t.index.register}</span>
                    <IconChevronRight width="1rem" />
                </a>
            {/if}
            {#if !hideAdminButton}
                <a class="action admin" href="/auth/v1/admin" data-sveltekit-reload>
                    <IconShieldCheck width="1.25rem" />
                    <span>{t.index.adminLogin}</span>
                    <IconChevronRight width="1rem" />
                </a>
            {/if}
        </nav>
    </section>
</PublicPage>

<style>
    .home-card {
        --public-card-width: 24rem;
        display: grid;
        gap: 1.5rem;
    }

    header {
        display: grid;
        justify-items: center;
        gap: 0.5rem;
    }

    nav {
        display: grid;
        gap: 0.5rem;
    }

    .action {
        display: grid;
        grid-template-columns: auto minmax(0, 1fr) auto;
        align-items: center;
        gap: 0.75rem;
        min-height: var(--ui-field-height);
        padding: 0.625rem 0.875rem;
        color: var(--ui-text);
        border: 1px solid var(--ui-outline);
        border-radius: var(--border-radius);
        font-size: var(--ui-control-font-size);
        line-height: var(--ui-control-line-height);
        font-weight: 600;
        text-decoration: none;
        transition:
            background-color 120ms,
            border-color 120ms;
    }

    .action:hover {
        background: var(--ui-hover);
    }

    .account {
        background: var(--ui-tonal);
        border-color: hsl(var(--action) / 0.46);
    }

    .account:hover {
        background: hsl(var(--action) / 0.22);
    }

    .admin {
        color: color-mix(in srgb, hsl(var(--accent)) 55%, var(--ui-text));
        background: hsl(var(--accent) / 0.08);
        border-color: hsl(var(--accent) / 0.5);
    }

    .admin:hover {
        background: hsl(var(--accent) / 0.17);
    }
</style>
