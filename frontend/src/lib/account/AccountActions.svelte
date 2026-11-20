<script lang="ts">
    import IconUserCircle from '#icons/IconUserCircle.svelte';
    import IconUserGroup from '#icons/IconUserGroup.svelte';
    import IconLogout from '#icons/IconLogout.svelte';
    import Tooltip from '#lib/Tooltip.svelte';
    import { useI18n } from '#state/i18n.svelte.ts';
    import { redirectToLogout } from '#utils/helpers.ts';

    let {
        compact = false,
        admin = false,
        adminLabel,
        showAdminLink = false,
    }: {
        compact?: boolean;
        admin?: boolean;
        adminLabel?: string;
        showAdminLink?: boolean;
    } = $props();

    const t = useI18n();
</script>

<div class="actions" class:admin class:compact>
    {#if admin || showAdminLink}
        {@const label = admin ? t.account.account : t.account.navBackToAdmin}
        <Tooltip text={label}>
            <a
                href={admin ? '/auth/v1/account' : '/auth/v1/admin/users'}
                aria-label={label}
                data-sveltekit-reload={admin ? '' : undefined}
            >
                {#if admin}<IconUserCircle width="1.125rem" />{:else}
                    <IconUserGroup width="1.125rem" />
                {/if}
                {#if !compact}<span class="action-label">{adminLabel ?? label}</span>{/if}
            </a>
        </Tooltip>
    {/if}
    <Tooltip text={t.account.navLogout}>
        <button type="button" aria-label={t.account.navLogout} onclick={redirectToLogout}>
            <IconLogout width="1.125rem" />
            {#if !compact}<span class="action-label">{t.account.navLogout}</span>{/if}
        </button>
    </Tooltip>
</div>

<style>
    .actions {
        display: flex;
        align-items: center;
        flex-wrap: wrap;
        gap: 0.25rem;
    }

    a,
    button {
        display: flex;
        align-items: center;
        gap: 0.375rem;
        min-height: 2rem;
        padding-inline: 0.375rem;
        border: 0;
        border-radius: var(--border-radius);
        background: transparent;
        color: var(--ui-muted);
        text-align: start;
        text-decoration: none;
        font: inherit;
        font-size: 0.8125rem;
        cursor: pointer;
    }

    a:hover,
    button:hover {
        color: var(--ui-text);
        background: var(--ui-hover);
    }

    button:focus-visible {
        outline: 2px solid var(--ui-primary);
    }

    .compact {
        justify-content: center;
    }

    .admin {
        display: grid;
        grid-template-columns: repeat(2, minmax(0, 1fr));
        gap: 0.125rem;
        align-items: stretch;
    }

    .admin a,
    .admin button {
        width: 100%;
        height: 100%;
        min-width: 0;
        min-height: var(--footer-control-height, 2rem);
        justify-content: center;
        text-align: center;
        gap: 0.25rem;
        padding-inline: 0.125rem;
    }

    .admin :global(svg) {
        flex-shrink: 0;
    }

    .admin .action-label {
        min-width: 0;
        overflow: hidden;
        text-overflow: ellipsis;
        white-space: nowrap;
    }
</style>
