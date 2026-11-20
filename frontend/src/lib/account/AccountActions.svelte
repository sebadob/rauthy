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

<div class="actions" class:compact>
    {#if admin || showAdminLink}
        {@const label = admin ? t.account.account : t.account.navBackToAdmin}
        <Tooltip text={label}>
            <a href={admin ? '/auth/v1/account' : '/auth/v1/admin/users'} aria-label={label}>
                {#if admin}<IconUserCircle width="1.125rem" />{:else}
                    <IconUserGroup width="1.125rem" />
                {/if}
                {#if !compact}{adminLabel ?? label}{/if}
            </a>
        </Tooltip>
    {/if}
    <Tooltip text={t.account.navLogout}>
        <button type="button" aria-label={t.account.navLogout} onclick={redirectToLogout}>
            <IconLogout width="1.125rem" />
            {#if !compact}{t.account.navLogout}{/if}
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
</style>
